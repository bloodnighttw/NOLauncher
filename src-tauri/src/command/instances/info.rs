use std::path::PathBuf;

use reginleif_utils::save_path::BaseStorePoint;
use tauri::State;

use crate::utils::{result::CommandResult, settings::instances::NLInstanceList};

#[derive(Debug, serde::Serialize)]
pub struct InstanceListPayload{
    pub id:String,
    pub name:String,
    pub base_path:PathBuf
}

#[tauri::command]
pub async fn instance_list(
    instance_list:State<'_,NLInstanceList>
)-> CommandResult<Vec<InstanceListPayload>> {
    
    let instances = instance_list.list_cloned().await;
    let base_path = instance_list.store_point().get_base();
    
    let list = instances.iter().map(|(id,name)|{
        InstanceListPayload{
            id:id.clone(),
            name:name.clone(),
            base_path:base_path.clone()
        }
    }).collect();

    Ok(list)
}