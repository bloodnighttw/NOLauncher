use core::panic;
use std::{ops::{Deref, DerefMut}, time::Duration};

use reginleif::metadata::client::package::PackageList;
use reginleif_macro::{NoRefresh, Storage};
use reginleif_utils::{expiring_data::{Expirable, ExpiringData}, save_path::Store};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use crate::utils::base_store::MetadataStorePoint;
use anyhow::Result;

#[derive(NoRefresh,Serialize,Deserialize,Debug,Clone)]
struct InnerTimer;

impl Expirable for InnerTimer {    
    fn get_duration(&self) -> std::time::Duration {
        Duration::from_secs(3600)
    }
}

#[derive(Serialize,Deserialize,Debug,Clone,Storage)]
#[base_on(MetadataStorePoint)] #[filepath(&["timer.json"])]
struct Timer{
    inner:ExpiringData<InnerTimer>
}


pub type NLPackageList = PackageList<MetadataStorePoint>;

/// mutex is to ensure only on command update package list at the same time
/// timer is to ensure that data need to refetch or not.
pub struct PackageListHandler(Mutex<Timer>,MetadataStorePoint);

impl From<MetadataStorePoint> for PackageListHandler{

    fn from(base:MetadataStorePoint) -> Self{
        
        let timer = match Timer::load(&base){
            Ok(config) => config,
            Err(_) => {
                Timer{
                    inner:ExpiringData::from(InnerTimer)
                }
            }
        };

        Self(Mutex::from(timer), base)

    }
}


impl PackageListHandler {
    pub async fn fetch(&self,client:Client,url:&str) -> Result<NLPackageList>{

        let mut _lock = self.0.lock().await;
        let timer = _lock.deref_mut();

        let temp = if timer.inner.is_expired(){
            timer.inner = ExpiringData::from(InnerTimer); // reset timer
            timer.save(&self.1)?; 
            PackageList::refresh(&self.1,client,url).await?
        }else{
            timer.save(&self.1)?;
            PackageList::fetch(&self.1,client,url).await?
        };

        Ok(temp)
    }

    pub async fn refresh(&self,client:Client,url:&str) -> Result<NLPackageList>{
        let mut _lock = self.0.lock().await;
        let timer = _lock.deref_mut();

        timer.inner = ExpiringData::from(InnerTimer); // reset timer

        PackageList::refresh(&self.1,client,url).await
    }

}
