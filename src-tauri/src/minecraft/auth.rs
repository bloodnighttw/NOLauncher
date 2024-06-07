use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use reqwest::{Client};
use reqwest::header::{CONTENT_TYPE, PRAGMA};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{json, Value};
use thiserror::Error;
use tokio::sync::RwLock;
use crate::utils::data::{TimeSensitiveData, TimeSensitiveTrait};
use anyhow::Result;
use tauri::{AppHandle, Manager, State};

const DEVICECODE_URL:&str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode";
const TOKEN_URL:&str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token";
const MINECRAFT_LOGIN_WITH_XBOX: &str = "https://api.minecraftservices.com/authentication/login_with_xbox";
const XBOX_USER_AUTHENTICATE: &str = "https://user.auth.xboxlive.com/user/authenticate";
const XBOX_XSTS_AUTHORIZE: &str = "https://xsts.auth.xboxlive.com/xsts/authorize";
const MINECRAFT_PROFILE: &str = "https://api.minecraftservices.com/minecraft/profile";
const SCOPE: &str = "XboxLive.signin offline_access";
const GRANT_TYPE: &str = "urn:ietf:params:oauth:grant-type:device_code";

pub enum MinecraftAuthStep{
    /// Initialize the Minecraft Authorization Flow.
    Init(),
    /// Generate a device code for the user to authorize.
    /// This is the first step in the Minecraft Authorization Flow.
    /// The user must open the URL in a browser and enter the code.
    /// The user code is valid for a limited time.
    /// The user must authorize the app within this time.
    /// If the user does not authorize the app within this time, the user code will expire.
    DeviceCode(TimeSensitiveData<DeviceCodeResponse>),
    /// Wait for the user to authorize the app.
    MicrosoftAuth(Arc<RwLock<TimeSensitiveData<MicrosoftAuthResponse>>>),

    /// Exchange the device code for an access token.
    XboxLiveAuth(Arc<RwLock<TimeSensitiveData<MicrosoftAuthResponse>>>,String),
    /// Exchange the Xbox Live access token for an Xbox Security Token.
    XboxSecurityAuth(Arc<RwLock<TimeSensitiveData<MicrosoftAuthResponse>>>,String,String),
    MinecraftAuth(Arc<RwLock<TimeSensitiveData<MicrosoftAuthResponse>>>,Arc<TimeSensitiveData<MinecraftAuthResponse>>),
}

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum MinecraftAuthError{

    #[error("Your Minecraft Auth Flow call wrong step! Please check your code.")]
    InvalidState,
    #[error("Failed to get device code. details:{0}")]
    GetDeviceCodeError(String),
    #[error("Failed to exchange device code. please try again. details:AuthorizationPending")]
    AuthorizationPending,
    #[error("Failed to exchange device code. details: AuthorizationDeclined")]
    AuthorizationDeclined,
    #[error("Failed to exchange device code. details: BadVerificationCode")]
    BadVerificationCode,
    #[error("Failed to exchange device code. details: ExpiredToken")]
    ExpiredToken,
    #[error("Failed to refresh token code. details:{0}")]
    RefreshMicrosoftTokenError(String),
    #[error("Failed to fetching Xbox Data. details:{0}")]
    XboxAuthError(String),
    #[error("The account doesn't have an Xbox account. Once they sign up for one (or login through minecraft.net to create one) then they can proceed with the login")]
    XboxAccountNotExist,
    #[error("The account doesn't have a Minecraft account. Once they sign up for one (or login through minecraft.net to create one) then they can proceed with the login")]
    XboxAccountCountryBan,
    #[error("The account needs adult verification on Xbox page")]
    XboxAccountNeedAdultVerification,
    #[error("The account is a child (under 18) and cannot proceed unless the account is added to a Family by an adult.")]
    AddToFamily,
    #[error("Profile Not Found. details:{0}")]
    ProfileNotFound(String),
    #[error("Unknown Error. details:{0}")]
    UnknownError(String),
}

fn to_duration<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Duration, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::Number(num) =>{
            let v = num.as_u64().ok_or(de::Error::custom("Invalid number"))?;
            Duration::from_secs(v)
        } ,
        _ => return Err(de::Error::custom("wrong type"))
    })
}

fn to_u64<S>(x: &Duration, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    s.serialize_u64(x.as_secs())
}

