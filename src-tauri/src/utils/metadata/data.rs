use serde::{Deserialize, Serialize};

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
struct DependencyPackage {
    suggests:Option<String>,
    equals:Option<String>,
    uid: String
}

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VersionInfo{
    recommended:bool,
    release_time:String,
    sha256:String,
    #[serde(rename="type")]
    rtype:Option<String>,
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    requires:Vec<DependencyPackage>,
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    conflicts:Vec<DependencyPackage>,
    version:String
}

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VersionInfoList{
    format_version:i32,
    name:String,
    uid:String,
    versions:Vec<VersionInfo>
}

#[cfg(test)]
mod test{
    use crate::utils::metadata::data::VersionInfoList;

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
            let res = reqwest::get(i.to_string()).await.unwrap().json::<VersionInfoList>().await.unwrap();
            //println!("{:?}",res);
        }

    }

}
