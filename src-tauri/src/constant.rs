/// This File contain all constant that no launcher need!
/// Include: Download Place, Cached Path, UID......

use crate::utils::config::SavePath;

pub const NOLAUNCHER_CONFIG_FILE: SavePath = SavePath::Config(&["config.json"]);
pub const ACCOUNTS_DATA:SavePath = SavePath::Config(&["accounts.json"]);