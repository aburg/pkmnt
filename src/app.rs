use std::error;

use crate::{device_list::StatefulList, lsblk::lsblk::Lsblk};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    pub devices: StatefulList,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            devices: StatefulList::with_items(Lsblk::devices()),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
