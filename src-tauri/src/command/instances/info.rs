use std::path::PathBuf;

use reginleif_utils::save_path::{BaseStorePoint, Load};
use reqwest::Client;
use tauri::State;

use crate::utils::{metadata::handler::PackageListHandler, result::CommandResult, settings::instances::{InstancesConfig, NLInstanceList}};

#[derive(Debug, serde::Serialize)]
pub struct InstanceListPayload{
    pub id:String,
    pub name:String,
    pub base_path:PathBuf
}

#[tauri::command]
pub async fn instance_list(
    instance_list:State<'_,NLInstanceList>
)-> CommandResult<Vec<InstanceListPayload>> {
    
    let instances = instance_list.list_cloned().await;
    let base_path = instance_list.store_point().get_base();
    
    let list = instances.iter().map(|(id,name)|{
        InstanceListPayload{
            id:id.clone(),
            name:name.clone(),
            base_path:base_path.clone()
        }
    }).collect();

    Ok(list)
}

#[derive(Debug,serde::Serialize)]
pub struct InstanceDownloadPayload{
    pub filename:String,
    pub path:String,
}

#[tauri::command]
pub async fn instance_downloads(
    id:String,
    instance_list:State<'_,NLInstanceList>,
    pkg_handler:State<'_,PackageListHandler>
)-> CommandResult<Vec<InstanceDownloadPayload>> {

    let base = instance_list.store_point();

    let config = InstancesConfig::load(&base, PathBuf::from(id).join("config.json"))?;
    let uid = config.uid;
    let version = config.version;
    let mc_version = config.mc_version;
    let url = "https://meta.prismlauncher.org/v1";
    let client = Client::new();

    let data = pkg_handler.download(client, url, &mc_version, &version, &uid).await?;
    println!("{:?}",data);

    
    let temp = vec![];
    Ok(temp)
}