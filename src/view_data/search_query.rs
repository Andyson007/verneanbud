use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

#[derive(Debug)]
pub struct SearchQuery {
    data: String,
    pub focused: bool,
}

impl SearchQuery {
    pub const fn new() -> Self {
        Self {
            data: String::new(),
            focused: true,
        }
    }
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchQuery {
    pub fn handle_input(&mut self, key: KeyEvent) -> bool {
        match key {
            KeyEvent {
                code: KeyCode::Char('w'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press | KeyEventKind::Repeat,
                state: _,
            } => {
                if self.data.is_empty() {
                    true
                } else {
                    self.data = self
                        .data
                        .rsplit_once(' ')
                        .map_or(String::new(), |x| x.0.to_string());
                    false
                }
            }
            other => match other.code {
                KeyCode::Enter => {
                    self.focused = false;
                    false
                }
                KeyCode::Esc => true,
                KeyCode::Backspace => {
                    if self.data.is_empty() {
                        true
                    } else {
                        self.data.pop();
                        false
                    }
                }
                KeyCode::Char(c) => {
                    self.data.push(c);
                    false
                }
                _ => false,
            },
        }
    }

    pub fn to_string(&self) -> &str {
        &self.data
    }
}
