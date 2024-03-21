use std::{process::Command, str::from_utf8};

use serde::{Deserialize, Serialize};

use super::device::Device;

#[derive(Deserialize, Serialize, Debug)]
pub struct Lsblk {
    blockdevices: Vec<Device>,
}

impl Lsblk {
    pub fn run() -> Self {
        let stdout = Command::new("lsblk")
            .arg("--json")
            .output()
            .expect("could not call lsblk")
            .stdout;

        serde_json::from_str::<Lsblk>(
            from_utf8(&stdout).expect("could not convert lsblk output to str"),
        )
        .expect("could not deserialize lsblk output")
    }
}
