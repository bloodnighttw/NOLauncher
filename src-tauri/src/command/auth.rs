mod devicecode;

use std::marker::PhantomData;
use reginleif::auth::microsoft::DeviceCode;
use reginleif_utils::expiring_data::ExpiringData;

use tauri::{Manager, Runtime};
use tokio::sync::RwLock;
use crate::utils::module::Module;


pub struct AuthModule<R>(PhantomData<R>) where R:Runtime;

impl <R:Runtime> Module<R> for AuthModule<R>{
    fn build(builder: tauri::Builder<R>) -> tauri::Builder<R> {
        builder
            .setup(|app| {
            // to init the device code data in the app
            let data:Option<ExpiringData<DeviceCode>> = None;
            app.manage(RwLock::from(data));

            Ok(())
        })
            .invoke_handler(
                tauri::generate_handler![
            devicecode::devicecode,
            devicecode::exchange,
            devicecode::refresh
        ])
    }
}