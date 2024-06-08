use tauri::{AppHandle, State};
use crate::event::user::change_user;
use crate::minecraft::auth::MinecraftUUIDMap;
use crate::utils::config::{LauncherConfig};


#[tauri::command]
pub async fn get_users(map:State<'_,MinecraftUUIDMap>) -> Result<String,String> {
    let mut vec = Vec::new();
    for (_,login_account) in map.read().await.iter(){
        vec.push(login_account.profile.clone());
    }
    Ok(serde_json::to_string(&vec).unwrap())
}

#[tauri::command]
pub async fn get_current_user(current_user:State<'_,LauncherConfig>) -> Result<String,String> {
    let current_user = current_user.read().await.activate_user_uuid.clone();
    return match current_user {
        None => { Err("no activate user.".to_string())}
        Some(content) => {Ok(content)}
    }
}

#[tauri::command]
pub async fn set_current_user(current_user:State<'_,LauncherConfig>,app:AppHandle,id:String) -> Result<String,String> {
    let mut current_user = current_user.write().await;
    current_user.activate_user_uuid = Some(id.clone());
    change_user(id.clone(),&app).await;
    let _ = current_user.save(&app).await;
    Ok("".to_string())
}