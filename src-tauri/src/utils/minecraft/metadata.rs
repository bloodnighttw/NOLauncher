use std::collections::HashMap;
use std::io::ErrorKind;
use std::io::ErrorKind::NotFound;
use std::num::ParseIntError;
use std::path::PathBuf;
use std::time::Duration;
use serde::{de, Deserialize, Deserializer, Serialize};
use sha1::{Digest as d1,Sha1};
use sha2::{Digest, Sha256};
use thiserror::Error;
use crate::utils::data::{TimeSensitiveData, TimeSensitiveTrait};
use anyhow::Result;
use serde_json::Value;
use tauri::AppHandle;
use nolauncher_derive::Load;
use crate::constant::ASSET_INDEX_ROOT;
use crate::utils::config::Load;

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq,Default)]
#[serde(rename_all = "camelCase")]
pub struct PackageList{
    pub format_version:i32,
    #[serde(deserialize_with = "package_vec_to_map" , serialize_with = "map_to_package_vec")]
    pub packages:HashMap<String,PackageInfo>
}

impl TimeSensitiveTrait for PackageList{
    fn get_duration(&self) -> Duration {
        Duration::from_secs(3600) // 1 hour
    }
}

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PackageInfo{
    name:String,
    pub(crate) sha256:String,
    uid:String
}

fn package_vec_to_map<'de, D: Deserializer<'de>>(deserializer: D) -> Result<HashMap<String,PackageInfo>, D::Error> {
    let vec = Vec::<PackageInfo>::deserialize(deserializer)?;
    let mut map = HashMap::new();
    for i in vec{
        map.insert(i.uid.clone(),i);
    }
    Ok(map)
}

fn map_to_package_vec<S>(map: &HashMap<String,PackageInfo>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let vec: Vec<PackageInfo> = map.values().cloned().collect();
    vec.serialize(serializer)
}


/// This struct is used to store the required or conflict package information.
#[derive(Debug,Clone,Deserialize,Serialize,PartialEq)]
pub struct DependencyPackage {
    pub suggests:Option<String>,
    pub equals:Option<String>,
    pub uid: String
}

/// For package version info, like: minecraft "1.8.9", fabric-loader "0.15.1",etc.
/// This struct is used to store the version info of a package, but we don't store
/// the package details, like dependencies, libraries, main class, etc.
#[derive(Debug,Clone,Deserialize,PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VersionInfo{
    pub recommended:bool,
    pub release_time:String,
    pub sha256:String,
    #[serde(rename="type")]
    pub rtype:Option<String>,
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub requires:Vec<DependencyPackage>,
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    conflicts:Vec<DependencyPackage>,
    pub version:String,
    volatile: Option<bool>
}

/// For package details, like: minecraft, fabric-loader, etc.
/// This struct is used to store the package details, like the name, uid, versions, etc.
#[derive(Debug,Clone,Deserialize,PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PackageDetails {
    format_version:i32,
    name:String,
    uid:String,
    pub(crate) versions:Vec<VersionInfo>
}

/// This enum list all supported platforms.
#[derive(Debug,Clone,Deserialize,PartialEq)]
pub enum Platform{
    #[serde(rename = "windows")]
    Windows,
    #[serde(rename = "windows-arm64")]
    WindowsArm64,
    #[serde(rename = "linux")]
    Linux,
    #[serde(rename = "linux-arm32")]
    LinuxArm32,
    #[serde(rename = "linux-arm64")]
    LinuxArm64,
    #[serde(rename = "osx-arm64")]
    MacOsArm64,
    Unknown
}


