use std::collections::HashMap;
use std::fs::{read_dir};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicI64;
use async_recursion::async_recursion;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State};
use tokio::fs::create_dir_all;
use crate::constant::{LIB_PATH, NO_SIZE_DEFAULT_SIZE};
use crate::event::instance::instance_status_update;
use crate::utils::config::{Storage, SafeNoLauncherConfig, NoLauncherConfig, Save, SavePath, Load};
use crate::utils::minecraft::instance::{get_launch_data, DownloadMutex, GameFile, InstanceConfig, LaunchData, SafeInstanceStatus, Status};
use crate::utils::minecraft::metadata::{decode_hex};
use crate::utils::minecraft::metadata::SHAType::SHA256;
use crate::utils::result::CommandResult;
use anyhow::Result;
use tauri_plugin_shell::process::{CommandChild, CommandEvent};
use tauri_plugin_shell::ShellExt;
use tokio::task::JoinSet;
use tauri::async_runtime::Receiver;

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
    dep:Option<String>
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
            .map(|x| -> SimpleInfo {
                let i = x.requires.clone();
                let dep = match i.first() {
                    None => { None }
                    Some(info) => {
                        info.equals.clone()
                    }
                };
                SimpleInfo{
                    version:x.version.clone(),
                    rtype:x.rtype.clone(),
                    dep
                }
            })
            .collect();
        vec
    }
}

