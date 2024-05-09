use arboard::Clipboard;
use oauth2::StandardDeviceAuthorizationResponse;
use serde_json::json;

pub(crate) mod msa_auth;
mod minecraft;

#[tauri::command]
pub async fn msa_auth_init() -> String {
    
    let ms_auth_flow = match msa_auth::MicrosoftAuthFlow::new(){
        Ok(flow) => flow,
        Err(e) => {
            return json!({
                "error": e.to_string()
            }).to_string()
        }
    };
    let detail:StandardDeviceAuthorizationResponse = match ms_auth_flow.generate_msa_device_code_auth().await{
        Ok(detail) => detail,
        Err(e) => {
            return json!({
                "error": e.to_string()
            }).to_string()
        }
    };

    println!(
        "Open this URL in your browser:\n{}\nand enter the code: {}",
        &detail.verification_uri().to_string(),
        &detail.user_code().secret().to_string()
    );
    
    
    json!({
        "verification_uri": detail.verification_uri().to_string(),
        "user_code": detail.user_code().secret().to_string(),
    }).to_string()

}

#[derive(serde::Deserialize,serde::Serialize)]
struct InnerMessage{
    user_code: String,
    verification_uri: String,
}

#[tauri::command]
pub async fn msa_auth_open_browser(invoke_message: String) -> String {
    let mut clipboard = Clipboard::new().unwrap();
    let detail:InnerMessage = serde_json::from_str(&invoke_message).unwrap();
    clipboard.set_text(detail.user_code.clone()).unwrap();
    open::that(format!("{}?otc={}",detail.verification_uri,detail.user_code).to_string()).unwrap();
    json!({
        "status": "success"
    }).to_string()
}


// #[cfg(test)]
// mod tests {
//     use oauth2::StandardDeviceAuthorizationResponse;
// 
//     use reqwest::Client;
//     use serde_json::json;
//     use crate::auth::minecraft::MinecraftAuthorizationFlow;
//     use crate::auth::msa_auth;
// 
// 
//     #[tokio::test]
//     pub async fn msa_test() -> Result<()>{
//         let ms_auth_flow = match msa_auth::MicrosoftAuthFlow::new(){
//             Ok(flow) => flow,
//             Err(e) => {
//                 return json!({
//                 "error": e.to_string()
//             }).to_string()
//             }
//         };
//         
//         let detail:StandardDeviceAuthorizationResponse = match ms_auth_flow.generate_msa_device_code_auth().await{
//             Ok(detail) => detail,
//             Err(e) => {
//                 return json!({
//                 "error": e.to_string()
//             }).to_string()
//             }
//         };
// 
//         println!(
//             "Open this URL in your browser:\n{}\nand enter the code: {}",
//             &detail.verification_uri().to_string(),
//             &detail.user_code().secret().to_string()
//         );
// 
//         open::that(detail.verification_uri().to_string()).unwrap();
// 
//         let mc_flow = MinecraftAuthorizationFlow::new(Client::new());
//         let (xbox_token,user_hash) = mc_flow.xbox_token(detail.user_code().secret()).unwrap();
//         let xbox_xsts_token = mc_flow.xbox_security_token(xbox_token).unwrap();
//         let mc_token = mc_flow.exchange_microsoft_token(user_hash,xbox_xsts_token).unwrap();
// 
//         println!("{}",mc_token.access_token);
//     }
// }
