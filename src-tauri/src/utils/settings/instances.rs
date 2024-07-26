use std::path::PathBuf;

use log::error;
use reginleif_macro::{Load, Save, Storage};
use reginleif_utils::save_path::{BaseStorePoint, ExpandStorePoint, Load, Store};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use crate::utils::base_store::InstanceStorePoint;

#[derive(Serialize,Deserialize,Debug,Default,Clone,PartialEq,Save,Load)]
#[base_on(InstanceStorePoint)]
pub struct InstancesConfig{
    pub id: String,
    pub name: String,
    pub mc_version: String, // minecraft version
    pub uid: String, // top level uid of instance
    pub version: String, // uid version of instance
}

impl ExpandStorePoint for InstancesConfig{
    fn get_suffix(&self) -> std::path::PathBuf {   
        let base:PathBuf = self.id.clone().into();
        base.join("config.json")
    }
}

#[derive(Serialize,Deserialize,Debug,Default,Clone,PartialEq,Storage)]
#[base_on(InstanceStorePoint)] #[filepath(&["instances.json"])]
struct InstanceList{
    pub instances:Vec<String>,
}

pub struct NLInstanceList(RwLock<InstanceList>,InstanceStorePoint);

impl NLInstanceList{
    pub fn new(base:InstanceStorePoint) -> Self{
        let data = InstanceList::load(&base).unwrap_or_default();
        Self(RwLock::new(data),base)
    }

    pub async fn list_cloned(&self) -> Vec<(String,String)>{ //(id, name)
        let reader = self.0.read().await;
        let mut vec = vec![];
        for i in reader.instances.iter(){
            let config = InstancesConfig::load(&self.1,PathBuf::from(i.as_str()).join("config.json")).unwrap_or_default();
            vec.push((i.clone(),config.name.clone()));
        }

        vec       
    }

    pub async fn add(&self, id:String)->anyhow::Result<()>{
        let mut writer = self.0.write().await;
        writer.instances.push(id);
        writer.save(&self.1)
    }

    pub async fn remove(&self, id:String)->anyhow::Result<()>{
        let mut writer = self.0.write().await;
        writer.instances.retain(|x| x != &id);
        tokio::fs::remove_dir_all(self.1.get_base().join(id)).await.unwrap_or_else(|x| error!("Failed to remove instance directory: {}",x));
        writer.save(&self.1)
    }

    pub fn store_point(&self) -> InstanceStorePoint {
        self.1.clone()
    }

}