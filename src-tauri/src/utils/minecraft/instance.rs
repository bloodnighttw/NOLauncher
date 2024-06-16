use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug)]
struct InstanceConfig{
    #[serde(default = "Default::default")]
    name: String
}

impl Default for InstanceConfig{
    fn default() -> Self {
        InstanceConfig{
            name:"Instance Name".to_string(),
        }
    }
}