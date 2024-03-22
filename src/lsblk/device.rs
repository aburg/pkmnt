// use ratatui::widgets::ListItem;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Device {
    pub name: String,
    pub size: String,
    mountpoints: Vec<Option<String>>,

    pub children: Option<Vec<Device>>,
}
