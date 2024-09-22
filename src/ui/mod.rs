//! This module is responsible for handling all ui operations
//! It uses an [`App`] instance for this

use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    Frame,
};

use crate::app::App;

mod ideas;

/// Draws the ui.
/// It probably assumes a lot about the
/// terminal being in raw mode etc.
pub fn ui(frame: &mut Frame, app: &App) {
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(Constraint::from_percentages([40, 60]))
        .split(frame.area());
    ideas::render(app, frame, main_layout[0], main_layout[1]);
    if let Some(x) = &app.popup {
        let area = centered_rect(70, 80, frame.area());
        x.render(app.style, area, frame);
    }
}

/// This code is absolutely stolen from the ratatui json example
/// Draws a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