#[derive(Debug, serde::Deserialize ,serde::Serialize,Clone)]
pub struct DeviceCodeResponse{
    pub user_code: String,
    pub device_code: String,
    pub verification_uri: String,
    #[serde(deserialize_with = "to_duration",serialize_with = "to_u64")]
    pub expires_in: Duration,
    pub interval: u64,
}

impl TimeSensitiveTrait for DeviceCodeResponse {
    fn get_duration(&self) -> Duration {
        self.expires_in
    }
}

#[derive(Debug, serde::Deserialize ,serde::Serialize)]
pub struct MicrosoftAuthResponse{
    pub token_type: String,
    pub scope: String,
    #[serde(deserialize_with = "to_duration",serialize_with = "to_u64")]
    pub expires_in: Duration,
    pub ext_expires_in: u64,
    pub access_token: String,
    pub refresh_token: String,
}

impl TimeSensitiveTrait for MicrosoftAuthResponse {
    fn get_duration(&self) -> Duration {
        self.expires_in
    }
}


pub struct MinecraftAuthorizationFlow {
    client: Client,
    client_id: String,
    pub status: MinecraftAuthStep
}

#[derive(Serialize, Deserialize)]
pub struct MinecraftAuthResponse{
    username: String,
    access_token: String,
    #[serde(deserialize_with = "to_duration",serialize_with = "to_u64")]
    expires_in: Duration,
    token_type: String,
}

impl TimeSensitiveTrait for MinecraftAuthResponse {
    fn get_duration(&self) -> Duration {
        self.expires_in
    }
}

#[derive(Debug,Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftSkin {
    pub id: String,
    pub state: String,
    pub url: String,
    pub texture_key: String,
    pub variant: String,
}

#[derive(Debug,Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftCaps {
    pub id: String,
    pub state: String,
    pub url: String,
    pub alias: String,
}


/// Represents the information of user's Minecraft profile
#[derive(Debug,Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftProfile {
    pub id: String,
    pub name: String,
    pub skins: Vec<MinecraftSkin>,
    pub capes: Vec<MinecraftCaps>,
}

impl MinecraftAuthorizationFlow {

    pub fn new(client_id:&str) -> Self {
        Self {
            client: Client::new(),
            client_id: client_id.to_string(),
            status: MinecraftAuthStep::Init()
        }
    }

    pub fn from(client_id:&str,step:MinecraftAuthStep) -> Self {
        Self {
            client: Client::new(),
            client_id: client_id.to_string(),
            status: step
        }
    }

    pub fn reset(&mut self){
        self.status = MinecraftAuthStep::Init();
    }

    pub async fn generate_device_code(&mut self) -> Result<(), MinecraftAuthError>{
        let params:HashMap<String,String> = HashMap::from([
            (String::from("client_id"),self.client_id.clone()),
            (String::from("scope"),String::from(SCOPE)),
        ]);
        let response = self.client.post(DEVICECODE_URL)
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await;

        let data:DeviceCodeResponse = match response{
            Ok(response) => {
                if response.status() == 200{
                    response.json().await.expect("this should be success!")
                }else{
                    return Err(MinecraftAuthError::GetDeviceCodeError(format!("Failed to get device code. status code:{}",response.status())))
                }
            },
            Err(e) => return Err(MinecraftAuthError::GetDeviceCodeError(e.to_string()))
        };

        self.status = MinecraftAuthStep::DeviceCode(TimeSensitiveData::new(data));
        Ok(())
    }

    pub async fn exchange_device_code(&mut self) -> Result<(), MinecraftAuthError>{
        let data = match &self.status{
            MinecraftAuthStep::DeviceCode(data) => data,
            _ => return Err(MinecraftAuthError::InvalidState)
        };

        let params:HashMap<String,String> = HashMap::from([
            (String::from("client_id"),self.client_id.clone()),
            (String::from("grant_type"),String::from(GRANT_TYPE)),
            (String::from("device_code"),data.data.device_code.clone()),
        ]);

        let request = self.client.post(TOKEN_URL)
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .form(&params);

        let response = request.try_clone().expect("should can").send().await;
        let response:MicrosoftAuthResponse = match response{
            Ok(response) => {
                if response.status() == 200{
                    response.json().await.expect("this should be success!")
                }else {
                    return match response.json::<Value>().await.expect("this should be success!")
                        .get("error").expect("this should be success!")
                        .as_str().expect("this should be success!"){
                        "authorization_pending" => Err(MinecraftAuthError::AuthorizationPending),
                        "authorization_declined" => Err(MinecraftAuthError::AuthorizationDeclined),
                        "bad_verification_code" => Err(MinecraftAuthError::BadVerificationCode),
                        "expired_token" => Err(MinecraftAuthError::ExpiredToken),
                        _ => Err(MinecraftAuthError::UnknownError("Unknown Error".to_string()))
                    }
                }
            },
            Err(e) => return Err(MinecraftAuthError::UnknownError(e.to_string()))
        };

        self.status = MinecraftAuthStep::MicrosoftAuth(Arc::new(RwLock::new(TimeSensitiveData::new(response))));

        Ok(())
    }

