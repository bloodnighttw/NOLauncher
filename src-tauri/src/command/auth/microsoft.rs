use std::time::Duration;
use anyhow::anyhow;
use reginleif::auth::microsoft::{DeviceCode, MicrosoftAuthError};
use reginleif_utils::expiring_data::ExpiringData;
use reqwest::Client;
use serde::{Serialize};
use tauri::{State};
use crate::command::auth::{AuthStep, NLAuthStep, NLDevicecode};
use crate::constant::token::MICROSOFT_CLIENT_ID;
use crate::utils::result::CommandResult;

#[derive(Debug,Clone,Serialize)]
pub struct DevicecodeInfo{
    url:String,
    code:String,
    expiring_in:Duration
}

impl From<ExpiringData<DeviceCode>> for DevicecodeInfo{
    fn from(value: ExpiringData<DeviceCode>) -> Self {
        let left = value.expire_in();
        Self{
            url:value.data.verification_uri,
            code:value.data.user_code,
            expiring_in: left
        }
    }
}

#[tauri::command]
pub async fn devicecode(
    devicecode: State<'_, NLDevicecode>,
    step:State<'_,NLAuthStep>
) -> CommandResult<DevicecodeInfo>{
    
    let _lock = step.lock().await;

    // http client
    let client = Client::new();

    let devicecode = {
        let mut devicecode = devicecode.write().await;

        match devicecode.as_ref() {
            None => {
                let data = DeviceCode::fetch(&client, MICROSOFT_CLIENT_ID).await?;
                let expiring_data = ExpiringData::from(data.clone());
                *devicecode = Some(expiring_data.clone());
                expiring_data
            }
            Some(data) => {
                if data.is_expired(){
                    let data = DeviceCode::fetch(&client, MICROSOFT_CLIENT_ID).await?;
                    let expiring_data = ExpiringData::from(data.clone());
                    *devicecode = Some(expiring_data.clone());
                    expiring_data
                }else{
                    devicecode.clone().unwrap()
                }
            }
        }
    };

    Ok(devicecode.into())
}


#[tauri::command]
pub async fn refresh(
    devicecode: State<'_, NLDevicecode>,
    step:State<'_,NLAuthStep>,
) -> CommandResult<DevicecodeInfo>{

    let mut _lock = step.lock().await;

    let client = Client::new();

    let mut devicecode = devicecode.write().await;
    let data = DeviceCode::fetch(&client, MICROSOFT_CLIENT_ID).await?;
    let expiring_data = ExpiringData::from(data.clone());
    *devicecode = Some(expiring_data.clone());
    *_lock = AuthStep::Exchange;

    Ok(expiring_data.into())
}


#[derive(Debug,Clone,Serialize)]
#[serde(tag = "action", content = "second")]
pub enum ExchangeStatus{
    Success,
    Pending(i8)
}


#[tauri::command]
pub async fn exchange(
    devicecode: State<'_, NLDevicecode>,
    step:State<'_,NLAuthStep>
) -> CommandResult<ExchangeStatus> {

    let mut _lock = step.lock().await;
    
    if *_lock != AuthStep::Exchange{
        return Err(anyhow!("Unexpected Step: {:?}",*_lock).into())
    }

    let client = Client::new();

    let reader = devicecode.read().await;

    let inner = reader
        .as_ref()
        .ok_or(anyhow!("device code is invalid!"))?
        .get_ref();

    let result = inner.exchange(&client, MICROSOFT_CLIENT_ID)
        .await;

    // drop reader lock to unlock the RwLock
    drop(reader);

    match result {
        Ok(data) => {
            let mut devicecode = devicecode.write().await;
            *devicecode = None; // clear the device code data since it's no longer valid
            *_lock = AuthStep::XboxLive(data); // change step to xbox live
            Ok(ExchangeStatus::Success)
        }
        Err(error) => {
            match error {
                MicrosoftAuthError::AuthorizationPending => {
                    Ok(ExchangeStatus::Pending(5))
                }
                _type => {
                    Err(anyhow!("error while fetching device code. Details:{:?}",_type).into())
                }
            }
        }
    }
}