/// to check if you are on argument platform.
///
/// # Arguments
///
/// * `platform`: the platform you want to check, always return true when None pass to it.
///
/// returns: bool
///
/// # Examples
///
/// ```
///
/// ```
pub fn equal_my_platform(platform:&Option<Platform>) -> bool{

    if let Some(platform) = platform{
        match platform {
            Platform::Windows => {
                cfg!(target_arch = "x86") || cfg!(target_arch = "x86_64") && cfg!(target_os = "windows")
            }
            Platform::WindowsArm64 => {
                cfg!(target_arch = "aarch64") && cfg!(target_os = "windows")
            }
            Platform::Linux => {
                cfg!(target_arch = "x86") || cfg!(target_arch = "x86_64") && cfg!(target_os = "linux")
            }
            Platform::LinuxArm32 => {
                cfg!(target_arch = "arm") && cfg!(target_os = "linux")
            }
            Platform::LinuxArm64 => {
                cfg!(target_arch = "aarch64") && cfg!(target_os = "linux")
            }
            Platform::MacOsArm64 => {
                cfg!(target_arch = "aarch64") && cfg!(target_os = "macos")
            }
            Platform::Unknown => {
                false // we don't support x86 macos
            }
        }
    } else {
        true
    }
}

pub fn string2platform(classifier:&str) -> Platform{
    match classifier {
        "natives-linux" => Platform::Linux,
        "natives-windows" => Platform::Windows,
        "natives-linux-arm32" => Platform::LinuxArm32,
        "natives-linux-arm64" => Platform::LinuxArm64,
        "natives-osx-arm64" => Platform::MacOsArm64,
        _ => Platform::Unknown // we don't support x86 macos
    }

}

/// This struct is used to store the rule of a library, which contain the information about
/// the package is need to install on the specific platform or not.
#[derive(Debug,Clone,Deserialize,PartialEq)]
pub struct Rule{
    action:Action,
    #[serde(deserialize_with = "os_processing", default)]
    os:Option<Platform>
}

/// Allow mean this rule is allow on the rule's platform, disallow mean this rule is disallow on the rule's platform.
#[derive(Debug,Clone,Deserialize,PartialEq)]
pub enum Action{
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "disallow")]
    Disallow
}

/// This function is used to deserialize the os field in Rule struct.
fn os_processing<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Platform>, D::Error> {

    return Ok(match Value::deserialize(deserializer)?{
        Value::Object(map) => {
            if let Some(Value::String(osname)) = map.get("name"){

                match osname.to_string().as_str(){
                    "windows" => Some(Platform::Windows),   
                    "windows-arm64" => Some(Platform::WindowsArm64),
                    "linux" => Some(Platform::Linux),
                    "linux-arm32" => Some(Platform::LinuxArm32),
                    "linux-arm64" => Some(Platform::LinuxArm64),
                    "osx-arm64" => Some(Platform::MacOsArm64),
                    _ => Some(Platform::Unknown)
                }
            } else {
                None
            }

        }
        _ => return Err(de::Error::custom("Invalid type"))
    })
}

/// This struct is used to store the artifact information of a library or client.
#[derive(Debug,Clone,Deserialize,PartialEq)]
pub struct Artifact{
    pub url:String,
    pub size:i64,
    pub sha1:String,
}

/// This struct is used to store the download information of a library or client.
/// It contains the artifact information or classifiers information, classifiers
/// is used to store some platform-specific libraries.
#[derive(Debug,Clone,Deserialize,PartialEq)]
pub struct Download{
    pub artifact:Option<Artifact>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub classifiers:HashMap<String,Artifact>,
}

/// This struct is used to store the extract information of a library.
#[derive(Debug,Clone,Deserialize,PartialEq,Hash,Eq)]
pub struct Extract{
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    exclude:Vec<String>
}

/// This struct is used to store the common library information.
#[derive(Debug,Clone,Deserialize,PartialEq)]
pub struct CommonLibrary {
    pub name:String,
    pub downloads:Download,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub rules:Vec<Rule>,
    pub extract:Option<Extract>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub natives:HashMap<String,String>
}

/// To know your platform can use this package or not!
///
/// # Arguments
///
/// * `rules`: the rules vec
///
/// returns: bool
///
/// # Examples
///
/// ```
/// let test_case = vec![Rule{
///     action: Action::Allow,
///     os: None,
/// }];
/// let test_result = rules_analyzer(test_case);
/// assert!(test_result);
///
/// ```
pub fn rules_analyzer(rules:Vec<Rule>) -> bool{

    let mut allow = rules.is_empty(); // if empty all allow, if not disallow.

    for rule in rules.iter(){
        if equal_my_platform(&rule.os){
            if rule.action == Action::Disallow {
                return false; // your platform is not allow
            }else{
                allow = true // a
            }
        }
    }
    allow
}

