/// This File contain all constant that no launcher need!
/// Include: Download Place, Cached Path, UID......

use crate::utils::config::SavePath;

pub const NOLAUNCHER_CONFIG_FILE: SavePath = SavePath::Config(&["config.json"]);
pub const ACCOUNTS_DATA:SavePath = SavePath::Config(&["accounts.json"]);
pub const LIB_PATH:SavePath = SavePath::Config(&["libraries"]);
pub const CACHED_DEFAULT:SavePath = SavePath::Cache(&[]);
pub const ASSET_INDEX_ROOT:SavePath = SavePath::Config(&["assets","indexes"]);
pub const ASSET_OBJECT_ROOT:SavePath = SavePath::Config(&["assets","objects"]);

pub const NO_SIZE_DEFAULT_SIZE:i64 = 100000;
