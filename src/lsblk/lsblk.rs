use std::{process::Command, str::from_utf8};

use serde::{Deserialize, Serialize};

use super::device::Device;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Lsblk {
    pub blockdevices: Vec<Device>,
}

impl Lsblk {
    fn run() -> Self {
        let stdout = Command::new("lsblk")
            .arg("--json")
            .output()
            .expect("could not call lsblk")
            .stdout;

        let mut lsblk = serde_json::from_str::<Lsblk>(
            from_utf8(&stdout).expect("could not convert lsblk output to str"),
        )
        .expect("could not deserialize lsblk output");

        lsblk.flatten_blockdevice_children();

        lsblk
    }

    fn flatten_blockdevice_children(&mut self) {
        self.blockdevices = self
            .blockdevices
            .iter()
            .flat_map(|device| match device.children.clone() {
                Some(children) => children,
                None => vec![device.clone()],
            })
            .collect();
    }

    pub fn devices() -> Vec<Device> {
        Self::run().blockdevices
    }
}
