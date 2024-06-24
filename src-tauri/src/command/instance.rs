use std::path::PathBuf;
use serde::Serialize;
use tauri::{AppHandle, Manager, State};
use crate::utils::config::{LauncherConfig, NoLauncherConfig};
use crate::utils::minecraft::metadata::{decode_hex};
use crate::utils::minecraft::metadata::SHAType::SHA256;

const MINECRAFT_UID:&str = "net.minecraft";
const FABRIC_UID:&str = "net.fabricmc.fabric-loader";
const INTERMEDIARY_UID:&str = "net.fabricmc.intermediary";
const FORGE_UID:&str = "net.minecraftforge";
const LITELOADER_UID:&str = "com.mumfrey.liteloader";
const NEOFORGE_UID:&str = "net.neoforged";
const QUILT_UID:&str = "org.quiltmc.quilt-loader";


#[derive(Debug,Serialize)]
pub struct SimpleInfo{
    version:String,
    rtype:Option<String>,
}

#[derive(Debug,Serialize)]
pub struct MinecraftInfoResponse{
    pub up_to_date:bool,
    pub minecraft:Vec<SimpleInfo>,
    pub fabric_loader:Vec<SimpleInfo>,
    pub intermediary:Vec<SimpleInfo>,
    pub forge:Vec<SimpleInfo>,
    pub liteloader:Vec<SimpleInfo>,
    pub neoforge:Vec<SimpleInfo>,
    pub quilt:Vec<SimpleInfo>
}

async fn fetch_uid(
    config:&NoLauncherConfig,
    default_path:&PathBuf,
    uid:&str
) -> Vec<SimpleInfo>{
    let package = &config.metadata_setting.package_list.data.packages.get(uid);
    if package.is_none(){
        Vec::default()
    }else{
        let sha256 = SHA256(decode_hex(&package.unwrap().sha256).unwrap());


        let version_list = &config.metadata_setting.get_package_details(default_path.clone(), uid, sha256).await.unwrap().versions;
        let vec:Vec<SimpleInfo> = version_list.iter()
            .map(|x| SimpleInfo{
                version:x.version.clone(),
                rtype:x.rtype.clone()
            })
            .collect();
        vec
    }
}

#[tauri::command]
pub async fn list_versions(config: State<'_, LauncherConfig>, app:AppHandle) -> Result<MinecraftInfoResponse, String> {
    let lock = config;
    let mut config = lock.write().await;

    let mut not_up_to_date_flag = false;

    if !&config.metadata_setting.package_list.is_vaild() {
        let res = config.metadata_setting.refresh().await;
        if res.is_err() {
            not_up_to_date_flag = true;
        }
    }

    let config = lock.read().await;
    
    let default_path = app.path().app_data_dir().unwrap();
    let minecraft = fetch_uid(&config,&default_path,MINECRAFT_UID).await;
    let fabric_loader = fetch_uid(&config,&default_path,FABRIC_UID).await;
    let intermediary = fetch_uid(&config, &default_path, INTERMEDIARY_UID).await;
    let forge = fetch_uid(&config, &default_path, FORGE_UID).await;
    let liteloader = fetch_uid(&config, &default_path, LITELOADER_UID).await;
    let neoforge = fetch_uid(&config, &default_path, NEOFORGE_UID).await;
    let quilt = fetch_uid(&config, &default_path, QUILT_UID).await;
    
    Ok(MinecraftInfoResponse{
        up_to_date:!not_up_to_date_flag,
        minecraft,
        fabric_loader,
        intermediary,
        forge,
        liteloader,
        neoforge,
        quilt
    })
}