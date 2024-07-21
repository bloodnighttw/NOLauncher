use anyhow::anyhow;
use reginleif::auth::xbox::{XboxLiveToken, XboxSecurityToken};
use reqwest::Client;
use tauri::State;
use crate::command::auth::{AuthStep, NLAuthStep};
use crate::utils::result::CommandResult;

#[tauri::command]
pub async fn xbox_live(
    step:State<'_,NLAuthStep>,
) -> CommandResult<()>{
    let mut lock = step.lock().await;

    let msa_token = match lock.clone() {
        AuthStep::XboxLive(token) => token,
        _ => return Err(anyhow!("Invalid auth step").into())
    };

    let client = Client::new();

    let xbox_live = XboxLiveToken::fetch(&client,&msa_token.data.access_token).await?;

    *lock = AuthStep::XboxSecurity(msa_token,xbox_live);

    Ok(())
}

#[tauri::command]
pub async fn xbox_security(
    step:State<'_,NLAuthStep>,
) -> CommandResult<()>{
    let mut lock = step.lock().await;

    let (msa_token,xbox_live) = match lock.clone() {
        AuthStep::XboxSecurity(msa_token,xbox_live) => (msa_token,xbox_live),
        _ => return Err(anyhow!("Invalid auth step").into())
    };

    let client = Client::new();

    let xbox_security = XboxSecurityToken::fetch(&client,xbox_live).await?;

    *lock = AuthStep::Minecraft(msa_token,xbox_security);

    Ok(())
}