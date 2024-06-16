use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::RwLock;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NoLauncherConfig {
    #[serde(default)]
    pub activate_user_uuid: Option<String>,
    #[serde(default)]
    pub mc_metadata:String
}

pub type LauncherConfig = Arc<RwLock<NoLauncherConfig>>;

impl Default for NoLauncherConfig {
    fn default() -> Self {
        NoLauncherConfig {
            activate_user_uuid: None,
            mc_metadata:"https://meta.prismlauncher.org/v1/".to_string()
        }
    }
}

impl NoLauncherConfig {
    pub async fn save(&self, app: &AppHandle) -> Result<(), String> {
        // save the config to the disk
        let config_path = app.path().app_config_dir();
        if let Ok(config_path) = config_path {
            let config_path = config_path.join("config.json");
            let content = serde_json::to_string(self).unwrap();
            let res = tokio::fs::write(config_path, content).await;
            return match res {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to save the config: {}", e.to_string())),
            };
        }
        return Err("Failed to get the config path".to_string());
    }

    pub async fn read_from_path(config_path: PathBuf) -> Result<LauncherConfig, String> {
        let res = tokio::fs::read_to_string(config_path).await;
        return match res {
            Ok(content) => {
                let config = serde_json::from_str::<NoLauncherConfig>(&content);
                match config {
                    Ok(config) => Ok(Arc::new(RwLock::new(config))),
                    Err(e) => Err(format!(
                        "Failed to parse the config. details:{}",
                        e.to_string()
                    )),
                }
            }
            Err(e) => Err(format!("Failed to read the config: {}", e.to_string()))
        };
    }
}