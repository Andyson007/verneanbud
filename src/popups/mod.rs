//! Responsible for storing a concrete popup type
use core::fmt;
use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

use crate::{app::DbActionReturn, style::Style};

pub mod idea;
pub mod comment;

/// A trait describing a popup. This is used for storing the popup more easily in `App`
pub trait Popup: fmt::Debug {
    /// Renders te popup onto the frame
    fn render(&self, style: Style, area: Rect, buffer: &mut Frame);
    /// Handles any imput sent to the popup
    fn handle_input<'a>(&mut self, key: &KeyEvent) -> Action<'a>;
}

/// What action should be taken when `handle_input` finishes running
pub enum Action<'a> {
    /// The popup was exited and should be exited without further action
    Close,
    /// The popup has handled all inputs has consumed the input
    Nothing,
    /// The popup has exited and wants to edit the DB. The popup should be closed
    Db(DbActionReturn<'a>),
}

impl Action<'_> {
    #[must_use]
    /// Check whether the popup should be closed or not
    /// true -> close
    /// false -> keep open
    pub const fn close_popup(&self) -> bool {
        !matches!(self, Self::Nothing)
    }
}
