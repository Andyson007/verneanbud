//! The main module.
//! implements App and all of its features

use crossterm::event::KeyCode;
use futures::executor::block_on;
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::fmt::Debug;

use crate::popups::idea::IdeaPopup;
use crate::popups::Popup;
use crate::style::Style;

const DATABASE_URL: &str = "postgres://vern:vern@localhost:5432/verneanbud";
// const DB_NAME: &str = "verneanbud";

/// The appstruct is responsible for containing all information
/// describing the current state
#[derive(Debug)]
pub struct App {
    view: View,
    pub(crate) popup: Option<Box<dyn Popup + 'static>>,
    db: DatabaseConnection,
    pub(crate) style: Style,
}

impl App {
    #[allow(clippy::missing_errors_doc)]
    /// Creates an app
    ///
    /// Initialized stuff like the db
    pub fn new() -> Result<Self, DbErr> {
        let db = block_on(async { Database::connect(DATABASE_URL).await })?;
        Ok(Self {
            view: View::Ideas,
            db,
            popup: None,
            style: Style::default(),
        })
    }

    /// Handles an input
    pub fn handle_input(&mut self, key: KeyCode) -> bool {
        if let Some(popup) = &mut self.popup {
            if popup.handle_input(key) {
                self.popup = None;
            };
            return false;
        }

        if matches!(key, KeyCode::Esc | KeyCode::Char('q')) {
            return true;
        }

        match self.view {
            View::Ideas => match key {
                KeyCode::Char('n') => self.popup = Some(Box::new(IdeaPopup::default())),
                KeyCode::Char(' ') => {
                    todo!("Toggle state")
                }
                KeyCode::Char('r') => {
                    todo!("Edit something")
                }
                _ => (),
            },
        }
        false
    }
}

#[derive(Debug)]
pub enum View {
    Ideas,
}
