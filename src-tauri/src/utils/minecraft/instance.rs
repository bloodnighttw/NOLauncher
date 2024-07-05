use std::collections::{HashMap, HashSet};
use std::fs::create_dir_all;
use std::path::{PathBuf};
use std::sync::{Arc};
use std::sync::atomic::{AtomicI64};
use std::sync::atomic::Ordering::Relaxed;
use serde::{Deserialize, Serialize};
use crate::utils::minecraft::metadata::{decode_hex, equal_my_platform, Library, MetadataSetting, rules_analyzer, string2platform};
use crate::utils::minecraft::metadata::Library::Common;
use crate::utils::minecraft::metadata::SHAType::SHA256;
use anyhow::Result;
use futures_util::StreamExt;
use tauri::AppHandle;
use tauri_plugin_shell::process::CommandChild;
use tokio::io::AsyncWriteExt;
use tokio::sync::{Mutex, RwLock};
use nolauncher_derive::{Load, Save};
use crate::constant::{CACHED_DEFAULT, LIB_PATH, NO_SIZE_DEFAULT_SIZE};
use crate::event::instance::{instance_status_update, progress_status_update};


#[derive(Serialize,Deserialize,Debug,Default,Save,Load)]
pub struct InstanceConfig{
    #[serde(default)]
    pub id:String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub dep: HashMap<String,String>, // key: uid, value: version,
    #[serde(default)]
    pub top: String // top dep uid
}

#[derive(Debug,PartialEq,Clone,Hash,Eq)]
pub enum LibType{
    Lib,
    Client,
    Installer // for forge, neoforge only.
}

impl Default for LibType{
    fn default() -> Self {
        LibType::Lib
    }
}

///TODO: Add Client and installer path. 
#[derive(Default,Debug,PartialEq)]
pub struct LaunchData{
    pub main_class: String,
    pub dep: Vec<Library>,
}

impl LaunchData {
    pub fn get_game_file(&self, app:&AppHandle) -> Result<Vec<GameFile>>{
        let mut downloads = vec![];
        
        let lib_path = LIB_PATH.to_path(&app)?;
        create_dir_all(&lib_path)?;

        for i in &self.dep{
            downloads.append(
                &mut GameFile::from(i.clone(), lib_path.clone())
            )
        }// correct


        // to remove duplicates
        // some data look like this, which will cause duplicates download:
        // {
        //     "downloads": {
        //         "artifact": {
        //             "sha1": "f378f889797edd7df8d32272c06ca80a1b6b0f58",
        //             "size": 13164,
        //             "url": "https://libraries.minecraft.net/com/mojang/text2speech/1.11.3/text2speech-1.11.3.jar"
        //         }
        //     },
        //     "name": "com.mojang:text2speech:1.11.3"
        // },
        // {
        //     "downloads": {
        //         "artifact": {
        //             "sha1": "f378f889797edd7df8d32272c06ca80a1b6b0f58",
        //             "size": 13164,
        //             "url": "https://libraries.minecraft.net/com/mojang/text2speech/1.11.3/text2speech-1.11.3.jar"
        //         },
        //         "classifiers": {
        //             "natives-linux": {
        //                 "sha1": "ac641755a2a841d1fca9e660194f42523ee5cfe0",
        //                 "size": 7833,
        //                 "url": "https://libraries.minecraft.net/com/mojang/text2speech/1.11.3/text2speech-1.11.3-natives-linux.jar"
        //             },
        //             "natives-windows": {
        //                 "sha1": "c0b242c0091be5acbf303263c7eeeaedd70544c7",
        //                 "size": 81379,
        //                 "url": "https://libraries.minecraft.net/com/mojang/text2speech/1.11.3/text2speech-1.11.3-natives-windows.jar"
        //             }
        //         }
        //     },
        //         "extract": {
        //         "exclude": [
        //         "META-INF/"
        //         ]
        //         },
        //         "name": "com.mojang:text2speech:1.11.3",
        //         "natives": {
        //         "linux": "natives-linux",
        //         "windows": "natives-windows"
        //     }
        // }
        let temp =downloads.into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        
        Ok(temp)
    }
}

#[derive(Debug,Clone,PartialEq,Hash,Eq)]
pub struct GameFile {
    pub path:PathBuf,
    pub filename:String,
    pub url:String,
    pub lib_type: LibType,
    pub size:Option<i64>
}

impl GameFile {

    /// The forge MCP package has version name XXX@ZIP, we need to store under version name XXX.
    ///
    /// # Arguments 
    ///
    /// * `s`: the string want to convert. 
    ///
    /// returns: String
    fn remove_zip(s:&str) -> String{
        match s.strip_suffix("@zip") {
            None => {s.to_string()}
            Some(s) => {s.to_string()}
        }
    }

