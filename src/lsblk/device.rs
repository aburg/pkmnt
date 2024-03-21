use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Device {
    name: String,
    size: String,

    children: Option<Vec<Device>>,
}
