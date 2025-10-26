//! Logging infrastructure for Devora

use std::env;
use log::LevelFilter;

pub fn init() {
    let default_level = if env::var("DEVORA_DEBUG").is_ok() {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    env_logger::Builder::from_default_env()
        .filter_level(default_level)
        .format_timestamp_secs()
        .format_module_path(false)
        .target(env_logger::Target::Stderr)
        .init();
}