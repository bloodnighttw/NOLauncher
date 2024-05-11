// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::{LevelFilter, Log, Metadata, Record};

mod auth;

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
    log::set_max_level(LevelFilter::Error);
    log::set_logger(&LOGGER).unwrap();
}


fn main() {
   
    init_log();
    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![auth::msa_auth_open_browser,auth::msa_auth_init,auth::msa_auth_exchange])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
