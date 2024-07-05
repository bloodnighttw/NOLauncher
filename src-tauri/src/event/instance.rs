use std::sync::Arc;
use std::sync::atomic::AtomicI64;
use tauri::{AppHandle, Manager};
use crate::utils::minecraft::instance::Status;

#[derive(Clone, serde::Serialize)]
pub struct StatusPayload {
    pub status: String
}

#[derive(Clone, serde::Serialize)]
struct ProgressPayload {
    now:Arc<AtomicI64>,
    total:i64
}
pub async fn instance_status_update(app: &AppHandle,id:&str,status:&Status) {
    
    let status = match status {
        Status::Running(_) => {"Running"}
        Status::Preparing => {"Preparing"}
        Status::Checking { .. } => {"Checking"}
        Status::Downloading { .. } => {"Downloading"}
        Status::Stopped => {"Stopped"}
        Status::Failed { .. } => {"Failed"}
    }.to_string();
    
    app.emit(&format!("instance_status_update:{id}"), StatusPayload { status })
        .unwrap()
}

pub async fn progress_status_update(now:Arc<AtomicI64>,total:i64,app:&AppHandle,id:&str){
    app.emit(&format!("progress_update:{id}"),ProgressPayload{
        now,
        total,
    }).unwrap()
}