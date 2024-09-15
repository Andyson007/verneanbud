use crossterm::event::KeyEvent;
use futures::Future;
use ratatui::{layout::Rect, Frame};
use sea_orm::{ConnectOptions, DatabaseConnection, DbErr};
use std::{fmt::Debug, pin::Pin, rc::Rc, sync::Mutex};

use crate::style::Style;

pub mod idea;

pub trait Popup {
    fn debug_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    fn render(&self, style: Style, area: Rect, buffer: &mut Frame);
    fn handle_input(&mut self, key: KeyEvent) -> Action;
}

impl Debug for dyn Popup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.debug_fmt(f)
    }
}

pub enum Action<'a> {
    Close,
    Nothing,
    Db(Box<dyn FnOnce(ConnectOptions) -> Pin<Box<dyn Future<Output = Result<(), DbErr>> + 'a>>>),
}

impl Action<'_> {
    pub fn close_popup(&self) -> bool {
        !matches!(self, Self::Nothing)
    }
}
