// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate core;

use crate::command::auth;
use command::{accounts, instances, metadata};
use log::{LevelFilter, Log, Metadata, Record};

use crate::utils::module::ModuleExtend;

mod utils;
mod constant;
mod command;

struct Logger;



impl Log for Logger {
    fn enabled(&self, _meta: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        eprintln!("{}: {}", record.level(), record.args());
    }

    fn flush(&self) {}
}

fn init_log() {
    static LOGGER: Logger = Logger;
    if std::env::var("RUST_Debug").unwrap_or("false".to_string()) == "true" {
        log::set_max_level(LevelFilter::Error);
    } else {
        log::set_max_level(LevelFilter::Info)
    }
    log::set_logger(&LOGGER).unwrap();
}

fn main() {

    let builder = tauri::Builder::default();
    
    #[cfg(not(debug_assertions))]
    init_log();
    
    #[cfg(debug_assertions)]
    let builder = builder.plugin(tauri_plugin_devtools::init());

    builder
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .module(auth::init)
        .module(accounts::init)
        .module(metadata::init)
        .module(instances::init)
        .expand()
        .invoke_handler(tauri::generate_handler![

            /* This is from auth module */
            auth::microsoft::devicecode,
            auth::microsoft::exchange,
            auth::microsoft::refresh,
            auth::xbox::xbox_live,
            auth::xbox::xbox_security,
            auth::minecraft::account,

            /* This is from accounts module */
            accounts::info::accounts_list,
            accounts::info::accounts_now,
            accounts::control::logout,
            accounts::control::switch,

            /* This is from metadata module */
            metadata::packages::pkg_info,
            metadata::packages::pkg_refresh,
            
            /* This is from instances module */
            instances::control::instance_create,
            instances::info::instance_list,
            instances::info::instance_downloads,

        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}