/// This struct is used to store the maven-based library information.
#[derive(Debug,Clone,Deserialize,PartialEq)]
pub struct MavenLibrary{
    pub name:String,
    pub url:String,
}

async fn fetch_and_store(file:PathBuf, url:&str) -> Result<()>{
    let res = reqwest::get(url).await;
    match res {
        Ok(res) => {

            if !res.status().is_success(){
                return Err(MetadataFileError::Fetching.into())
            }

            let body = res.text().await.expect("this should be success!");
            tokio::fs::write(file,body.into_bytes()).await.unwrap();
            Ok(())
        }
        Err(e) => {
            Err(MetadataFileError::Unknown(e.to_string()).into())
        }
    }
}

/// This enum is used to store the library information, it contains the common library
/// information or maven-based library information.
#[derive(Debug,Clone,Deserialize,PartialEq)]
#[serde(untagged)]
pub enum Library{
    Common(CommonLibrary),
    Maven(MavenLibrary)
}

#[derive(Debug,Clone,Deserialize,PartialEq)]
pub struct AssetObject {
    pub hash:String,
    pub size:i64,
}

impl From<AssetObject> for String{
    fn from(value: AssetObject) -> Self {
        format!("{}:{}",value.hash,value.size)
    }
}

#[derive(Debug,Clone,Deserialize,PartialEq,Load)]
pub struct AssetInfo{
    pub objects:HashMap<String,AssetObject>
}

#[derive(Debug,Clone,Deserialize,PartialEq)]
pub struct AssetIndex{
    pub id:String,
    pub sha1:String,
    pub size:i64,
    pub total_size:i64,
    pub url:String
}

impl AssetIndex{
    pub async fn get_objects(&self,app:&AppHandle)->Result<AssetInfo>{
        let assets_index_root = ASSET_INDEX_ROOT.to_path(&app)?;
        let assets_index_file = assets_index_root.join(&self.id);
        
        fetch_and_store(assets_index_file.clone(),&self.url).await?;
        
        let file = *AssetInfo::load(&assets_index_file)?;
        Ok(file)
    }
}

/// This struct is used to store the version details of a package, like minecraft, fabric-loader, etc.
/// Compared with VersionInfo, this struct contains more details, like the dependencies, libraries, main class, etc.
#[derive(Debug,Clone,Deserialize,PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VersionDetails {
    pub format_version: i32,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub conflicts:Vec<DependencyPackage>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub requires:Vec<DependencyPackage>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub libraries:Vec<Library>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub maven_files:Vec<Library>, // for forge and neoforge 
    pub name:String,
    pub uid:String,
    pub release_time:String,
    #[serde(rename="type")]
    pub type_:Option<String>, // neoforged hasn't this field
    pub version:String,
    pub volatile: Option<bool>,
    pub main_class:Option<String>,
    pub main_jar:Option<CommonLibrary>,
    pub minecraft_arguments:Option<String>,
    pub asset_index:Option<AssetIndex>,
} 

/* the function to handle metadata */
pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

#[derive(Error, Debug, PartialEq)]
pub enum MetadataFileError{
    #[error("IO ERROR {0}")]
    IO(ErrorKind),
    #[error("the cached file is not found")]
    Invalid,
    #[error("the cached file is not found")]
    Fetching,
    #[error("the cached file is not found")]
    RetryTooManyTime,
    #[error("Unknown error, details: {0}")]
    Unknown(String)
}

#[derive(Clone, Debug, PartialEq)]
pub enum SHAType{
    SHA1(Vec<u8>),
    SHA256(Vec<u8>)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetadataSetting{
    api:String,
    cache_override:Option<PathBuf>,
    pub package_list: TimeSensitiveData<PackageList>
}

impl Default for MetadataSetting{
    fn default() -> Self {
        MetadataSetting{
            api: "https://meta.prismlauncher.org/v1/".to_string(),
            cache_override: None,
            package_list: TimeSensitiveData::new_invalid(PackageList::default()),
        }
    }
}

impl MetadataSetting{

