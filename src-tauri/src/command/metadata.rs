pub mod packages;

use anyhow::Result;
use tauri::{App, Manager, Runtime};

use crate::utils::{base_store::MetadataStorePoint, metadata::handler::PackageListHandler, module::BuilderWrapper};

pub fn init<R>(wrapper: BuilderWrapper<R>) -> BuilderWrapper<R>
where
    R:Runtime{
    wrapper
        .setup(|app:&&mut App<R>|{
            let metadata_path:MetadataStorePoint = app.try_into()?;
            let package_list_handler:PackageListHandler = metadata_path.clone().into();
            // to store package fetch list handler
            app.manage(package_list_handler);
            app.manage(metadata_path);
            Ok(()) as Result<(), Box<dyn std::error::Error>>
        })
    
}