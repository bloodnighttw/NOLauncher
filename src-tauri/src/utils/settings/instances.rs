use std::path::PathBuf;

use reginleif_macro::{Load, Save};
use reginleif_utils::save_path::ExpandStorePoint;
use serde::{Deserialize, Serialize};
use crate::utils::base_store::InstanceStorePoint;

#[derive(Serialize,Deserialize,Debug,Default,Clone,PartialEq,Save,Load)]
#[base_on(InstanceStorePoint)]
pub struct InstanceConfig{
    pub id: String,
    pub name: String,
    pub mc_version: String, // minecraft version
    pub uid: String, // top level uid of instance
    pub version: String, // uid version of instance
}

impl ExpandStorePoint for InstanceConfig{
    fn get_suffix(&self) -> std::path::PathBuf {   
        let base:PathBuf = self.id.clone().into();
        base.join("config.json")
    }
}

