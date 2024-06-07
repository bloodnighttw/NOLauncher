use std::sync::Arc;
use log::{warn};
use serde_json::json;
use tauri::{AppHandle, State};
use crate::minecraft::auth::{AuthFlow, MinecraftAuthError, MinecraftAuthStep, MinecraftUUIDMap, save};

#[tauri::command]
pub async fn devicecode_init(authflow_rwlock: State<'_,AuthFlow>) -> Result<String,String> {
    if let MinecraftAuthStep::DeviceCode(var) = &authflow_rwlock.read().await.status {
        if var.is_vaild(){
            return Ok(json!({
                "user_code": var.data.user_code,
                "device_code": var.data.device_code,
                "verification_uri": var.data.verification_uri,
            }).to_string());
        }
    }

    {
        let mut auth_flow = authflow_rwlock.write().await;
        let response = &auth_flow.generate_device_code().await;
        match response {
            Ok(_) => {
                if let MinecraftAuthStep::DeviceCode(var) = &auth_flow.status {
                    if var.is_vaild() {
                        let expire_in = var.time + var.data.expires_in;
                        return Ok(json!({
                            "user_code": var.data.user_code,
                            "expire_in": expire_in.to_rfc3339(),
                            "verification_uri": var.data.verification_uri,
                        }).to_string());
                    }

                }
                Err(json!({
                    "status": "Failed to fetch the device code"
                }).to_string())
            }
            Err(_) => {
                Err (json!({
                    "status": "Failed to fetch the device code"
                }).to_string())
            }
        }
    }
}

#[tauri::command]
pub async fn devicecode_exchange(authflow_rwlock: State<'_,AuthFlow>) -> Result<String,String> {
    loop {
        {
            let mut auth_flow = authflow_rwlock.write().await;
            match &auth_flow.status {
                MinecraftAuthStep::DeviceCode(var) => {
                    if !var.is_vaild() {
                        return Err(json!({
                            "status": "error",
                            "description": "The device code is expired"
                        }).to_string());
                    }
                }
                _ => {
                    return Err(json!({
                        "status": "error",
                        "description": "The device code is not ready to exchange or status is not DeviceCode"
                    }).to_string());
                }
            }
            let res = auth_flow.exchange_device_code().await;
            match res {
                Ok(_) => { break; }
                Err(e) => {
                    match e {
                        MinecraftAuthError::AuthorizationPending => {} // just wait next time
                        _ => {
                            return Err(json!({
                                "status": "error",
                                "description": "Failed to exchange the device code"
                            }).to_string());
                        }
                    }
                }
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }

    Ok(json!({
        "status": "success",
        "description": "getting the Xbox Live Auth Token......"
    }).to_string())
}

#[tauri::command]
pub async fn xbox_live_auth(authflow_rwlock: State<'_,AuthFlow>) -> Result<String,String> {
    let response = {
        let mut authflow = authflow_rwlock.write().await;
        authflow.xbox_live_auth().await
    };
    match response {
        Ok(_) => {
            Ok(json!({
                    "status": "success",
                    "description": "Xbox Live Auth Success,Getting the XSTS token"
                }).to_string())
        }
        Err(e) => {
            Err(json!({
                    "status": "error",
                    "description": e.to_string()
                }).to_string())
        }
    }
}

#[tauri::command]
pub async fn xbox_xsts_auth(authflow_rwlock: State<'_,AuthFlow>) -> Result<String,String> {
    let response = {
        let mut authflow = authflow_rwlock.write().await;
        authflow.xbox_security_auth().await
    };
    match response {
        Ok(_) => {
            Ok(json!({
                    "status": "success",
                    "description": "getting the token for Minecraft"
                }).to_string())
        }
        Err(e) => {
            Err(json!({
                    "status": "error",
                    "description": e.to_string()
                }).to_string())
        }
    }
}

#[tauri::command]
pub async fn minecraft_token(authflow_rwlock: State<'_,AuthFlow>) -> Result<String,String> {
    let response = {
        let mut authflow = authflow_rwlock.write().await;
        authflow.get_minecraft_token().await
    };
    match response {
        Ok(_) => {
            Ok(json!({
                    "status": "success",
                    "description": "checking the profile..."
                }).to_string())
        }
        Err(e) => {
            Err(json!({
                    "status": "error",
                    "description": e.to_string()
                }).to_string())
        }
    }
}

#[tauri::command]
pub async fn minecraft_profile(authflow_rwlock: State<'_,AuthFlow>, map:State<'_,MinecraftUUIDMap>,app:AppHandle) -> Result<String,String> {
    let response = {
        let mut authflow = authflow_rwlock.write().await;
        authflow.check_minecraft_profile().await
    };
    
    match response {
        Ok(login_data) => {

            let login_data = Arc::new(login_data);
            {
                let mut user = map.write().await;
                user.insert(login_data.profile.clone().id.clone(),login_data.clone());
            }
            
            let response = save(&app).await;

            if let Err(e) = response {
                warn!("Failed to save the user data. details: {}",e);
            }
            
            crate::event::user::change_user(login_data.clone(),&app).await;
            
            Ok(json!({
                    "status": "success"
                }).to_string())
        }
        Err(e) => {
            Err(json!({
                    "status": "error",
                    "description": e.to_string()
                }).to_string())
        }
    }
}
