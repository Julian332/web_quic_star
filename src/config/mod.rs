use std::env;
use std::sync::LazyLock;

pub const FILE_SERVER_DIRECTORY: LazyLock<String> =
    LazyLock::new(|| env::var("FILE_SERVER_DIRECTORY").expect(".env FILE_SERVER_DIRECTORY"));
