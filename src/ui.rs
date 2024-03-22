use ratatui::{
    style::{Modifier, Style},
    widgets::{Block, Borders, List, StatefulWidget},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let devices = app
        .devices
        .items
        .iter()
        .map(|device| device.name.clone())
        .collect::<Vec<String>>();

    let list = List::new(devices)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">");

    StatefulWidget::render(
        list,
        frame.size(),
        frame.buffer_mut(),
        &mut app.devices.state,
    )
}
