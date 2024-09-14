//! The main module.
//! implements App and all of its features

use crossterm::event::KeyCode;
use futures::executor::block_on;
use sea_orm::{Database, DatabaseConnection, DbErr};

const DATABASE_URL: &str = "postgres://vern:vern@localhost:5432/verneanbud";
const DB_NAME: &str = "verneanbud";

/// The appstruct is responsible for containing all information
/// describing the current state
#[derive(Debug)]
pub struct App {
    view: View,
    db: DatabaseConnection,
}

impl App {
    #[allow(clippy::missing_errors_doc)]
    /// Creates an app
    ///
    /// Initialized stuff like the db
    pub fn new() -> Result<Self, DbErr> {
        let db = block_on(async { Database::connect(DATABASE_URL).await })?;
        Ok(Self {
            view: View::ToTalk,
            db,
        })
    }
}

impl App {
    /// Handles an input
    pub fn handle_input(&mut self, key: KeyCode) -> bool {
        if matches!(key, KeyCode::Char('q')) {
            return true;
        }
        match self.view {
            View::ToTalk => (),
        }
        false
    }
}

#[derive(Debug)]
enum View {
    ToTalk,
}
