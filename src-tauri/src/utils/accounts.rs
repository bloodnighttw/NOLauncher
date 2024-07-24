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
use reginleif_utils::expiring_data::Refreshable;

#[derive(Default,Debug,Serialize,Deserialize,Storage)]
#[base_on(ConfigStorePoint)] #[filepath(&["accounts.txt"])]
struct AccountMapping(String,HashMap<String,Account>);

#[derive(Debug)]
pub struct NLAccounts(RwLock<AccountMapping>,ConfigStorePoint);

impl NLAccounts {

    pub fn default(base:ConfigStorePoint) -> Self{
        Self(Default::default(),base)
    }

    pub async fn load(base:&ConfigStorePoint) -> Result<Self>{
        let data = AccountMapping::load(base)?;
        Ok(Self(data.into(),base.clone()))
    }

    pub async fn add(&self,account: Account) -> Result<()>{
        let mut writer = self.0.write().await;
        let config = &self.1;
        writer.1.insert(account.profile.id.clone(),account);
        writer.save(config)?;
        Ok(())
    }

    pub async fn switch(&self,id: &str) -> Result<()>{
        let mut writer = self.0.write().await;
        writer.0 = id.to_string();
        writer.save(&self.1)?;
        Ok(())
    }

    pub async fn logout(&self,id: &str) -> Result<()>{
        let mut writer = self.0.write().await;
        writer.1.remove(id);
        writer.save(&self.1)?;
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

    pub async fn me_latest(&mut self) -> Result<Option<Account>>{
        let config = &self.1;
        let mut writer = self.0.write().await;
        let id = writer.0.clone();
        let data = writer.1.get_mut(&id);
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
            writer.save(config)?;
        }
        data
    }

}

#[derive(Debug, Serialize, Clone)]
pub struct AccountPayload{
    pub id:String,
    pub name:String,
    pub skin:String    
}

impl From<&Account> for AccountPayload{
    fn from(value: &Account) -> Self {
        Self{
            id:value.profile.id.clone(),
            name:value.profile.name.clone(),
            skin:value.profile.skins.iter().find(|x| x.state == "ACTIVE").unwrap().url.clone()
        }
    }
}