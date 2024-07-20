use std::path::PathBuf;
use reginleif_macro::BaseStorePoint;
use tauri::{App, AppHandle, Manager};

#[derive(BaseStorePoint)]
pub struct MetadataStorePoint(PathBuf);

impl TryFrom<&AppHandle> for MetadataStorePoint {
    type Error = tauri::Error;

    fn try_from(app_handle: &AppHandle) -> Result<Self, Self::Error> {
        let dir = app_handle
            .path()
            .app_cache_dir()?;
        Ok(Self(dir))
    }
}

#[derive(BaseStorePoint)]
pub struct AssetStorePoint(PathBuf);

impl TryFrom<&AppHandle> for AssetStorePoint {
    type Error = tauri::Error;

    fn try_from(app_handle: &AppHandle) -> Result<Self, Self::Error> {
        let dir = app_handle
            .path()
            .app_data_dir()?
            .join("assets");
        Ok(Self(dir))
    }
}

#[derive(BaseStorePoint)]
pub struct InstanceStorePoint(PathBuf);

impl TryFrom<&AppHandle> for InstanceStorePoint {
    type Error = tauri::Error;

    fn try_from(app_handle: &AppHandle) -> Result<Self, Self::Error> {
        let cache_dir = app_handle
            .path()
            .app_data_dir()?
            .join("instances");
        Ok(Self(cache_dir))
    }
}

#[derive(BaseStorePoint, Clone)]
pub struct ConfigStorePoint(PathBuf);

impl <R> TryFrom<&&mut tauri::App<R>> for ConfigStorePoint
where R: tauri::Runtime{
    type Error = tauri::Error;

    fn try_from(value: &&mut App<R>) -> Result<Self, Self::Error> {
       let dir = value.path()
           .app_config_dir()?;
        Ok(Self(dir))
    }
}