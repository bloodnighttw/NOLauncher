use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug,Default)]
pub struct InstanceConfig{
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub dep: HashMap<String,String>, // key: uid, value: version,
    #[serde(default)]
    pub top: String // top dep uid
}