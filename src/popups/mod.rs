use crossterm::event::KeyCode;
use ratatui::{layout::Rect, Frame};
use std::fmt::Debug;

use crate::{app::View, style::Style};

pub mod idea;

pub trait Popup {
    fn debug_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    fn render(&self, style: Style, area: Rect, buffer: &mut Frame);
    fn handle_input(&mut self, key: KeyCode) -> bool;
}

impl Debug for dyn Popup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.debug_fmt(f)
    }
}