    pub async fn xbox_live_auth(&mut self) -> Result<(), MinecraftAuthError>{
        let data = match &self.status{
            MinecraftAuthStep::MicrosoftAuth(data) => data,
            _ => return Err(MinecraftAuthError::InvalidState)
        };

        let xbox_authenticate_json = {
            let r_data = data.write().await;
            let xbox_authenticate_json = json!({
                "Properties": {
                    "AuthMethod": "RPS",
                    "SiteName": "user.auth.xboxlive.com",
                    "RpsTicket": &format!("d={}", r_data.data.access_token)
                },
                "RelyingParty": "http://auth.xboxlive.com",
                "TokenType": "JWT"
            });
            xbox_authenticate_json
        };

        let response = self
            .client
            .post(XBOX_USER_AUTHENTICATE)
            .json(&xbox_authenticate_json)
            .send()
            .await;

        let res = match response{
            Ok(response) => {
                if response.status() == 200{
                    response
                }else{
                    return Err(MinecraftAuthError::XboxAuthError(format!("Failed to get Xbox Data. status code:{}",response.status())))
                }
            },
            Err(e) => return Err(MinecraftAuthError::UnknownError(e.to_string()))
        };

        let token = res.json::<Value>().await.expect("this should be success!")
            .get("Token").expect("this should be success!")
            .as_str().expect("this should be success!").to_string();

        self.status = MinecraftAuthStep::XboxLiveAuth(data.clone(),token);

        Ok(())
    }

    pub async fn xbox_security_auth(&mut self) -> Result<(), MinecraftAuthError>{
        let (data,token) = match &self.status{
            MinecraftAuthStep::XboxLiveAuth(data,token) => (data,token),
            _ => return Err(MinecraftAuthError::InvalidState)
        };

        let xbox_authenticate_json = json!({
            "Properties": {
                "SandboxId": "RETAIL",
                "UserTokens": [token],
            },
            "RelyingParty": "rp://api.minecraftservices.com/",
            "TokenType": "JWT"
        });
        let response = self
            .client
            .post(XBOX_XSTS_AUTHORIZE)
            .json(&xbox_authenticate_json)
            .send()
            .await;

        let res = match response{
            Ok(response) => {
                if response.status() == 200{
                    response
                }else{
                    let value = response.json::<Value>().await.expect("this should be success!");
                    return match value["XErr"].as_u64().expect("should be x") {
                        2148916233 => Err(MinecraftAuthError::XboxAccountNotExist),
                        2148916235 => Err(MinecraftAuthError::XboxAccountCountryBan),
                        2148916236|2148916237 => Err(MinecraftAuthError::XboxAccountNeedAdultVerification),
                        2148916238 => Err(MinecraftAuthError::AddToFamily),
                        _ => Err(MinecraftAuthError::XboxAuthError("Unknown Error".to_string()))
                    }
                }
            },
            Err(e) => return Err(MinecraftAuthError::UnknownError(e.to_string()))
        };

        let value = res.json::<Value>().await.or(Err(MinecraftAuthError::UnknownError("Failed to parse response".to_string())))?;
        let token = value["Token"].as_str().expect("this should be success!").to_string();
        let user_hash = value["DisplayClaims"]["xui"][0]["uhs"].as_str().expect("this should be success!").to_string();

        self.status = MinecraftAuthStep::XboxSecurityAuth(data.clone(),token,user_hash);

        Ok(())
    }

