use tauri::{AppHandle, Manager};

#[derive(Clone, serde::Serialize)]
struct UUIDPayload {
    pub uuid: Option<String>,
}

pub async fn change_user(uuid: Option<String>, app: &AppHandle) {
    app.emit("change_user", UUIDPayload { uuid: uuid.clone() })
        .unwrap()
}
