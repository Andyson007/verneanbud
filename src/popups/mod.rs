//! Responsible for storing a concrete popup type
use core::fmt;
use crossterm::event::KeyEvent;
use futures::Future;
use ratatui::{layout::Rect, Frame};
use sea_orm::{ConnectOptions, DbErr};
use std::pin::Pin;

use crate::style::Style;

pub mod idea;

/// A trait describing a popup. This is used for storing the popup more easily in `App`
pub trait Popup: fmt::Debug {
    /// Renders te popup onto the frame
    fn render(&self, style: Style, area: Rect, buffer: &mut Frame);
    /// Handles any imput sent to the popup
    fn handle_input(&mut self, key: KeyEvent) -> Action;
}

/// What action should be taken when `handle_input` finishes running
pub enum Action<'a> {
    /// The popup was exited and should be exited without further action
    Close,
    /// The popup has handled all inputs has consumed the input
    Nothing,
    /// The popup has exited and wants to edit the DB. The popup should be closed
    #[allow(clippy::type_complexity)]
    Db(Box<dyn FnOnce(ConnectOptions) -> Pin<Box<dyn Future<Output = Result<(), DbErr>> + 'a>>>),
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