#[tauri::command]
pub async fn list_versions(config: State<'_, SafeNoLauncherConfig>, app:AppHandle) -> CommandResult<MinecraftInfoResponse> {
    let lock = config;
    let mut not_up_to_date_flag = false;

    {
        let mut config = lock.write().await;
        if !&config.metadata_setting.package_list.is_vaild() {
            let res = config.metadata_setting.refresh().await;
            if res.is_err() {
                not_up_to_date_flag = true;
            }
        }
        config.save_by_app(&app)?
    }

    let config = lock.read().await;
    
    let default_path = app.path().app_cache_dir()?;
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


#[derive(Debug,Serialize,Deserialize)]
pub enum PlatformType {
    Minecraft,
    Fabric,
    Forge,
    Liteloader,
    NeoForge,
    Quilt
}

#[derive(Debug,Serialize,Deserialize)]
pub struct InstanceCreateRequest{
    pub name:String, // instance name
    pub ptype: PlatformType, // platform
    pub version:String, // minecraft version
    pub mod_version:Option<String> // mod loader version, vanilla is None
}

fn ptype2uid(ptype:PlatformType) -> String {
    match ptype {
        PlatformType::Minecraft => MINECRAFT_UID.to_string(),
        PlatformType::Fabric => FABRIC_UID.to_string(),
        PlatformType::Forge => FORGE_UID.to_string(),
        PlatformType::Liteloader => LITELOADER_UID.to_string(),
        PlatformType::NeoForge => NEOFORGE_UID.to_string(),
        PlatformType::Quilt => QUILT_UID.to_string()
    }
}

/// Get the dependency of the package recursively.
///
/// # Arguments
///
/// * `uid`: the package uid.
/// * `mc_version`: minecraft version. (e.g. 1.16.5, 1.17.1,etc.)
/// * `p_version`: platform specific version (e.g. fabric, forge, liteloader, neoforge, quilt, lwjgl, lwjgl3).
/// * `metadata_setting`: metadata setting. the variable is used to get the package details.
/// * `cached`: the cached folder.
///
/// returns: HashMap<String, String> - the key is the uid, the value is the version.
///
/// # Examples
///
/// ```
/// let uid = "net.minecraft";
/// let version = "1.16.5";
/// let p_version = None;
/// let mut config = NoLauncherConfig::default();
/// let r = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
/// r.block_on(async { config.metadata_setting.refresh().await }).unwrap();
/// let cached = env::current_dir().unwrap().join("test");
/// let res = handle_dep(uid, version, p_version, &config, &cached);
///
/// println!("{:?}", res); // print the result
///
/// ```
// version: minecraft version, p_version: version of non-minecraft platform (e.g. fabric, forge, liteloader, neoforge, quilt, lwjgl, lwjgl3.)
#[async_recursion]
async fn handle_dep(uid:&str, mc_version:&str, p_version:Option<String>, config: &NoLauncherConfig, cached:&PathBuf) -> HashMap<String,String> { // uid to version

    let data = config.metadata_setting.package_list.data.packages.get(uid).unwrap();
    let sha256 = SHA256(decode_hex(&data.sha256).unwrap());
    let details = config.metadata_setting.get_package_details(cached.clone(), uid, sha256).await.unwrap();

    let (uid, version) = match uid {
        uid @ (MINECRAFT_UID | INTERMEDIARY_UID)  => { // all platform followed the vanilla version
            (uid.to_string(), mc_version.to_string())
        }
        _uid=> {
            (_uid.to_string(),p_version.unwrap())
        }
    };

    let mut req = None;

    for i in details.versions.iter() { // linear search for version
        if i.version == version {
            req = Some(i.clone());
            break;
        }
    }

    let mut dep = match req {
        None => { HashMap::default() }
        Some(req) => {

            let mut map = HashMap::new();

            for i in req.requires.iter() {
                let equal = i.equals.clone().or(i.suggests.clone());
                let temp = handle_dep(&i.uid, mc_version, equal, config, cached).await;
                for (key,value) in temp.iter(){
                    map.insert(key.clone(),value.clone());
                }
            }
            map
        }
    };

    dep.insert(uid, version);
    dep
}

#[tauri::command]
pub async fn create_instance(
    request:InstanceCreateRequest,
    config:State<'_, SafeNoLauncherConfig>,
    app: AppHandle,
) -> CommandResult<String> {

    let uid = ptype2uid(request.ptype);
    let version = request.version;
    let p_version = request.mod_version;

    let dep ={
        let config = config.read().await;
        let cached = app.path().app_cache_dir()?;
        handle_dep(&uid, &version, p_version, &config, &cached).await
    };

    let uuid:String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .collect(); // random 6 id
    
    let instance_config = InstanceConfig{
        id:uuid.clone(),
        name:request.name.clone(),
        dep,
        top:uid
    };
    
    let instance_path = SavePath::from_data(&app,vec![&uuid]).unwrap();
    tokio::fs::create_dir_all(&instance_path).await?;
    let instance_config_path = instance_path.join("instance.json");
    instance_config.save(&instance_config_path)?;

    {
        let mut config = config.write().await;
        config.instances.push(instance_path);
        config.save_by_app(&app)?;
    }

    Ok(String::default())
}

#[derive(Serialize,Debug)]
pub struct InstanceInfo{
    pub id:String,
    pub name:String
}

#[tauri::command]
pub async  fn list_instance(
    app: AppHandle
) -> CommandResult<Vec<InstanceInfo>>{
    let mut vec = Vec::default();

    let data_path = &app.path().app_data_dir()?;

    let folders = read_dir(data_path)?;

    for i in folders {
        let path = i?.path();
        if path.is_dir(){
            let data = InstanceConfig::load(path.join("instance.json").as_ref());
            if let Ok(data) = data{
                vec.push(*data);
            }
        }
    }
    
    let vec = vec.iter().map(|x| InstanceInfo{ id: x.id.to_string(), name: x.name.to_string()}).collect();
    
    Ok(vec)
}

async fn prepare(
    id:&str,
    app: &AppHandle,
    map: &SafeInstanceStatus,
    config: &SafeNoLauncherConfig,
) -> Result<(Vec<GameFile>,LaunchData)> // return (the game file need to launch
{
    {   // we are going to prepare the file we need.
        let mut map = map.write().await;
        map.insert(id.to_string(),Status::Preparing);
    }

    instance_status_update(&id,&app).await; // trigger event update.

    let instance_config_path = SavePath::from_data(&app,vec![&id,"instance.json"])?;
    let instance_config = *InstanceConfig::load(instance_config_path.as_path())?;

    let launch_data = {   // prepare
        let metadata = &config.read().await.metadata_setting;
        get_launch_data(&metadata, &instance_config, app).await?
    };

    let lib_path = LIB_PATH.to_path(&app)?;
    create_dir_all(lib_path.clone()).await?;

    let game_files = launch_data.get_download_entities(lib_path);

    Ok((game_files,launch_data))
}

async fn download(
    id:&str,
    need_download:Vec<GameFile>,
    map:&SafeInstanceStatus,
    app:&AppHandle,
) -> CommandResult<()>
{
    
    if need_download.len() <= 0{
        return Ok(())
    }


    let ai64: Arc<AtomicI64> = AtomicI64::default().into();
    let total_size = need_download.iter()
        .map(|x| x.size.unwrap_or(NO_SIZE_DEFAULT_SIZE))
        .sum();

    let status = Status::Downloading { now: ai64.clone(), total: total_size };

    {
        let mut map = map.write().await;
        map.insert(id.to_string(), status);
    }

    instance_status_update(&id, &app).await;

    {
        let mut join_set:JoinSet<Result<()>> = JoinSet::new();

        for i in need_download{
            let move_ai64 = ai64.clone();
            let app_clone = app.clone();
            let id = id.to_string();

            join_set.spawn(async move {
                i.download_file(move_ai64,&id,&app_clone).await?;
                Ok(())
            });
        }

        while let Some(res) = join_set.join_next().await{
            if let Ok(result) = res{
                result?;
            }else{
                println!("{:?}",res);
            }
        }

        Ok(())
    }
}

async fn running(
    id:&str,
    game_files:Vec<GameFile>,
    app:&AppHandle,
    map:&SafeInstanceStatus,
    main_class:&str
) -> Result<Receiver<CommandEvent>>{

    let list = game_files.iter()
        .map(|x|x.get_fullpath().to_str().unwrap().to_string())
        .collect::<Vec<String>>()
        .join(":");// windows use ";"
    
    let shell = app.shell();
    let (output,command_child)= shell
            .command("java")
            .arg("-cp")
            .arg(list)
            .arg(main_class)
            .arg("--accessToken")
            .arg("nothing here")
            .arg("--version")
            .arg("test")
            .spawn()?;
    
    let command_child:Arc<CommandChild> = command_child.into();
    
    let status = Status::Running(command_child.clone());
    
    {
        let mut map = map.write().await;
        map.insert(id.to_string(), status);
    }
    
    instance_status_update(&id, &app).await;
    
    
    Ok(output)
}

async fn failed(
    id:&str,
    app:&AppHandle,
    details:String,
    map:&SafeInstanceStatus
){
    let status = Status::Failed{details};

    {
        let mut map = map.write().await;
        map.insert(id.to_string(), status);
    }

    instance_status_update(&id, &app).await;
}

#[tauri::command]
pub async fn launch_game(
    id:String,
    app: AppHandle,
    map: State<'_, SafeInstanceStatus>,
    config: State<'_,SafeNoLauncherConfig>,
    lock: State<'_,DownloadMutex>
) -> CommandResult<()>
{

    {
        let map = map.read().await;
        let status = map.get(&id).unwrap_or(&Status::Stopped);

        match status {
            Status::Failed{..} | Status::Stopped{ .. } => {
                // do nothing
            }
            _=>{
                return Ok(())
            }
        }
    }

    let prepare_result = prepare(&id, &app, &map, &config).await;

    let (game_files,launch_data) = match prepare_result {
        Ok((game,launch_data)) => (game,launch_data),
        Err(details) => {
            failed(&id,&app,details.to_string(),&map).await;
            return Ok(())
        }
    };

    {
        let _lock = lock.lock().await;
        let need_download:Vec<GameFile> = game_files.iter()
            .filter(|x| !x.get_fullpath().exists())
            .map(|x|x.clone())
            .collect();
        
        let download_result = download(&id, need_download, &map, &app).await;

        match download_result {
            Ok(_) => {}
            Err(details) => {
                failed(&id, &app, details.to_string(), &map).await;
                return Ok(());
            }
        }
    }

    let running_result = running(&id, game_files, &app, &map, &launch_data.main_class).await;
    
    let mut reciver = match running_result {
        Ok(child) => {
            child
        }
        Err(details) => {
            failed(&id,&app,details.to_string(),&map).await;
            return Ok(())
        }
    };

    while let Some(details) = reciver.recv().await {
        if let CommandEvent::Stdout(message) = details{
            let str:String = String::from_utf8(message).unwrap();
            println!("{str}");
        }
    }

    {   // we are going to prepare the file we need.
        let mut map = map.write().await;
        map.insert(id.to_string(),Status::Stopped);
    }

    instance_status_update(&id,&app).await;

    Ok(())
}

#[tauri::command]
pub async fn trigger_instance_update_event(
    id:String,
    app:&AppHandle
) -> CommandResult<()>{
    instance_status_update(&id,&app).await;
    Ok(())
}

#[cfg(test)]
mod test{
    use std::collections::HashMap;
    use std::env;
    use crate::command::instance::{FABRIC_UID, FORGE_UID, handle_dep};
    use crate::utils::config::NoLauncherConfig;

    fn vec2hashmap(vec:Vec<(&str,&str)>) -> HashMap<String,String> {
        let mut map = HashMap::new();
        for (key,value) in vec.iter(){
            map.insert(key.to_string(),value.to_string());
        }
        map
    }

    #[tokio::test]
    async fn test_handle_dep(){
        let uid = "net.minecraft";
        let version = "1.16.5";
        let p_version = None;
        let mut config = NoLauncherConfig::default();
        config.metadata_setting.refresh().await.unwrap();
        let cached = env::current_dir().unwrap().join("test");

        let res = handle_dep(uid, version, p_version, &config, &cached).await;
        let valid_vec = vec![
            ("net.minecraft", "1.16.5"),
            ("org.lwjgl3", "3.2.2")
        ];
        let valid_case = vec2hashmap(valid_vec);
        assert_eq!(res, valid_case);

        let res = handle_dep(FORGE_UID, "1.21", Some("51.0.16".to_string()), &config, &cached).await;
        let valid_vec = vec![
            ("net.minecraft", "1.21"),
            ("org.lwjgl3", "3.3.3"),
            ( "net.minecraftforge","51.0.16")
        ];
        let valid_case = vec2hashmap(valid_vec);
        assert_eq!(res, valid_case);


        let res = handle_dep(FABRIC_UID, "1.21", Some("0.14.0".to_string()), &config, &cached).await;
        let valid_vec = vec![
            ("net.minecraft", "1.21"),
            ("org.lwjgl3", "3.3.3"),
            ("net.fabricmc.fabric-loader","0.14.0"),
            ("net.fabricmc.intermediary", "1.21")
        ];
        let valid_case = vec2hashmap(valid_vec);
        assert_eq!(res, valid_case);

    }
}