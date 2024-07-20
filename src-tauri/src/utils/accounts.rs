#![allow(dead_code)]

use crate::utils::base_store::ConfigStorePoint;
use std::collections::HashMap;
use reginleif::auth::account::Account;
use reginleif_macro::Storage;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use crate::constant::token::MICROSOFT_CLIENT_ID;
use anyhow::Result;
use reginleif_utils::save_path::Store;
use reginleif_utils::expiring_data::{Refreshable};
use tauri::{App, Manager, Runtime};
use crate::utils::module::BuilderWrapper;

#[derive(Default,Debug,Serialize,Deserialize,Storage)]
#[base_on(ConfigStorePoint)] #[filepath(&["accounts.txt"])]
struct AccountMapping(String,HashMap<String,Account>);

#[derive(Debug,Default)]
pub struct NLAccounts(RwLock<AccountMapping>);

impl NLAccounts {

    pub async fn load(base:&ConfigStorePoint) -> Result<Self>{
        let data = AccountMapping::load(base)?;
        Ok(Self(data.into()))
    }

    pub async fn add(&self,account: Account,path:&ConfigStorePoint) -> Result<()>{
        let mut writer = self.0.write().await;
        writer.1.insert(account.profile.id.clone(),account);
        writer.save(&path)?;
        Ok(())
    }

    pub async fn list(&self) -> Vec<Account>{
        let reader = self.0.read().await;
        reader.1.values().cloned().collect()
    }

    pub async fn get(&self,id: &str) -> Option<Account>{
        let reader = self.0.read().await;
        reader.1.get(id).cloned()
    }

    pub async fn me(&self) -> Option<Account>{
        let reader = self.0.read().await;
        let id = &reader.0;
        reader.1.get(id).cloned()
    }

    pub async fn get_latest(&mut self,id:&str,config:ConfigStorePoint) -> Result<Option<Account>>{
        let mut writer = self.0.write().await;
        let data = writer.1.get_mut(id);
        let mut changed = false;
        let data = match data {
            Some(account) => {
                if account.msa.is_expired(){
                    account.refresh(&MICROSOFT_CLIENT_ID.to_string()).await?;
                    changed = true;
                }
                let account = account.clone();
                Ok(Some(account))
            }
            None => Ok(None)
        };
        if changed{
            writer.save(&config)?;
        }
        data
    }

}


pub fn init<R>(wrapper: BuilderWrapper<R>) -> BuilderWrapper<R>
where
    R:Runtime{
    wrapper
        .setup(|app:&&mut App<R>|{
            // to init the device code data in the app
            tauri::async_runtime::block_on(async {    // added this line
                let config_path = ConfigStorePoint::try_from(app).unwrap();
                let accounts = NLAccounts::load(&config_path).await.unwrap_or(NLAccounts::default());
                app.manage(accounts);
                app.manage(config_path);
                Ok::<(), anyhow::Error>(())
            }).expect("WTF");
            Ok(())
        })
}