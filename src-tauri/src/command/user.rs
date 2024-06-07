use tauri::State;
use crate::minecraft::auth::MinecraftUUIDMap;

#[tauri::command]
pub async fn get_users(map:State<'_,MinecraftUUIDMap>) -> Result<String,String> {
    let mut vec = Vec::new();
    for (_,login_account) in map.read().await.iter(){
        vec.push(login_account.profile.clone());
    }
    Ok(serde_json::to_string(&vec).unwrap())
}