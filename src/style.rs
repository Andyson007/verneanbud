use ratatui::style::{Color, Style as rataStyle};

#[derive(Debug, Copy, Clone)]
pub struct Style {
    pub highlighted: rataStyle,
    pub not_highlighted: rataStyle,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            highlighted: rataStyle::new().fg(Color::Yellow),
            not_highlighted: rataStyle::new().fg(Color::White),
        }
    }
}
