//! The main module.
//! implements App and all of its features

use crossterm::event::{KeyCode, KeyEvent};
use futures::executor::block_on;
use sea_orm::ConnectOptions;
use sea_orm::SqlxError;
use std::fmt::Debug;

use crate::popups::idea::IdeaPopup;
use crate::popups::{Action, Popup};
use crate::style::Style;

const DATABASE_URL: &str = "postgres://vern:vern@localhost:5432/verneanbud";
// const DB_NAME: &str = "verneanbud";

/// The appstruct is responsible for containing all information
/// describing the current state
#[derive(Debug)]
pub struct App {
    view: View,
    pub(crate) popup: Option<Box<dyn Popup + 'static>>,
    conn_opts: ConnectOptions,
    pub(crate) style: Style,
}

impl App {
    #[allow(clippy::missing_errors_doc)]
    /// Creates an app
    ///
    /// Initialized stuff like the db
    pub fn new() -> Result<Self, SqlxError> {
        // let db = block_on(async { Database::connect(DATABASE_URL).await })?;
        let mut conn_opts = ConnectOptions::new(DATABASE_URL);
        conn_opts
            .max_connections(100)
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);
        Ok(Self {
            view: View::Ideas,
            conn_opts,
            popup: None,
            style: Style::default(),
        })
    }

    /// Handles an input
    pub fn handle_input(&mut self, key: KeyEvent) -> bool {
        if if let Some(popup) = &mut self.popup {
            let popup_action = popup.handle_input(key);
            let should_close = popup_action.close_popup();
            if let Action::Db(db_action) = popup_action {
                let ans = block_on(db_action(self.conn_opts.clone()));
                println!("{ans:?}");
            }
            if !should_close {
                return false;
            }
            true
        } else {
            false
        } {
            self.popup = None;
            return false;
        }

        if matches!(key.code, KeyCode::Esc | KeyCode::Char('q')) {
            return true;
        }

        match self.view {
            View::Ideas => match key.code {
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
