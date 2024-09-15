//! This module is responsible for handling all ui operations
//! It uses an [`App`] instance for this

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, List, ListState},
    Frame,
};

use ratatui::prelude::*;

use crate::app::App;

/// Draws the ui.
/// It probably assumes a lot about the
/// terminal being in raw mode etc.
pub fn ui(frame: &mut Frame, app: &App) {
    let mut list_state = ListState::with_selected(ListState::default(), Some(1));
    let list = List::new((0..100).map(|_| "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"))
        .block(Block::bordered().title("List").style(Color::White))
        .scroll_padding(3)
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED));

    frame.render_stateful_widget(list, frame.area(), &mut list_state);

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
