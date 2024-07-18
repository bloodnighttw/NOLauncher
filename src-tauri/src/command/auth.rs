mod devicecode;

use reginleif::auth::microsoft::DeviceCode;
use reginleif_utils::expiring_data::ExpiringData;

use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime};
use tokio::sync::RwLock;


/// we make each module as an inner plugin in command
pub fn init<R: Runtime>() -> TauriPlugin<R, Option<()>> {
    // Make the plugin config optional
    // by using `Builder::<R, Option<Config>>` instead
    
    Builder::<R, Option<()>>::new("auth")
        .setup(|app, _api| {

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
        .build()
}
