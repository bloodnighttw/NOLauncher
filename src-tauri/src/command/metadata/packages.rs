use std::{collections::HashMap, ops::Deref};

use reqwest::Client;
use serde::Serialize;
use tauri::State;

use crate::utils::{base_store::MetadataStorePoint, metadata::packages::PackageListHandler, result::CommandResult};

#[derive(Debug,Serialize)]
pub struct VersionInfo{
    
    #[serde(rename = "type")]
    pub type_:String,
    pub version:String,
    pub require:Option<String> // for mod loader known what version the mod is depend
}

#[tauri::command]
pub async fn pkg_info(
    pkg_handler:State<'_,PackageListHandler>,
    base_on:State<'_,MetadataStorePoint>,
    uid:String
) -> CommandResult<Vec<VersionInfo>>{ // name -> uid

    let client = Client::new();
    let url =  "https://meta.prismlauncher.org/v1";
    let data = pkg_handler.fetch(client.clone(),url).await?;

    let temp = data.iter().find(|x| x.uid == uid).ok_or(anyhow::anyhow!("Package not found"))?;
    let details = temp.get_details(base_on.deref(), client, url).await?;

    let vec = details.iter().map(|x|{

        let require = x.requires.iter().find(|x| {
            matches!(x.uid.as_str(), "net.minecraft"|"net.fabricmc.intermediary")   
        }).and_then(|x|x.equals.clone());

        VersionInfo{
            type_:x.rtype.clone().unwrap_or("release".to_string()),
            version:x.version.clone(),
            require
        }
        
    }).collect();

    Ok(vec)
}