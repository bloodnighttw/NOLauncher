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
use tokio::io::AsyncWriteExt;
use tokio::sync::{Mutex, RwLock};
use nolauncher_derive::{Load, Save};

const NO_SIZE_DEFAULT_SIZE:i64 = 100000;

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
    pub fn get_download_entities(&self,lib_path:PathBuf) -> Vec<GameFile>{
        let mut downloads = vec![];

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
        downloads.into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
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

}

pub async fn download_file(download:&GameFile, progress:Arc<AtomicI64>, instance_id:&str) -> Result<()>{

    create_dir_all(&download.path)?; // create path
    let fullpath = download.get_fullpath();
    let mut file = tokio::fs::File::create(fullpath).await?;
    let mut stream = reqwest::get(&download.url)
        .await?
        .bytes_stream();

    let skip_download_size_log = match download.size {
        None => {true}
        Some(_) => {false}
    };

    while let Some(chunk_result) = stream.next().await{
        let chunk = chunk_result?;
        if !skip_download_size_log {
            progress.fetch_add(chunk.len() as i64, Relaxed);
        }
        file.write_all(&chunk).await?;
    }

    if skip_download_size_log{
        progress.fetch_add(NO_SIZE_DEFAULT_SIZE, Relaxed);
    }

    Ok(())
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
///
/// println!("{:?}", res); // print the result
///
/// ```
pub async fn check_instance(config: &MetadataSetting, instance_config: &InstanceConfig, cached_path:PathBuf) -> Result<LaunchData> {
    let pkg = &instance_config.dep;

    let mut vec = Vec::default();
    let mut main_class:Option<String> = None;
    for (uid,version) in pkg.iter(){
        let pkg_info = config.package_list.data.packages.get(uid).unwrap();
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
    print!("{:?}",vec);
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
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Status{
    Running,
    Preparing,
    Checking{now:Arc<AtomicI64>,total:i64}, // (the file amount has been checked, total)
    Downloading{now:Arc<AtomicI64>,total:i64}, // (the amount of data has been download, total)
    Stopped,
    Failed
}

pub type DownloadMutex = Mutex<()>; // only one at most instance can download file at the same time.
pub type InstanceStatus = RwLock<HashMap<String,Status>>;  // to store the status of instance


#[cfg(test)]
mod test{
    use std::collections::{HashMap};
    use std::path::PathBuf;
    use std::sync::Arc;
    use std::sync::atomic::AtomicI64;
    use std::time::Duration;
    use tokio::task::{JoinSet};
    use crate::utils::minecraft::instance::{check_instance, download_file, GameFile, InstanceConfig, NO_SIZE_DEFAULT_SIZE};
    use crate::utils::minecraft::metadata::MetadataSetting;
    use anyhow::Result;
    use tokio::process::Command;
    use tokio::time;

    fn vec2hashmap(vec:Vec<(&str, &str)>) -> HashMap<String,String> {
        let mut map = HashMap::new();
        for (key,value) in vec.iter(){
            map.insert(key.to_string(),value.to_string());
        }
        map
    }

    #[tokio::test]
    #[cfg(target_arch = "x86_64")]
    #[cfg(target_os = "linux")]
    async fn test(){

        let mut metadata = MetadataSetting::default();
        metadata.refresh().await.unwrap();

        let valid_vec = vec![
            ("net.minecraft", "1.16.5"),
            ("org.lwjgl3", "3.2.2"),
            ("net.fabricmc.intermediary","1.16.5"),
            ("net.fabricmc.fabric-loader","0.15.1")
        ];
        
        let instance = InstanceConfig{
            id: "123456".to_string(),
            name: "hello".to_string(),
            dep: vec2hashmap(valid_vec),
            top: "net.fabricmc.fabric-loader".to_string(),
        };
        
        let path:PathBuf = "./test".into();
        let cached = path.join("cached");
        let lib_path = path.join("libraries");
        let launch_data = check_instance(&metadata, &instance, cached).await.unwrap();
        
        let mut tasks: JoinSet<Result<()>> = JoinSet::new();
        let game_file = launch_data.get_download_entities(lib_path.clone());
        
        tokio::fs::create_dir_all(lib_path.clone()).await.unwrap();

        let ai64:Arc<AtomicI64>= AtomicI64::new(0).into();

        let downloads:Vec<GameFile> = game_file.iter()
            .filter(|x| !x.get_fullpath().exists()) // find the file not exists on pc
            .map(|x|x.clone()) // bring borrow into own
            .collect();

        let _total_size:i64 = downloads.iter()
            .map(|x| x.size.unwrap_or(NO_SIZE_DEFAULT_SIZE))
            .sum(); // the total file size need to download.

        for i in downloads{
            let move_value = ai64.clone();
            tasks.spawn(async move {
                download_file(&i,move_value,"123456").await?;
                Ok(())
            });
        }


        println!("Started {} tasks. Waiting...", tasks.len());

        while let Some(res) = tasks.join_next().await{
            if let Ok(Ok(res)) = res{
            }else{
                println!("{:?}",res);
            }
        }

        let list = game_file.iter()
            .map(|x|x.get_fullpath().to_str().unwrap().to_string())
            .collect::<Vec<String>>()
            .join(":");
        
        let time_ = Duration::from_secs(30);

        let _:Result<(),()> = time::timeout(time_,async {

            let mut child = Command::new("java")
                .arg("-cp")
                .arg(list.clone())
                .arg(launch_data.main_class)
                .arg("--accessToken")
                .arg("nothing here")
                .arg("--version")
                .arg("test")
                .spawn()
                .expect("this should work");

            let _ = child.wait().await.unwrap();
            Err(())
        }).await.unwrap_or(Ok(()));// timeout error is allowed for testing!
        
        println!("{:?}",list);
        tokio::fs::remove_dir_all(path).await.unwrap();
    }
}