    /// # How we handle cached file
    /// First we will have [PackageList] in [Metadata], we will fetch data
    /// from [api_path], which contain package info.
    /// Then we will check [PackageDetails] is in [cached_path] folder and sha of [PackageDetails],
    /// if not download or re-download it from [api_path].
    /// [VersionDetails] is also work like this, so this is the function to handle it.
     async fn get_cached_file_content(path:PathBuf, sha:SHAType) -> Result<String, MetadataFileError> {
        let i = tokio::fs::read_to_string(path).await;

        let content = match i {
            Ok(content) => {content}
            Err(e) =>{
                return Err(MetadataFileError::IO(e.kind()))
            }
        };

        match sha {
            SHAType::SHA1(sha1) => {
                assert_eq!(sha1.len(),20);
                let mut hasher = Sha1::new();
                sha1::digest::Update::update(&mut hasher, content.clone().as_bytes());
                let result = &hasher.finalize()[..];
                if sha1 != result{
                    return Err(MetadataFileError::Invalid)
                }
            }
            SHAType::SHA256(sha256) => {
                assert_eq!(sha256.len(),32);
                let mut hasher = Sha256::new();
                hasher.update(content.clone().as_bytes());
                let result = &hasher.finalize()[..];
                if sha256 != result{
                    return Err(MetadataFileError::Invalid)
                }
            }
        }

        Ok(content)
     }
    
    async fn check_and_create_folder(path:PathBuf) -> Result<(),MetadataFileError>{
        match tokio::fs::create_dir_all(path).await{
            Ok(_) => {Ok(())}
            Err(e) => {
                Err(MetadataFileError::IO(e.kind()))
            }
        }
    }

    pub async fn get_package_details(&self,default:PathBuf,uid:&str,sha:SHAType) -> Result<PackageDetails>{
        let cache_root = self.cache_override.clone().unwrap_or({
            default
        });
        
        let path = cache_root.join(uid);
        let file = path.join("index.json");
        let url = format!("{}/{}/index.json",self.api,uid);


        Self::check_and_create_folder(path.clone()).await?;

        let mut count = 0;

        loop{
            if count > 3{
                return Err(MetadataFileError::RetryTooManyTime.into())
            }
            count+=1;
            let content = Self::get_cached_file_content(file.clone(),sha.clone()).await;
            return match content {
                Ok(str) => {
                    Ok(serde_json::from_str(&str).unwrap())
                }
                Err(error) => {
                    if let MetadataFileError::IO(e) = error {
                        if let NotFound = e{
                            let url = format!("{}/{}",self.api,uid);
                            fetch_and_store(file.clone(),&url).await?;
                            continue;
                        }
                        return Err(MetadataFileError::IO(e).into())
                    }

                    if let MetadataFileError::Invalid = error{
                        fetch_and_store(file.clone(),&url).await?; // error here!
                        continue
                    }

                    return Err(error.into())

                }
            }
        }
    }

    pub async fn get_version_details(&self,default:PathBuf,uid:&str,version:&str,sha:SHAType) -> Result<VersionDetails>{
        let cache_root = self.cache_override.clone().unwrap_or({
            default
        });

        let path = cache_root.join(uid);
        let file = path.join(format!("{}.json",version));
        let url = format!("{}/{}/{}.json",self.api,uid,version);

        Self::check_and_create_folder(path.clone()).await?;
        let mut count = 0;

        loop{

            if count > 3{
                return Err(MetadataFileError::RetryTooManyTime.into())
            }
            count+=1;

            let content = Self::get_cached_file_content(file.clone(),sha.clone()).await;
            return match content {
                Ok(str) => {
                    Ok(serde_json::from_str(&str).unwrap())
                }
                Err(error) => {
                    if let MetadataFileError::IO(e) = error {
                        if let NotFound = e{    
                            fetch_and_store(file.clone(),&url).await?;
                            continue;
                        }
                        return Err(MetadataFileError::IO(e).into())
                    }

                    if let MetadataFileError::Invalid = error{
                        fetch_and_store(file.clone(),&url).await?;
                    }

                    return Err(error.into())

                }
            }
        }
    }
    
