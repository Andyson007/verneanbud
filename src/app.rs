//! This module contains everything related to the appstate

use crossterm::event::{KeyCode, KeyEvent};
use futures::executor::block_on;
use sea_orm::{ConnectOptions, DbErr};

use crate::{
    popups::{idea::IdeaPopup, Action, Popup},
    style::Style,
    view_data::ViewData,
};

const DATABASE_URL: &str = "postgres://vern:vern@localhost:5432/verneanbud";
// const DB_NAME: &str = "verneanbud";

/// The appstruct is responsible for containing all information
/// describing the current state
#[derive(Debug)]
pub struct App {
    view: View,
    pub(crate) view_data: ViewData,
    pub(crate) popup: Option<Box<dyn Popup + 'static>>,
    conn_opts: ConnectOptions,
    pub(crate) style: Style,
}

impl App {
    #[allow(clippy::missing_errors_doc)]
    /// Creates an app
    ///
    /// Initialized stuff like the db
    pub async fn new() -> Result<Self, DbErr> {
        let mut conn_opts = ConnectOptions::new(DATABASE_URL);
        conn_opts
            .max_connections(100)
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);

        let view_data = ViewData::new(&conn_opts).await?;
        Ok(Self {
            view: View::Ideas,
            popup: None,
            style: Style::default(),
            view_data,
            conn_opts,
        })
    }

    /// Handles an input
    pub fn handle_input(&mut self, key: KeyEvent) -> bool {
        if self.popup.is_some() {
            {
                let popup_action = self.popup.as_mut().unwrap().handle_input(key);
                let should_close = popup_action.close_popup();
                if let Action::Db(db_action) = popup_action {
                    let _ = block_on(db_action(self.conn_opts.clone()));
                }
                if !should_close {
                    return false;
                }
            }

            self.popup = None;
            return false;
        }
        if matches!(key.code, KeyCode::Esc | KeyCode::Char('q')) {
            return true;
        }

        match self.view {
            View::Ideas => match key.code {
                KeyCode::Char('j') | KeyCode::Up => self.view_data.idea.up(),
                KeyCode::Char('k') | KeyCode::Down => self.view_data.idea.down(),
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
/// Descripbes the currently highlighted menu outside of popup
pub enum View {
    /// All ideas/suggestions from all the students
    Ideas,
}
