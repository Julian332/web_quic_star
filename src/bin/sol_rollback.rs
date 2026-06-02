use futures::StreamExt;
use solana_client::nonblocking::pubsub_client::PubsubClient;
use solana_client::rpc_response::SlotUpdate;
use std::collections::BTreeMap;
use std::sync::LazyLock;
use tokio::join;
use tracing::{debug, info, warn};
use web3_quick::subscribe::subscribe_with_retry;
use web3_quick::{AppRes, CONFIG};

#[tokio::main]
async fn main() {
    // Force CONFIG init so env vars and tracing are set up before subscribing.
    LazyLock::force(&CONFIG);
    let x = join!(
        subscribe_with_retry(watch_sol_rollbacks),
        subscribe_with_retry(watch_sol_rollbacks2)
    );
    println!("finished");
}

/// Continuously listen to Solana slot notifications and print every rollback
/// (fork switch) the validator observes. A rollback is detected when a newly
/// reported slot's `parent` does not match the previously observed head, which
/// means the validator switched to a competing fork and abandoned the slots
/// that descended from the old head.
#[tracing::instrument]
async fn watch_sol_rollbacks() -> AppRes<()> {
    let ws_url = CONFIG.ws_solana_rpc.as_str();
    let client = PubsubClient::new(ws_url).await?;
    let (mut stream, _unsubscribe) = client.slot_subscribe().await?;
    info!(%ws_url, "connecting to Solana pubsub");

    // Chain of recently seen slots: slot -> parent. Bounded so memory stays flat.
    let mut chain: BTreeMap<u64, u64> = BTreeMap::new();
    let mut head: Option<u64> = None;
    const CHAIN_RETENTION: usize = 4096;

    while let Some(slot_info) = stream.next().await {
        // debug!(?slot_info);
        let slot = slot_info.slot;
        let parent = slot_info.parent;

        if let Some(prev_head) = head
            && parent != prev_head
            && slot > prev_head
        {
            // New slot does not extend the head we last saw — a fork was chosen.
            // Walk the old head back until we reach a common ancestor (the new
            // slot's parent or one of its ancestors that we have on file) to
            // enumerate the rolled-back slots.
            let mut rolled_back = Vec::new();
            let mut cursor = Some(prev_head);
            while let Some(s) = cursor {
                if s == parent {
                    break;
                }
                rolled_back.push(s);
                cursor = chain.get(&s).copied();
                if rolled_back.len() > 256 {
                    break;
                }
            }
            warn!(
                new_slot = slot,
                new_parent = parent,
                previous_head = prev_head,
                rolled_back_slots = ?rolled_back,
                "Solana rollback detected"
            );
        }

        chain.insert(slot, parent);
        if chain.len() > CHAIN_RETENTION {
            // Drop oldest entries to bound memory.
            let drop_until = slot.saturating_sub(CHAIN_RETENTION as u64);
            chain.retain(|s, _| *s >= drop_until);
        }
        head = Some(slot);
    }

    Ok(())
}
/// Variant 2: use `slot_updates_subscribe` for finer-grained signals.
///
/// `SlotUpdate` distinguishes the lifecycle stages of every slot the validator
/// sees, which lets us classify rollbacks more precisely than `slot_subscribe`:
///   * `CreatedBank { slot, parent }` — a new bank was forked off `parent`.
///     If `parent` is not the previously seen head, the validator switched
///     forks and every slot between the old head and the common ancestor was
///     rolled back.
///   * `Dead { slot, err }` — the slot was abandoned (e.g. shred timeout,
///     duplicate block). These slots are guaranteed not to appear in the
///     finalized chain, so each one is itself a rollback.
///   * `Root { slot }` — the slot was finalized. We use this to bound memory
///     and to log slots that were created but never reached `Root`.
#[tracing::instrument]
async fn watch_sol_rollbacks2() -> AppRes<()> {
    let ws_url = CONFIG.ws_solana_rpc.as_str();
    let client = PubsubClient::new(ws_url).await?;
    let (mut stream, _unsubscribe) = client.slot_updates_subscribe().await?;
    info!(%ws_url, "connecting to Solana pubsub (slot_updates)");

    // slot -> parent for banks that haven't been rooted yet.
    let mut chain: BTreeMap<u64, u64> = BTreeMap::new();
    let mut last_created_head: Option<u64> = None;

    while let Some(update) = stream.next().await {
        // debug!(?update);
        match update {
            SlotUpdate::CreatedBank { slot, parent, .. } => {
                if let Some(prev) = last_created_head
                    && parent != prev
                    && slot > prev
                {
                    // Walk the old head backward until we hit the common
                    // ancestor (= the new bank's parent) to enumerate which
                    // slots got abandoned by this fork switch.
                    let mut rolled_back = Vec::new();
                    let mut cursor = Some(prev);
                    while let Some(s) = cursor {
                        if s == parent {
                            break;
                        }
                        rolled_back.push(s);
                        cursor = chain.get(&s).copied();
                        if rolled_back.len() > 256 {
                            break;
                        }
                    }
                    warn!(
                        new_slot = slot,
                        new_parent = parent,
                        previous_head = prev,
                        rolled_back_slots = ?rolled_back,
                        "fork switch detected (CreatedBank with unexpected parent)"
                    );
                }
                chain.insert(slot, parent);
                last_created_head = Some(slot);
            }
            SlotUpdate::Dead { slot, err, .. } => {
                warn!(slot, %err, "dead slot (abandoned, will not be rooted)");
            }
            SlotUpdate::Root { slot, .. } => {
                // Anything below `slot` is finalized; drop it from the chain
                // to keep memory bounded.
                chain.retain(|s, _| *s > slot);
                debug!(slot, "root advanced");
            }
            SlotUpdate::OptimisticConfirmation { slot, .. } => {
                debug!(slot, "optimistic confirmation");
            }
            SlotUpdate::FirstShredReceived { .. }
            | SlotUpdate::Completed { .. }
            | SlotUpdate::Frozen { .. } => {}
        }
    }

    Ok(())
}
