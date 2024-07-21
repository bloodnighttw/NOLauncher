use std::ops::Deref;
use anyhow::anyhow;
use reginleif::auth::account::Account;
use reginleif::auth::minecraft::{MinecraftAuth, Profile};
use reqwest::Client;
use tauri::State;
use crate::command::auth::{AuthStep, NLAuthStep};
use crate::utils::result::CommandResult;
use crate::utils::accounts::NLAccounts;
use crate::utils::base_store::ConfigStorePoint;

#[tauri::command]
pub async fn account(
    step:State<'_,NLAuthStep>,
    accounts:State<'_,NLAccounts>,
    config:State<'_,ConfigStorePoint>
) -> CommandResult<()>{
    let mut lock = step.lock().await;

    let (msa_token,xbox_security) = match lock.clone() {
        AuthStep::Minecraft(msa_token,xbox_security) => (msa_token,xbox_security),
        _ => return Err(anyhow!("Invalid auth step").into())
    };

    let client = Client::new();

    let minecraft_auth =  MinecraftAuth::fetch(&client,xbox_security).await?;

    let profile = Profile::fetch(&client,&minecraft_auth).await?;

    let account:Account = (minecraft_auth,profile.clone(),msa_token).into();

    accounts.add(account,config.deref()).await?;

    *lock = AuthStep::Exchange; // reset auth step

    Ok(())

}