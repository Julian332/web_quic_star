use serde::Deserialize;
use tracing::error;
use tracing_subscriber::EnvFilter;
use url::Url;

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct Config {
    pub database_url: Url,
    pub eth_rpc: Url,
    pub solana_rpc: Url,
    pub ws_eth_rpc: Url,
    pub ws_solana_rpc: Url,
    pub uni_graph_url: Url,
    pub server_port: u64,
    pub file_server_directory: String,

    #[cfg(feature = "eth_mode")]
    #[serde(skip)]
    pub eth_addrs: crate::domain::eth_addr::EthAddrs,
    #[cfg(feature = "solana_mode")]
    #[serde(skip)]
    pub sol_addrs: crate::domain::solana_addr::SolAddrs,
}

pub fn set_env() {
    #[cfg(feature = "dev")]
    set_dev_env();

    #[cfg(not(feature = "dev"))]
    set_prod_env();
}

#[allow(clippy::expect_used)]
pub fn set_dev_env() {
    tracing::info!("profile :{} is active", "dev");
    dotenvy::from_filename(".env").expect("no .env file");
}

#[allow(clippy::expect_used)]
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

#[test]
pub fn test() {
    use std::ops::Deref;
    set_log();
    use tracing::error;

    let (err_info, port) = ("No connection", 22);

    error!(err_info);
    error!(target: "app_events", "App Error: {}", err_info);
    error!({ info = err_info }, "error on port: {}", port);
    error!(name: "invalid_input", "Invalid input: {}", err_info);
    println!("{:?}", crate::CONFIG.deref());
}