    pub async fn get_minecraft_token(&mut self) -> Result<Arc<TimeSensitiveData<MinecraftAuthResponse>>,MinecraftAuthError>{
        let (data,token,uhs) = match &self.status{
            MinecraftAuthStep::XboxSecurityAuth(data,token,user_hash) => (data,token,user_hash),
            _ => return Err(MinecraftAuthError::InvalidState)
        };

        let response = self.client.post(MINECRAFT_LOGIN_WITH_XBOX)
            .header("Content-Type", "application/json")
            .json(&json!({
                "identityToken": format!("XBL3.0 x={};{}",uhs,token)
            }))
            .send()
            .await;

        let res:MinecraftAuthResponse = match response{
            Ok(response) => {
                if response.status() == 200{
                    response.json().await.expect("this should be success!")
                }else{
                    return Err(MinecraftAuthError::XboxAuthError(format!("Failed to get Xbox Data. status code:{}",response.status())))
                }
            },
            Err(e) => return Err(MinecraftAuthError::UnknownError(e.to_string()))
        };

        let minecraft_auth = Arc::new(TimeSensitiveData::new(res));
        self.status = MinecraftAuthStep::MinecraftAuth(data.clone(), minecraft_auth.clone());

        Ok(minecraft_auth.clone())
    }

    pub async fn check_minecraft_profile(&mut self) -> Result<
        LoginAccount
        ,MinecraftAuthError> {
        let (data,profile) = match &self.status {
            MinecraftAuthStep::MinecraftAuth(data,profile) => (data.clone(),profile.clone()),
            _ => return Err(MinecraftAuthError::InvalidState)
        };

        let response = self.client.get(MINECRAFT_PROFILE)
            .bearer_auth(profile.data.access_token.clone())
            .send()
            .await;

        let profile_data = match response {
            Ok(response) => {
                if response.status() == 200 {
                    response.json::<MinecraftProfile>().await.expect("this should be success!")
                } else {
                    return Err(MinecraftAuthError::ProfileNotFound(response.text().await.expect("this should be success!")))
                }
            },
            Err(e) => {
                return Err(MinecraftAuthError::UnknownError(e.to_string()))
            }
        };

        self.reset();
        
        let profile_data = Arc::new(profile_data);

        Ok(LoginAccount {
            microsoft: data.clone(),
            profile: profile_data.clone()
        })
        
    }
}

pub type AuthFlow = RwLock<MinecraftAuthorizationFlow>;

pub struct LoginAccount {
    pub microsoft: Arc<RwLock<TimeSensitiveData<MicrosoftAuthResponse>>>,
    pub profile: Arc<MinecraftProfile>
}

#[derive(Clone,Serialize, Deserialize)]
pub struct MinecraftLaunchData{
    pub profile: Arc<MinecraftProfile>,
    pub token: Arc<TimeSensitiveData<MinecraftAuthResponse>>
}

pub type MinecraftUUIDMap = RwLock<HashMap<String, Arc<LoginAccount>>>;

impl LoginAccount {

    pub async fn check_microsoft_token(&mut self) -> Result<(),String>{
        if !self.microsoft.read().await.is_vaild(){
            return Ok(())
        }

        {
            let mut data = self.microsoft.write().await;
            let params:HashMap<String,String> = HashMap::from([
                (String::from("client_id"),env!("MICROSOFT_CLIENT_ID").to_string()),
                (String::from("scope"),String::from(SCOPE)),
                (String::from("refresh_token"),data.data.refresh_token.clone()),
                (String::from("grant_type"),String::from("refresh_token")),
            ]);

            let client = Client::new();

            let response = client.post(TOKEN_URL)
                .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                .form(&params)
                .header(PRAGMA, "no-cache")
                .header("Cache-Control", "no-store")
                .send()
                .await;

            let refresh_token:MicrosoftAuthResponse = match response{
                Ok(response) => {
                    if response.status() == 200{
                        response.json().await.expect("this should be success!")
                    }else{
                        return Err(format!("Failed to refresh token. status code:{}",response.status()))
                    }
                },
                Err(e) => return Err(format!("Failed to refresh token. details:{}",e))
            };

            *data = TimeSensitiveData::new(refresh_token);

        }
        
        Ok(())
    }

