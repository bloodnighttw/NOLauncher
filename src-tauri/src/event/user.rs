use tauri::{AppHandle, Manager};

#[derive(Clone, serde::Serialize)]
struct UUIDPayload {
    pub uuid: String,
}

pub async fn change_user(uuid: String, app: &AppHandle) {
    app.emit("change_user", UUIDPayload { uuid: uuid.clone() })
        .unwrap()
}
