mod microsoft;
mod xbox;
mod minecraft;

use reginleif::auth::microsoft::{DeviceCode, MicrosoftAuth};
use reginleif::auth::xbox::{XboxLiveToken, XboxSecurityToken};
use reginleif_utils::expiring_data::ExpiringData;

use tauri::Runtime;
use tokio::sync::{Mutex, RwLock};
use crate::utils::module::BuilderWrapper;

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

pub fn init<R>(wrapper: BuilderWrapper<R>) -> BuilderWrapper<R>
where
    R:Runtime{

    let data:Option<ExpiringData<DeviceCode>> = None;
    let step:NLAuthStep = AuthStep::Exchange.into();

    wrapper
        .manage(RwLock::from(data))
        .manage(step)
        .invoke_handler(
            tauri::generate_handler![
                microsoft::devicecode,
                microsoft::exchange,
                microsoft::refresh,
                xbox::xbox_live,
                xbox::xbox_security,
                minecraft::account
            ]
        )
}