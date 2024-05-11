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
use log::info;
use oauth2::RefreshToken;

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

pub(crate) struct MicrosoftAuthFlow {
    client: BasicClient,
}

impl MicrosoftAuthFlow {
    
    pub fn new() -> Result<Self> {
        let client = BasicClient::new(
            ClientId::new(MSA_CLIENT_ID.to_string()),
            None,
            AuthUrl::new(MSA_AUTHORIZE_URL.to_string())?,
            Some(TokenUrl::new(MSA_TOKEN_URL.to_string())?)
        )
            .set_device_authorization_url(DeviceAuthorizationUrl::new(DEVICE_CODE_URL.to_string()).expect("Invalid Device Code URL"))
            .set_auth_type(AuthType::RequestBody);
        
        
        Ok(Self {
            client
        })
    }

    pub async fn refresh_token(&self,refresh_token:String) -> Result<BasicTokenResponse> {
        let refresh_token = RefreshToken::new(refresh_token);
        let token = self.client
            .exchange_refresh_token(&refresh_token)
            .request_async(async_http_client)
            .await?;
        Ok(token)
    }
    
    pub async fn generate_msa_device_code_auth(&self) -> Result<StandardDeviceAuthorizationResponse> {
        let details: StandardDeviceAuthorizationResponse = self.client
            .exchange_device_code()? 
            .add_scope(Scope::new(SCOPE.to_string()))
            .request_async(async_http_client)
            .await?;

        Ok(details)
    }

    pub async fn get_msa_token(&self, details: &StandardDeviceAuthorizationResponse) -> Result<BasicTokenResponse> {
        info!("Token expire in {:?}",details.expires_in());
        let token = self.client
            .exchange_device_access_token(details)
            .request_async(async_http_client, tokio::time::sleep, Some(details.expires_in()))
            .await?;
        Ok(token)
    }
}