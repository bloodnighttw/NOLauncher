use tauri::{AppHandle, Manager};
use crate::utils::minecraft::instance::{InstanceStatus, Status};

#[derive(Clone, serde::Serialize)]
struct StatusPayload {
    pub status: Status
}

pub async fn instance_status_update(id:&str, app: &AppHandle) {
    let map = app.state::<InstanceStatus>();

    {
        let status = map.read().await.get(id).unwrap_or(&Status::Stopped).clone();
        app.emit(&format!("instance_status_update:{id}"), StatusPayload { status })
            .unwrap()
    }
    
}