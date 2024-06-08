use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Deserialize, Serialize,Debug,Clone)]
pub struct NoLauncherConfig{
    pub activate_user_uuid: Option<String>,
}

pub type LauncherConfig = Arc<RwLock<NoLauncherConfig>>;