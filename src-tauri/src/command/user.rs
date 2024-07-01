use crate::event::user::change_user;
use crate::utils::config::{SafeNoLauncherConfig, Storage};
use tauri::{AppHandle, State};
use crate::utils::minecraft::auth::{MinecraftProfile, SafeAccountList};
use crate::utils::result::CommandResult;

#[tauri::command]
pub async fn get_users(map: State<'_, SafeAccountList>) -> CommandResult<Vec<MinecraftProfile>> {
    let list = map.read().await.0
        .iter()
        .map(|x| x.profile.clone())
        .collect();
    
    Ok(list)
}

#[tauri::command]
pub async fn get_current_user(current_user: State<'_, SafeNoLauncherConfig>) -> Result<String, String> {
    let current_user = current_user.read().await.activate_user_uuid.clone();
    current_user.ok_or("no active user".to_string())
}

#[tauri::command]
pub async fn set_current_user(
    current_user: State<'_, SafeNoLauncherConfig>,
    app: AppHandle,
    id: String,
) -> CommandResult<String> {
    let mut current_user = current_user.write().await;
    current_user.activate_user_uuid = Some(id.clone());
    change_user(Some(id), &app).await;
    current_user.save_by_app(&app)?;
    Ok("".to_string())
}

#[tauri::command]
pub async fn logout_user(
    accounts: State<'_, SafeAccountList>,
    config: State<'_, SafeNoLauncherConfig>,
    app: AppHandle,
    id: String,
) -> CommandResult<()> {

    {
        let mut accounts = accounts.write().await;
        accounts.remove(&id);
        accounts.save_by_app(&app)?;
    }

    {
        let mut config =config.write().await;
        if let Some(uid) = &config.activate_user_uuid{
            if uid == &id{
                config.activate_user_uuid = None;
                config.save_by_app(&app)?;
            }
        }
    };

    change_user(None, &app).await;
    Ok(())
}