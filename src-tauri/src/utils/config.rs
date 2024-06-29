use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::utils::minecraft::metadata::MetadataSetting;
use anyhow::Result;
use tauri::{AppHandle, Manager};
use nolauncher_derive::{Storage, Load, Save};
use crate::constant::NOLAUNCHER_CONFIG_FILE;


#[derive(Deserialize, Serialize, Debug, Clone, Default, Save, Load, Storage)]
#[save_path(NOLAUNCHER_CONFIG_FILE)]
pub struct NoLauncherConfig {
    #[serde(default)]
    pub activate_user_uuid: Option<String>,
    #[serde(default)]
    pub metadata_setting: MetadataSetting,
    #[serde(default)]
    pub instances:Vec<PathBuf>
}

pub type LauncherConfig = Arc<RwLock<NoLauncherConfig>>;

pub trait Save: Serialize{
    fn save(&self,path:&Path) -> Result<()>;
}

pub trait Load<'a> : Deserialize<'a>{
    fn load(path:&Path) -> Result<Box<Self>>;
}

#[derive(Debug)]
pub enum SavePath {
    Cache(&'static [&'static str]),
    Data(&'static [&'static str]),
    Config(&'static [&'static str]),
    Log(&'static [&'static str])
}

impl SavePath {
    pub fn to_path(&self,app:&AppHandle) -> Result<PathBuf>{
        match self {
            SavePath::Cache(expand) => {
                let mut j = app.path().app_cache_dir()?;
                for i in expand.iter(){
                    j = j.join(i);
                }
                Ok(j)
            }
            SavePath::Data(expand) => {
                let mut j = app.path().app_data_dir()?;
                for i in expand.iter(){
                    j = j.join(i);
                }
                Ok(j)
            }
            SavePath::Config(expand) => {
                let mut j = app.path().app_config_dir()?;
                for i in expand.iter(){
                    j = j.join(i);
                }
                Ok(j)
            }
            SavePath::Log(expand) => {
                let mut j = app.path().app_log_dir()?;
                for i in expand.iter(){
                    j = j.join(i);
                }
                Ok(j)
            }
        }
    }
}


pub trait Storage<'a>: Save + Load<'a> {
    fn save_by_app(&self, app:&AppHandle) -> Result<()>;
    fn load_by_app(app:&AppHandle) -> Result<()>;
}