    pub fn from(lib:Library,mut path:PathBuf) -> Vec<GameFile>{
        match lib {
            Common(lib) => {
                let mut spilt = lib.name.splitn(4,":");
                let orgs = spilt.next().unwrap();
                let pkg = spilt.next().unwrap();
                let version = Self::remove_zip(spilt.next().unwrap());// mcp version path don't have @zip
                let lib_type_raw = spilt.next(); // for client or installer type
                
                let lib_type = match lib_type_raw {
                    Some("installer") => {LibType::Installer}
                    Some("client") => {LibType::Client}
                    _ => {LibType::Lib}
                };

                let j = orgs.split('.'); // to path
                for x in j {
                    path = path.join(x);
                }

                path = path.join(pkg);
                path = path.join(version);

                let mut vec = Vec::default();

                if let Some(lib) = lib.downloads.clone().artifact{

                    let filename = lib.url.rsplit_once("/").unwrap().1.to_string();

                    vec.push(
                        GameFile {
                            path:path.clone(),
                            filename,
                            url: lib.url,
                            lib_type:lib_type.clone(),
                            size:Some(lib.size)
                        }
                    )
                }

                for (platform,v) in lib.downloads.clone().classifiers{
                    let platform = string2platform(&platform);
                    if equal_my_platform(&Some(platform)){
                        let filename = v.url.rsplit_once('/').unwrap().1.to_string();

                        vec.push(
                            GameFile {
                                path:path.clone(),
                                filename,
                                url: v.url,
                                lib_type:lib_type.clone(),
                                size:Some(v.size)
                            }
                        )
                    }
                }

                vec
            }
            Library::Maven(maven) => {
                let mut spilt = maven.name.splitn(3,':');
                let orgs = spilt.next().unwrap();
                let pkg = spilt.next().unwrap();
                let version = spilt.next().unwrap();

                let filename = format!("{}-{}.jar",pkg,version);

                let j = orgs.split('.'); // to path
                for x in j {
                    path = path.join(x);
                }

                path = path.join(pkg);
                path = path.join(version);

                let url = format!("{}/{}/{}/{}/{}",maven.url,orgs.replace('.',"/"),pkg,version,filename);

                vec![
                    GameFile {
                        path,
                        filename,
                        url,
                        lib_type: LibType::Lib,
                        size:None
                    }
                ]
            }
        }
    }
    
    pub fn get_fullpath(&self) -> PathBuf{
        self.path.join(&self.filename)
    }
    
    pub async fn download_file(&self, progress:Arc<AtomicI64>, total:i64, id:&str, app:&AppHandle) -> Result<()>{

        create_dir_all(&self.path)?; // create path
        let fullpath = self.get_fullpath();
        let mut file = tokio::fs::File::create(fullpath).await?;
        let mut stream = reqwest::get(&self.url)
            .await?
            .bytes_stream();

        let skip_download_size_log = match &self.size {
            None => {true}
            Some(_) => {false}
        };

        while let Some(chunk_result) = stream.next().await{
            let chunk = chunk_result?;
            if !skip_download_size_log {
                progress.fetch_add(chunk.len() as i64, Relaxed);
                progress_status_update(progress.clone(),total,&app,id).await;
            }
            file.write_all(&chunk).await?;
        }
        
        if skip_download_size_log{
            progress.fetch_add(NO_SIZE_DEFAULT_SIZE, Relaxed);
            progress_status_update(progress.clone(),total,&app,id).await;
        }

        Ok(())
    }
}


pub async fn get_launch_data(config: &MetadataSetting, instance_config: &InstanceConfig,app:&AppHandle) -> Result<LaunchData> {
    let pkg = &instance_config.dep;
    let cached_path = CACHED_DEFAULT.to_path(app)?;

    let mut vec = Vec::default();
    let mut main_class:Option<String> = None;
    for (uid,version) in pkg.iter(){
        let pkg_info = config
            .package_list
            .data.packages
            .get(uid).unwrap();
        let sha256 = SHA256(decode_hex(&pkg_info.sha256).unwrap());

        let pkg_details = config
            .get_package_details(cached_path.clone(),uid,sha256)
            .await?;

        let version_info = pkg_details.versions
            .iter()
            .find(|&x| x.version == *version)
            .unwrap();

        let sha256 = SHA256(decode_hex(&version_info.sha256)?);
        let version_details = config
            .get_version_details(cached_path.clone(),uid,version,sha256)
            .await?;

        'it: for i in version_details.libraries.iter(){
            match i {
                Common(a) => {
                    if !rules_analyzer(a.rules.clone()){
                        continue 'it
                    }
                }
                Library::Maven(_) => {}
            }

            vec.push(i.clone())
        }

        'it2: for i in version_details.maven_files.iter(){
            match i {
                Common(a) => {
                    if !rules_analyzer(a.rules.clone()){
                        continue 'it2
                    }
                }
                Library::Maven(_) => {}
            }

            vec.push(i.clone())
        }

        if uid == &instance_config.top{
            main_class = version_details.main_class.clone();
        }

        if let Some(client) = version_details.main_jar{
            vec.push(Common(client.clone()));
        }
    }

    Ok(LaunchData{
        main_class:main_class.unwrap(),
        dep:vec
    })
}


/// the status of starting game
/// 1. Stopped -> the game is not running
/// 2. Preparing -> fetching metadata and get launch information from it.
/// 3. Downloading -> Downloading the game file instance need.
/// 4. Checking -> Checking the game file is valid!
/// 5. Running -> the game is running.
/// 6. Failed -> the game start failed!
#[derive(Clone, Serialize)]
#[serde(tag = "type")]
pub enum Status{
    Running(#[serde(skip)] Arc<CommandChild>),
    Preparing,
    Checking{now:Arc<AtomicI64>,total:i64}, // (the file amount has been checked, total)
    Downloading{now:Arc<AtomicI64>,total:i64}, // (the amount of data has been download, total)
    Stopped,
    Failed{details:String}
}

pub type DownloadMutex = Mutex<()>; // only one at most instance can download file at the same time.
pub type SafeInstanceStatus = RwLock<HashMap<String,Status>>;  // to store the status of instance