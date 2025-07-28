use serde::Deserialize;
use std::env;
use std::sync::LazyLock;
use tracing::error;
use tracing_subscriber::EnvFilter;
use url::Url;

pub const FILE_SERVER_DIRECTORY: LazyLock<String> =
    LazyLock::new(|| env::var("FILE_SERVER_DIRECTORY").expect(".env FILE_SERVER_DIRECTORY"));
#[allow(unused)]
#[derive(Deserialize)]
pub struct Config {
    database_url: Url,
    eth_rpc: Url,
    solana_rpc: Url,
    ws_eth_rpc: Url,
    ws_solana_rpc: Url,
    uni_graph_url: Url,
    server_port: u64,
    file_server_directory: String,
    
    #[cfg(feature = "eth_mode")]
    #[serde(skip)]
    eth_addrs: crate::domain::eth_addr::EthAddrs,
    #[cfg(feature = "solana_mode")]
    #[serde(skip)]
    sol_addrs: crate::domain::solana_addr::SolAddrs,
}

pub fn set_env() {
    #[cfg(feature = "dev")]
    set_dev_env();

    #[cfg(not(feature = "dev"))]
    set_prod_env();
}

pub fn set_dev_env() {
    tracing::info!("profile :{} is active", "dev");
    dotenvy::from_filename(".env").expect("no .env file");
}

pub fn set_prod_env() {
    tracing::info!("profile :{} is active", "release");
    dotenvy::from_filename("env_prod.env").expect("no env_prod.env file");
}

pub fn set_log() {
    // panic::set_hook(Box::new(|info| {
    //     error!(error = %info, "panic occurred");
    // }));
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(EnvFilter::from_default_env())
        .with_max_level(tracing::Level::INFO)
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

#[test]
pub fn test() {
    set_log();
    use tracing::error;

    let (err_info, port) = ("No connection", 22);

    error!(err_info);
    error!(target: "app_events", "App Error: {}", err_info);
    error!({ info = err_info }, "error on port: {}", port);
    error!(name: "invalid_input", "Invalid input: {}", err_info);
    panic!("asda")
}
