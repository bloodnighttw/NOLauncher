use reginleif_utils::save_path::Save;
use tauri::State;

use crate::utils::{base_store::InstanceStorePoint, result::CommandResult, settings::instances::InstanceConfig};

#[tauri::command]
pub async fn instance_create(
    name:String,
    mc_version:String,
    uid:String,
    version:String,
    instance_path:State<'_,InstanceStorePoint>
)-> CommandResult<()> {

    let random_id = uuid::Uuid::new_v4().to_string();
    let instance = InstanceConfig{
        id: random_id,
        name,
        mc_version,
        uid,
        version,
    };

    instance.save(&instance_path)?;    
    
    Ok(())
}