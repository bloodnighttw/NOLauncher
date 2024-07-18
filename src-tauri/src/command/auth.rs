mod devicecode;

use reginleif::auth::microsoft::DeviceCode;
use reginleif_utils::expiring_data::ExpiringData;

use tauri::{Manager, Runtime};
use tokio::sync::RwLock;

pub fn init<R>(builder: tauri::Builder<R>) -> tauri::Builder<R> where R:Runtime{
    builder.setup(|app| {
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
            ]
        )
}
