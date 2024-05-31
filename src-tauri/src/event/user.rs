use std::sync::Arc;
use tauri::{AppHandle, Manager};
use crate::minecraft::auth::LoginAccount;

#[derive(Clone, serde::Serialize)]
struct UUIDPayload{
    pub uuid: String,
    pub username: String,
    pub img: String,
}

pub async fn change_user(login_account: Arc<LoginAccount>,app: &AppHandle){
    app.emit_all("change_user",UUIDPayload{
        uuid: login_account.profile.id.clone(),
        username: login_account.profile.name.clone(),
        img: login_account.profile.skins[0].url.clone()
    }).unwrap()
}