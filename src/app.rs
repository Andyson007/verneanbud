//! This module contains everything related to the appstate

use std::pin::Pin;

use crossterm::event::{KeyCode, KeyEvent};
use futures::{executor::block_on, Future};
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
pub struct App<'a> {
    view: View,
    pub(crate) view_data: ViewData,
    pub(crate) popup: Option<Box<dyn Popup + 'static>>,
    conn_opts: ConnectOptions,
    pub(crate) style: Style,
    #[allow(clippy::type_complexity)]
    db_actions: Vec<Pin<Box<dyn Future<Output = Result<(), DbErr>> + Send + 'a>>>,
}

impl std::fmt::Debug for App<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App")
            .field("view", &self.view)
            .field("view_data", &self.view_data)
            .field("popup", &self.popup)
            .field("conn_opts", &self.conn_opts)
            .field("style", &self.style)
            .field("db_actions", &self.db_actions.len())
            .finish()
    }
}

impl App<'_> {
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
            db_actions: Vec::new(),
        })
    }

    /// Handles an input
    pub fn handle_input(&mut self, key: KeyEvent) -> bool {
        if self.popup.is_some() {
            let should_close = {
                let popup_action = self.popup.as_mut().unwrap().handle_input(key);
                let should_close = popup_action.close_popup();
                if let Action::Db(db_action) = popup_action {
                    let future = db_action(self.conn_opts.clone());
                    self.db_actions.push(future);
                }
                should_close
            };
            if should_close {
                self.popup = None;
            }

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
                KeyCode::Char('r') => block_on(self.view_data.refresh(&self.conn_opts)).unwrap(),
                KeyCode::Char(' ') => {
                    todo!("Toggle state")
                }
                KeyCode::Char('e') => {
                    todo!("Edit something")
                }
                _ => (),
            },
        }
        false
    }

    /// blocks on completing each of the pending Database actions
    /// FIXME: This should be possible to be awaited asyncronousely instead
    pub fn run_db_actions(&mut self) -> Result<(), DbErr> {
        while let Some(future) = self.db_actions.pop() {
            block_on(future)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
/// Descripbes the currently highlighted menu outside of popup
pub enum View {
    /// All ideas/suggestions from all the students
    Ideas,
}
