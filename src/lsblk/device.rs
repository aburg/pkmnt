use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Device {
    name: String,
    size: String,
    mountpoints: Vec<Option<String>>,

    pub children: Option<Vec<Device>>,
}
