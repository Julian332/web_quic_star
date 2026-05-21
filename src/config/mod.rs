#![allow(unsafe_code)]
#![allow(clippy::expect_used)]
#![allow(clippy::expect_fun_call)]
use serde::Deserialize;
use std::env;
use tracing::error;
use tracing_subscriber::EnvFilter;
use url::Url;

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct Config {
    pub database_url: Url,
    #[cfg(feature = "eth_mode")]
    pub eth_rpc: Url,
    #[cfg(feature = "solana_mode")]
    pub solana_rpc: Url,
    #[cfg(feature = "eth_mode")]
    pub ws_eth_rpc: Url,
    #[cfg(feature = "solana_mode")]
    pub ws_solana_rpc: Url,
    // pub uni_graph_url: Url,
    pub server_port: u64,
    pub file_server_directory: String,

    #[cfg(feature = "eth_mode")]
    #[serde(skip)]
    pub eth_addrs: crate::domain::eth_addr::EthAddrs,
    #[cfg(feature = "solana_mode")]
    #[serde(skip)]
    pub sol_addrs: crate::domain::solana_addr::SolAddrs,
    pub is_dev: bool,
}

pub fn set_env() {
    let dev_config = "dev";
    dotenvy::from_filename_override(".env").expect("no .env file");
    let binding = env::var("ACTIVE_CONFIG");
    let active_config = binding.as_ref().map(|x| x.as_str()).unwrap_or(dev_config);
    if active_config == dev_config {
        unsafe {
            //  single thread in start stage
            env::set_var("IS_DEV", "true")
        }
    } else {
        unsafe {
            //  single thread in start stage
            env::set_var("IS_DEV", "false")
        }
    }
    println!(" profile :{active_config} is active");
    dotenvy::from_filename_override(format!("{active_config}.env"))
        .expect(&format!("no {active_config}.env file"));
}

pub fn set_log() {
    // panic::set_hook(Box::new(|info| {
    //     error!(error = %info, "panic occurred");
    // }));
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(tracing::Level::INFO)
        .with_env_filter(EnvFilter::from_default_env())
        .event_format(
            tracing_subscriber::fmt::format()
                // .with_file(true)
                .with_line_number(true),
        )
        .init();
    aide::generate::on_error(|error| {
        error!("{error}");
    });
}

// #[derive(Default, Deref, From)]
// #[allow(unused)]
// struct TaskLocalFormatter(Format);
//
// impl<S, N> FormatEvent<S, N> for TaskLocalFormatter
// where
//     S: Subscriber + for<'a> LookupSpan<'a>,
//     N: for<'a> FormatFields<'a> + 'static,
// {
//     fn format_event(
//         &self,
//         ctx: &fmt::FmtContext<'_, S, N>,
//         mut writer: fmt::format::Writer<'_>,
//         event: &Event<'_>,
//     ) -> std::fmt::Result {
//         let _ = CURRENT_REQ.try_with(|id| write!(writer, "[req_id:{}] ", id.req_id));
//         self.0.format_event(ctx, writer, event)
//     }
// }