    pub async fn refresh(&mut self) -> Result<(),MetadataFileError>{
        
        let res = reqwest::get(&self.api).await;
        match res {
            Ok(res) => {
                let pkg = res.json::<PackageList>().await;
                
                match pkg {
                    Ok(content) => {
                        self.package_list = TimeSensitiveData::new(content);
                        Ok(())
                    }
                    Err(e) => {
                        Err(MetadataFileError::Unknown(e.to_string()))
                    }
                }
            }
            Err(_) => {Err(MetadataFileError::Fetching)}
        }
    }

}




#[cfg(test)]
mod test{
    use std::{env, fs};
    use std::io::ErrorKind;
    use serde_json::json;
    use crate::utils::minecraft::metadata::{VersionDetails, PackageDetails, Rule, decode_hex, MetadataFileError, MetadataSetting, Action, rules_analyzer, Platform};
    use crate::utils::minecraft::metadata::SHAType::{SHA1, SHA256};

    #[tokio::test]
    async fn test_version_info(){

        let test_api = vec![
            "https://meta.prismlauncher.org/v1/org.quiltmc.quilt-loader/",
            "https://meta.prismlauncher.org/v1/net.minecraftforge/",
            "https://meta.prismlauncher.org/v1/com.mumfrey.liteloader/",
            "https://meta.prismlauncher.org/v1/net.fabricmc.fabric-loader",
            "https://meta.prismlauncher.org/v1/net.fabricmc.intermediary/",
            "https://meta.prismlauncher.org/v1/net.minecraft/",
            "https://meta.prismlauncher.org/v1/org.lwjgl/",
            "https://meta.prismlauncher.org/v1/org.lwjgl3/",
        ];

        for i in &test_api{
            let _res = reqwest::get(i.to_string()).await.unwrap().json::<PackageDetails>().await.unwrap();
            //println!("{:?}",res);
        }
    }

    #[tokio::test]
    async fn test_rule(){

        let jsons = vec![
            json!(
                {
                    "action": "allow"
                }
            ).to_string(),
            json!(
                {
                    "action": "disallow",
                    "os": {
                        "name": "osx-arm64"
                    }
                }
            ).to_string(),
        ];

        for i in &jsons{
            let _res = serde_json::from_str::<Rule>(i).unwrap();
        }
    }

    #[tokio::test]
    async fn test_package_details(){

        let test_api = vec![
            "https://meta.prismlauncher.org/v1/org.lwjgl/2.9.3.json", // lwjgl 2.9.3
            "https://meta.prismlauncher.org/v1/org.lwjgl3/3.3.1.json", // lwjgl 3.3.1
            "https://meta.prismlauncher.org/v1/org.lwjgl3/3.2.1.json", // lwjgl 3.2.1, which has natives and classifiers
            "https://meta.prismlauncher.org/v1/net.minecraft/1.21.json", // minecraft 1.21
            "https://meta.prismlauncher.org/v1/net.minecraft/1.0.json", // minecraft 1.0
            "https://meta.prismlauncher.org/v1/net.fabricmc.fabric-loader/0.15.1.json", // fabric-loader 0.15.1
            "https://meta.prismlauncher.org/v1/net.fabricmc.intermediary/1.21.json", // fabric-intermediary 1.21
            "https://meta.prismlauncher.org/v1/net.minecraftforge/51.0.16.json", // forge 51.0.16/1.21
            "https://meta.prismlauncher.org/v1/net.neoforged/21.0.21-beta.json", // neoforged 21.0.21-beta
            "https://meta.prismlauncher.org/v1/org.quiltmc.quilt-loader/0.26.1-beta.1.json", // quilt-loader 0.26.1-beta.1
        ];

        for i in &test_api{
            let _res = reqwest::get(i.to_string()).await.unwrap().json::<VersionDetails>().await.unwrap();
            // println!("{:?}",_res)
        }
    }

    #[tokio::test]
    async fn test_package_info(){
        let url = "https://meta.prismlauncher.org/v1/";
        let _res = reqwest::get(url).await.unwrap().json::<crate::utils::minecraft::metadata::PackageList>().await.unwrap();
    }

