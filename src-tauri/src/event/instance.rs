use std::sync::Arc;
use std::sync::atomic::AtomicI64;
use tauri::{AppHandle, Manager};
use crate::utils::minecraft::instance::{SafeInstanceStatus, Status};

#[derive(Clone, serde::Serialize)]
pub struct StatusPayload {
    #[serde(flatten)]
    pub status: Status
}

#[derive(Clone, serde::Serialize)]
struct ProgressPayload {
    now:Arc<AtomicI64>,
    total:i64
}
pub async fn instance_status_update(id:&str, app: AppHandle, map:&SafeInstanceStatus) {

    let status = map.read().await.get(id).unwrap_or(&Status::Stopped).clone();
    app.emit(&format!("instance_status_update:{id}"), StatusPayload { status })
        .unwrap()

}

pub async fn progress_status_update(now:Arc<AtomicI64>,total:i64,app:&AppHandle,id:&str){
    app.emit(&format!("progress_update:{id}"),ProgressPayload{
        now,
        total,
    }).unwrap()
}