use arboard::Clipboard;
use oauth2::{StandardDeviceAuthorizationResponse, TokenResponse};
use reqwest::Client;
use serde_json::{from_str, json};
use tauri::Manager;
use log::error;
use crate::auth::minecraft::MinecraftAuthorizationFlow;

pub(crate) mod msa_auth;
mod minecraft;

const SUCCESS: i32 = 0;
const DEVICE_CODE_FLOW_ERROR: i32 = 1;
const XBOX_ERROR: i32 = 2;
const MINECRAFT_PROFILE_ERROR: i32 = 3;

#[tauri::command]
pub async fn msa_auth_init() -> String {
    
    let ms_auth_flow = match msa_auth::MicrosoftAuthFlow::new(){
        Ok(flow) => flow,
        Err(e) => {
            return json!({
                "status": DEVICE_CODE_FLOW_ERROR,
            }).to_string()
        }
    };
    let detail:StandardDeviceAuthorizationResponse = match ms_auth_flow.generate_msa_device_code_auth().await{
        Ok(detail) => detail,
        Err(e) => {
            return json!({
                "status": DEVICE_CODE_FLOW_ERROR
            }).to_string()
        }
    };

    println!(
        "Open this URL in your browser:\n{}\nand enter the code: {}",
        &detail.verification_uri().to_string(),
        &detail.user_code().secret().to_string()
    );
    
    serde_json::to_string(&detail).unwrap()
}

#[derive(serde::Deserialize,serde::Serialize)]
struct InnerMessage{
    user_code: String,
    verification_uri: String,
}

#[tauri::command]
pub async fn msa_auth_open_browser(invoke_message: String) -> String {
    let mut clipboard = Clipboard::new().unwrap();
    let detail:StandardDeviceAuthorizationResponse = from_str(&invoke_message).unwrap();
    clipboard.set_text(&detail.user_code().secret().to_string()).unwrap();
    open::that(format!("{}?otc={}",&detail.verification_uri().to_string(),&detail.user_code().secret().to_string()).to_string()).unwrap();
    json!({
        "status": SUCCESS
    }).to_string()
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[tauri::command]
pub async fn msa_auth_exchange(invoke_message: String,app: tauri::AppHandle)-> String{
    
    let detail:StandardDeviceAuthorizationResponse = from_str(&invoke_message).unwrap();
    let ms_auth_flow = match msa_auth::MicrosoftAuthFlow::new(){
        Ok(flow) => flow,
        Err(e) => {
            error!("stop auth 1 {}", e.to_string());
            return json!({
                "status":DEVICE_CODE_FLOW_ERROR,
                "detail":e.to_string()
            }).to_string()
        }
    };
    
    let token = match ms_auth_flow.get_msa_token(&detail).await{
        Ok(token) => token,
        Err(e) => {
            error!("stop auth 2 {}",e.to_string());
            return json!({
                "status":DEVICE_CODE_FLOW_ERROR,
                "detail":e.to_string()
            }).to_string()
        }
    };

    app.emit_all("mc_login", Payload { message: "Fetching Xbox Token......".into() }).unwrap();


    let mc_flow = MinecraftAuthorizationFlow::new(Client::new());
    let (xbox_token,user_hash) = match mc_flow.xbox_token(token.access_token().secret()).await{
        Ok((token,hash)) => (token,hash),
        Err(e) => {
            error!("stop auth 3 {}",e.to_string());
            return json!({
                "status":XBOX_ERROR,
                "detail":e.to_string()
            }).to_string()
        }
    };

    app.emit_all("mc_login", Payload { message: "Fetching Xbox Security Token......".into() }).unwrap();

    let xbox_xsts_token = match mc_flow.xbox_security_token(xbox_token).await{
        Ok(token) => token,
        Err(e) => {
            error!("stop auth 3 {}",e.to_string());
            return json!({
                "status":XBOX_ERROR,
                "detail":e.to_string()
            }).to_string()
        }
    };

    app.emit_all("mc_login", Payload { message: "Fetching Minecraft Token......".into() }).unwrap();

    let mc_token = match mc_flow.exchange_microsoft_token(user_hash,xbox_xsts_token).await{
        Ok(token) => token,
        Err(e) => {
            error!("stop auth 4 {}",e.to_string());
            return json!({
                "status":XBOX_ERROR,
                "detail":e.to_string()
            }).to_string()
        }
    };

    app.emit_all("mc_login", Payload { message: "Fetching Your Profile......".into() }).unwrap();

    let profile = match mc_flow.get_minecraft_profile(mc_token.access_token).await{
        Ok(profile) => profile,
        Err(e) => {
            error!("stop auth 5 {}",e.to_string());
            return json!({
                "status":MINECRAFT_PROFILE_ERROR,
                "detail":e.to_string()
            }).to_string()
        }
    };

    app.emit_all("mc_login", Payload { message: "Good! ......".into() }).unwrap();

    json!(
        {
            "status":"success"
        }
    ).to_string()
}

// #[cfg(test)]
// mod tests {
// 
//     use oauth2::{StandardDeviceAuthorizationResponse, TokenResponse};
// 
//     use reqwest::Client;
//     use crate::auth::minecraft::MinecraftAuthorizationFlow;
//     use crate::auth::msa_auth;
// 
// 
//     #[tokio::test]
//     pub async fn msa_test() -> Result<(), String>{
//         let ms_auth_flow = match msa_auth::MicrosoftAuthFlow::new(){
//             Ok(flow) => flow,
//             Err(e) => {
//                 return Err(e.to_string())
//             }
//         };
// 
//         let detail:StandardDeviceAuthorizationResponse = match ms_auth_flow.generate_msa_device_code_auth().await{
//             Ok(detail) => detail,
//             Err(e) => {
//                 return Err(e.to_string())
//             }
//         };
// 
//         println!(
//             "Open this URL in your browser:\n{}\nand enter the code: {}",
//             &detail.verification_uri().to_string(),
//             &detail.user_code().secret().to_string()
//         );
// 
//         open::that(format!("{}?otc={}",detail.verification_uri().to_string(),detail.user_code().secret().to_string()).to_string()).unwrap();
// 
//         println!("Waiting for user to authorize the app...{:?}",detail);
// 
//         // let token = client
//         //     .exchange_device_access_token(&detail.user_code().secret())
//         //     .request_async(async_http_client, tokio::time::sleep, None)
//         //     .await?;
//         // println!("microsoft token: {:?}", token);
// 
//         let token = match ms_auth_flow.await_exchenge(&detail).await{
//             Ok(token) => token,
//             Err(e) => {
//                 return Err(e.to_string())
//             }
//         };
// 
//         let mc_flow = MinecraftAuthorizationFlow::new(Client::new());
//         let (xbox_token,user_hash) = match mc_flow.xbox_token(token.access_token().secret()).await{
//             Ok((token,hash)) => (token,hash),
//             Err(e) => {
//                 return Err(e.to_string())
//             }
//         };
//         let xbox_xsts_token = match mc_flow.xbox_security_token(xbox_token).await{
//             Ok(token) => token,
//             Err(e) => {
//                 return Err(e.to_string())
//             }
//         };
//         let mc_token = match mc_flow.exchange_microsoft_token(user_hash,xbox_xsts_token).await{
//             Ok(token) => token,
//             Err(e) => {
//                 return Err(e.to_string())
//             }
//         };
// 
//         println!("{}",mc_token.access_token);
//         Ok(())
//     }
// }
