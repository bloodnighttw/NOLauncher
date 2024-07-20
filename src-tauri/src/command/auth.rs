mod microsoft;
mod xbox;
mod minecraft;

use reginleif::auth::microsoft::{DeviceCode, MicrosoftAuth};
use reginleif::auth::xbox::{XboxLiveToken, XboxSecurityToken};
use reginleif_utils::expiring_data::ExpiringData;

use tauri::{Manager, Runtime};
use tokio::sync::{Mutex, RwLock};

type NLDevicecode = RwLock<Option<ExpiringData<DeviceCode>>>;

#[derive(Debug, Clone, PartialEq)]
pub enum AuthStep{
    /// wait for user to auth
    Exchange,
    /// fetching xbox live token
    XboxLive(ExpiringData<MicrosoftAuth>),
    /// fetching xbox security token
    XboxSecurity(ExpiringData<MicrosoftAuth>,XboxLiveToken),
    /// fetching minecraft profile and token
    Minecraft(ExpiringData<MicrosoftAuth>,XboxSecurityToken),
}

pub type NLAuthStep = Mutex<AuthStep>;

pub fn init<R>(builder: tauri::Builder<R>) -> tauri::Builder<R> where R:Runtime{
    builder.setup(|app| {
        // to init the device code data in the app
        let data:Option<ExpiringData<DeviceCode>> = None;
        let step:NLAuthStep = AuthStep::Exchange.into();
        app.manage(RwLock::from(data));
        app.manage(step);
        Ok(())
    })
        .invoke_handler(
            tauri::generate_handler![
                microsoft::devicecode,
                microsoft::exchange,
                microsoft::refresh,
                xbox::xbox_live,
                xbox::xbox_security,
            ]
        )
}