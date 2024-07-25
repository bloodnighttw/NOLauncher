use reginleif_utils::save_path::Save;
use tauri::State;

use crate::utils::{result::CommandResult, settings::instances::{InstancesConfig, NLInstanceList}};

#[tauri::command]
pub async fn instance_create(
    name:String,
    mc_version:String,
    uid:String,
    version:String,
    instance_list:State<'_,NLInstanceList>
)-> CommandResult<()> {

    let random_id = uuid::Uuid::new_v4().to_string();
    let instance_path = instance_list.store_point();

    let instance = InstancesConfig{
        id: random_id.clone(),
        name,
        mc_version,
        uid,
        version,
    };

    instance.save(&instance_path)?;    

    instance_list.add(random_id).await?;

    Ok(())
}