    #[tokio::test]
    async fn test_metadata_get_cached_file_content_not_found(){
        let path = env::current_dir().unwrap();

        let test_sha1 = "a6b0f24b706870b0e0c1813f0805850a2a2988bf";
        let decode_from_hex = decode_hex(test_sha1).unwrap();

        let test = MetadataSetting::get_cached_file_content(path.join("not found"), SHA1(decode_from_hex)).await;
        let data = Err(MetadataFileError::IO(ErrorKind::NotFound));
        assert_eq!(test,data)
    }

    #[tokio::test]
    async fn test_metadata_get_cached_file_content_sha1(){
        let path = env::current_dir().unwrap();

        let test_str = "it's is a str";
        let test_sha1 = "a6b0f24b706870b0e0c1813f0805850a2a2988bf";

        tokio::fs::write(path.join("test1.txt"),test_str.to_string()).await.unwrap();
        let decode_from_hex = decode_hex(test_sha1).unwrap();
        let _ = MetadataSetting::get_cached_file_content(path.join("test1.txt"), SHA1(decode_from_hex)).await.unwrap();

        fs::remove_file(path.join("test1.txt")).unwrap();
    }

    #[tokio::test]
    async fn test_metadata_get_cached_file_content_sha256(){
        let path = env::current_dir().unwrap();


        let test_str = "it's is a str";
        let test_sha1 = "5541938b005426931bd062af5b23d0ed69ca5cf577ae80dc441e3d1d7f38c072";

        tokio::fs::write(path.join("test2.txt"),test_str.to_string()).await.unwrap();
        let decode_from_hex = decode_hex(test_sha1).unwrap();
        let _ = MetadataSetting::get_cached_file_content(path.join("test2.txt"), SHA256(decode_from_hex)).await.unwrap();

        fs::remove_file(path.join("test2.txt")).unwrap();
    }

    
    #[tokio::test]
    async fn fetch_package_details(){
        let path = env::current_dir().unwrap();
        let test_path = path.join("test");
        let metadata = MetadataSetting::default();
        let uid = "org.lwjgl";
        let sha = SHA256(decode_hex("c0094ab29be4be93b7cf0e05067608814afb6c4f40223784ecb69e6635cd6bbf").unwrap());
        
        metadata.get_package_details(test_path.clone(),uid,sha).await.unwrap();
        
        // clean up
        tokio::fs::remove_dir_all(test_path.clone()).await.unwrap();
       
    }

    #[tokio::test]
    async fn fetch_version_details(){
        let path = env::current_dir().unwrap();
        let test_path = path.join("test2");
        let metadata = MetadataSetting::default();
        let uid = "org.lwjgl";
        let version = "2.9.1";
        let sha = SHA256(decode_hex("be9e7ac96da952c9461d6f08e5a4e4e0ffcc2dafba291b48ed430269a9af0497").unwrap());

        metadata.get_version_details(test_path.clone(),uid,version,sha).await.unwrap();

        // clean up
        tokio::fs::remove_dir_all(test_path.clone()).await.unwrap();
    }

    #[tokio::test]
    async fn refresh_all(){
        let mut metadata = MetadataSetting::default();
        metadata.refresh().await.unwrap();
    }

    #[cfg(target_arch = "x86_64")]
    #[cfg(target_os = "linux")]
    #[test]
    fn rule_test(){
        let test_case = vec![Rule{
                action: Action::Allow,
                os: None,
            }];
        let test_result = rules_analyzer(test_case);
        assert!(test_result);

        let test_case = vec![
            Rule{
                action: Action::Allow,
                os: None,
            },
            Rule{
                action: Action::Disallow,
                os:Some(Platform::Linux)
            }
        ];
        let test_result = rules_analyzer(test_case);
        assert!(!test_result);

        let test_case = vec![
            Rule{
                action: Action::Disallow,
                os: Some(Platform::MacOsArm64),
            },
            Rule{
                action: Action::Allow,
                os:Some(Platform::Linux)
            }
        ];
        let test_result = rules_analyzer(test_case);
        assert!(test_result);
    }
    
}
