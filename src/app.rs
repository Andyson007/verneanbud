//! The main module.
//! implements App and all of its features

use crossterm::event::KeyCode;

/// The appstruct is responsible for containing all information
/// describing the current state
#[derive(Debug)]
pub struct App {
    view: View,
}

impl Default for App {
    fn default() -> Self {
        Self { view: View::ToTalk }
    }
}

impl App {
    /// Handles an input
    pub fn handle_input(&mut self, key: KeyCode) -> bool {
        if matches!(key, KeyCode::Char('q')) {
            return true;
        }
        match self.view {
            View::ToTalk => self.,
        }
        false
    }
}

#[derive(Debug)]
enum View {
    ToTalk,
}
