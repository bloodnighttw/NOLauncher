use std::collections::HashMap;
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::{Value};

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
pub struct DependencyPackage {
    pub suggests:Option<String>,
    pub equals:Option<String>,
    pub uid: String
}

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
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
    version:String,
    volatile: Option<bool>
}

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PackageInfo {
    format_version:i32,
    name:String,
    uid:String,
    versions:Vec<VersionInfo>
}

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
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

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
pub enum Action{
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "disallow")]
    Disallow
}

fn os_processing<'de, D: Deserializer<'de>>(deserializer: D) -> anyhow::Result<Option<Platform>, D::Error> {

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

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
pub struct Rule{
    action:Action,
    #[serde(deserialize_with = "os_processing", default)]
    os:Option<Platform>
}


#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
pub struct Artifact{
    pub url:String,
    pub size:i64,
    pub sha1:String,
}

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
pub struct Download{
    artifact:Option<Artifact>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    classifiers:HashMap<String,Artifact>,
}

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
pub struct Extract{
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    exclude:Vec<String>
}


#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
pub struct CommonLibrary {
    name:String,
    downloads:Download,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    rules:Vec<Rule>,
    extract:Option<Extract>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    natives:HashMap<String,String>
}

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
pub struct MavenLibrary{
    pub name:String,
    pub url:String,
}

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
#[serde(untagged)]
pub enum Library{
    Common(CommonLibrary),
    Maven(MavenLibrary)
}


#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PackageDetails{
    pub format_version: i32,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub conflicts:Vec<DependencyPackage>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub requires:Vec<DependencyPackage>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub libraries:Vec<Library>,
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
}

#[cfg(test)]
mod test{
    use serde_json::json;
    use crate::utils::metadata::data::{PackageDetails, PackageInfo, Rule};

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
            let _res = reqwest::get(i.to_string()).await.unwrap().json::<PackageInfo>().await.unwrap();
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
            let _res = reqwest::get(i.to_string()).await.unwrap().json::<PackageDetails>().await.unwrap();
            // println!("{:?}",_res)
        }
    }
}
