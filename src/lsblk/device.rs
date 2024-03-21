use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Device {
    name: String,
    size: String,
    mountpoints: Vec<Option<String>>,

    children: Option<Vec<Device>>,
}