    pub async fn get_launch_data(&mut self) -> Result<MinecraftLaunchData,String>{
        self.check_microsoft_token().await?;
        let mut authflow = MinecraftAuthorizationFlow::from(env!("MICROSOFT_CLIENT_ID"),MinecraftAuthStep::MicrosoftAuth(self.microsoft.clone()));
        let response = authflow.xbox_live_auth().await;
        if let Err(e) = response{
            return Err(e.to_string())
        }

        let response = authflow.xbox_security_auth().await;
        if let Err(e) = response{
            return Err(e.to_string())
        }

        let response = authflow.get_minecraft_token().await;
        let token = match response {
            Ok(res) => {res.clone()},
            Err(e) => { return Err(e.to_string())}
        };

        let response = authflow.check_minecraft_profile().await;
        let latest_account = match response {
            Ok(pair) => {pair},
            Err(e) => { return Err(e.to_string())}
        };
        
        self.profile = latest_account.profile.clone();
        
        Ok(MinecraftLaunchData {
            profile: latest_account.profile,
            token
        })
    }

}

pub async fn save(app_handle: &AppHandle) -> Result<(),String>{
    let usermap = app_handle.state::<MinecraftUUIDMap>();
    let config = app_handle.path_resolver().app_config_dir();
    if let Some(config_path) = config {
        let token = config_path.join("token");
        let profile = config_path.join("profile");
        let response = tokio::fs::create_dir_all(&token).await;
        if let Err(e) = response {
            return Err(format!("Failed to create user directory. details:{}",e))
        }
        for (uuid,login_data) in usermap.read().await.iter(){
            let token_file_path = token.join(format!("{}.json", uuid));
            let profile_file_path = profile.join(format!("{}.json", uuid));
            let microsoft = login_data.microsoft.read().await;
            if let Err(e) = tokio::fs::write(token_file_path, serde_json::to_string(&microsoft.deref()).expect("this should be success!")).await {
                return Err(format!("Failed to save token data. details:{}", e))
            }
            if let Err(e) = tokio::fs::write(profile_file_path, serde_json::to_string(&login_data.profile.deref()).expect("this should be success!")).await {
                return Err(format!("Failed to save profile data. details:{}", e))
            }
            
        }
    } else {
        return Err("Failed to get the config directory.".to_string())
    }

    Ok(())
}

pub async fn read(app_handle: &AppHandle) -> Result<(),String>{
    let usermap: State<MinecraftUUIDMap> = app_handle.state::<MinecraftUUIDMap>();
    let config = app_handle.path_resolver().app_config_dir();
    if let Some(config_path) = config{
        let token_path = config_path.join("users");
        let profile_path = config_path.join("profile");
        let token = tokio::fs::read_dir(token_path).await;
        if let Ok(mut files) = token {
            loop{
                let file = files.next_entry().await;
                let (uuid,microsoft)  = if let Ok(file) = file{
                    if let Some(file) = file{
                        if let Ok(metadata) = file.metadata().await{
                            if metadata.is_file() {continue}
                        }
                        let uuid = file.file_name().to_string_lossy().strip_suffix(".json").expect("this should be success!").to_string();
                        let body = tokio::fs::read_to_string(file.path()).await;
                        let mirosoft:MicrosoftAuthResponse = if let Ok(body) = body {
                            serde_json::from_str(&body).expect("this should be success!")
                        } else { 
                            println!("failed to read the file. details:{}",body.err().unwrap());
                            continue
                        };
                        (uuid,mirosoft)
                    } else {
                        break
                    }
                }
                else{
                    return Err(format!("Failed to read the user directory. details:{}",file.err().unwrap()))
                };
                
                let profile_file = profile_path.join(format!("{}.json",uuid));
                let profile_body = tokio::fs::read_to_string(profile_file).await;
                let profile:MinecraftProfile = if let Ok(profile_body) = profile_body {
                    serde_json::from_str(&profile_body).expect("this should be success!")
                } else {
                    println!("failed to read the file. details:{}",profile_body.err().unwrap());
                    continue
                };
                
                let login_data = LoginAccount {
                    microsoft: Arc::new(RwLock::new(TimeSensitiveData::new(microsoft))),
                    profile: Arc::new(profile)
                };
                
                usermap.write().await.insert(uuid, Arc::new(login_data));
            }
        }else{
            return Err("Failed to read the user directory.".to_string())
        }

    } else {
        return Err("Failed to get the config directory.".to_string())
    }
    Ok(())
}

