// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::command::auth::AuthModule;
use log::{LevelFilter, Log, Metadata, Record};

mod utils;
mod constant;
mod command;

struct Logger;
use crate::utils::module::ModuleExtend;

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
        .invoke_handler(tauri::generate_handler![
        ])
        .setup(|app| {
            Ok(())
        })
        .module::<AuthModule<_>>()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
