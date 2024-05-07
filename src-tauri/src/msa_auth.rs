/*  To gain Minecraft Access Token,You do these thing:
 *      1. Gain the key to access XBOX LIVE API         <--- You are here.
 *      2. Obtain XSTS token for Minecraft              <--- Next
 *      3. Authenticate with Minecraft                  <--- To get & check user Minecraft information
 *
 *  You can find more details on https://wiki.vg/Microsoft_Authentication_Scheme
 */

use oauth2::basic::{BasicClient, BasicTokenResponse};
use oauth2::{AuthType, AuthUrl, ClientId, DeviceAuthorizationUrl, Scope, StandardDeviceAuthorizationResponse, TokenUrl};
use oauth2::reqwest::async_http_client;
use anyhow::Result;

const MSA_CLIENT_ID: &str = env!("MICROSOFT_CLIENT_ID");
const DEVICE_CODE_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode";
const MSA_AUTHORIZE_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize";
const MSA_TOKEN_URL: &str = "https://login.microsoftonline.com/common/oauth2/v2.0/token";
const SCOPE:&str = "XboxLive.signin offline_access";

/*  To understand what these code is doing, please refer to the following links:
 *  1. https://learn.microsoft.com/en-us/entra/identity-platform/v2-oauth2-device-code for the device code auth flow
 *  2. https://learn.microsoft.com/en-us/entra/identity-platform/quickstart-register-app for registering an app with Microsoft Entra
 *  
 *  Don't forget to enable Allow public client flows in Authentication > Advanced settings, 
 *  if you don't enable it, you won't be able to use device code auth flow.
 */

#[tokio::main]
async fn create_client() -> Result<BasicClient> {

    let client = BasicClient::new(
        ClientId::new(MSA_CLIENT_ID.to_string()),
        None,
        AuthUrl::new(MSA_AUTHORIZE_URL.to_string())?,
        Some(TokenUrl::new(MSA_TOKEN_URL.to_string())?),
    )
        .set_device_authorization_url(DeviceAuthorizationUrl::new(DEVICE_CODE_URL.to_string())?)
        .set_auth_type(AuthType::RequestBody);

    Ok(client)
}


#[tokio::main]
async fn generate_msa_device_code_auth(client: &BasicClient) -> Result<StandardDeviceAuthorizationResponse> {
    
    let details: StandardDeviceAuthorizationResponse = client
        .exchange_device_code()?
        .add_scope(Scope::new(SCOPE.to_string()))
        .request_async(async_http_client)
        .await?;
    
    Ok(details)
}

#[tokio::main]
async fn get_msa_token(client: &BasicClient, details:&StandardDeviceAuthorizationResponse) -> Result<BasicTokenResponse>{
    let token = client
        .exchange_device_access_token(&details)
        .request_async(async_http_client, tokio::time::sleep, None)
        .await?;
    Ok(token)
}

#[cfg(test)]
mod tests {
    use oauth2::StandardDeviceAuthorizationResponse;
    use super::{create_client, generate_msa_device_code_auth, get_msa_token};
    use oauth2::TokenResponse;

    #[test]
    fn msa_test() {
        let client = create_client().unwrap();
        let detail:StandardDeviceAuthorizationResponse = generate_msa_device_code_auth(&client).unwrap();

        println!(
            "Open this URL in your browser:\n{}\nand enter the code: {}",
            detail.verification_uri().to_string(),
            detail.user_code().secret().to_string()
        );
        
        open::that(detail.verification_uri().to_string()).unwrap();
        let token = get_msa_token(&client, &detail).unwrap();
        
        println!("Access Token: {}", token.access_token().secret());
        
    }
}