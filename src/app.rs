//! This module contains everything related to the appstate

use std::{cmp, collections::HashMap, pin::Pin};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use futures::{executor::block_on, Future};
use sea_orm::{ConnectOptions, DbErr};

use crate::{
    popups::{comment::CommontPopup, edit::EditPopup, idea::IdeaPopup, Action, Popup},
    style::Style,
    view_data::{search_query::SearchQuery, ViewData},
};

/// The url of the database. It should be stored:
/// `protocol://username:password@address:port/database_name`
const DATABASE_URL: &str = "postgres://vern:vern@localhost:5432/verneanbud";

/// A future which is intended to modify the DB
pub type DbAction<'a> = Pin<Box<dyn Future<Output = Result<Option<i32>, DbErr>> + Send + 'a>>;
/// This function should be called after the element has been inserted into the Db.
pub type DbActionCallback = Box<dyn FnOnce(&mut ViewData, Option<i32>)>;
/// The return type of something that is going to modify the db
pub type DbActionReturn<'a> = Box<
    dyn FnOnce(&mut ViewData, ConnectOptions) -> Option<(usize, (DbAction<'a>, DbActionCallback))>,
>;

/// The appstruct is responsible for containing all information
/// describing the current state
pub struct App<'a> {
    view: View,
    pub(crate) view_data: ViewData,
    pub(crate) popup: Option<Box<dyn Popup + 'static>>,
    conn_opts: ConnectOptions,
    pub(crate) style: Style,
    #[allow(clippy::type_complexity)]
    db_actions: HashMap<usize, (DbAction<'a>, DbActionCallback)>,
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
            db_actions: HashMap::new(),
        })
    }

    /// Handles an input
    /// true: exit
    /// false: don't exit
    pub fn handle_input(&mut self, key: KeyEvent) -> bool {
        if self.handle_popup(&key) {
            return false;
        }

        if self.view_data.idea.handle_search(&key) {
            return false;
        };

        match self.view {
            View::Ideas => match key {
                KeyEvent {
                    code: KeyCode::Char('d'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Repeat | KeyEventKind::Press,
                    state: KeyEventState::NONE,
                } => {
                    if let Some(x) = self.view_data.idea.current_mut() {
                        x.2 = cmp::min(
                            x.2 + 3,
                            u16::try_from(
                                x.1.iter()
                                    .map(|x| x.get_entry().content.lines().count() + 1)
                                    .sum::<usize>()
                                    + x.0.get_entry().description.lines().count(),
                            )
                            .unwrap(),
                        );
                    }
                }
                KeyEvent {
                    code: KeyCode::Char('u'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Repeat | KeyEventKind::Press,
                    state: KeyEventState::NONE,
                } => {
                    if let Some(x) = self.view_data.idea.current_mut() {
                        x.2 = x.2.saturating_sub(3);
                    }
                }
                key => match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        return true;
                    }
                    KeyCode::Char('j') | KeyCode::Up => self.view_data.idea.down(),
                    KeyCode::Char('k') | KeyCode::Down => self.view_data.idea.up(),
                    KeyCode::Char('n') => self.popup = Some(Box::new(IdeaPopup::default())),
                    KeyCode::Char('r') => {
                        block_on(self.view_data.refresh(&self.conn_opts)).unwrap();
                    }
                    KeyCode::Char('d') => self.delete_idea(),
                    KeyCode::Char('c') => {
                        if self.view_data.idea.selected.is_some() {
                            self.popup = Some(Box::new(CommontPopup::default()));
                        }
                    }
                    KeyCode::Char('/') => {
                        self.view_data.idea.search_query = Some(SearchQuery::new());
                    }

                    KeyCode::Char('e') => {
                        if let Some(x) = self.view_data.idea.current() {
                            self.popup = Some(Box::new(EditPopup::new(x)));
                        }
                    }
                    _ => (),
                },
            },
        }

        false
    }

    fn handle_popup(&mut self, key: &KeyEvent) -> bool {
        let Some(ref mut popup) = self.popup else {
            return false;
        };
        let popup_action = popup.handle_input(key);
        let should_close = popup_action.close_popup();

        if let Action::Db(db_action) = popup_action {
            if let Some((id, (future, callback))) =
                db_action(&mut self.view_data, self.conn_opts.clone())
            {
                self.db_actions.insert(id, (future, callback));
            };
        }

        if should_close {
            self.popup = None;
        };
        true
    }

    fn delete_idea(&mut self) {
        let Some(db_action) = self.view_data.idea.delete() else {
            return;
        };
        let Some((id, db_action)) = db_action(&mut self.view_data, self.conn_opts.clone()) else {
            return;
        };
        self.db_actions.insert(id, db_action);
    }

    /// blocks on completing each of the pending Database actions
    /// FIXME: This should be possible to be awaited asyncronousely instead
    pub fn run_db_actions(&mut self) -> Result<(), DbErr> {
        for (_, (future, callback)) in self.db_actions.drain() {
            let id = block_on(future)?;
            callback(&mut self.view_data, id);
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
