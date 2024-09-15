use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::{popups::Popup, style::Style};

#[derive(Default)]
pub struct IdeaPopup {
    pub(crate) author: String,
    pub(crate) title: String,
    pub(crate) description: String,
    selected: Selected,
}

impl Popup for IdeaPopup {
    fn debug_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }

    fn render(&self, style: Style, area: ratatui::prelude::Rect, frame: &mut Frame) {
        frame.render_widget(Clear, area);
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Fill(1),
            ])
            .split(area);
        let base_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let block = base_block
            .clone()
            .border_style(if matches!(self.selected, Selected::Author) {
                style.highlighted
            } else {
                style.not_highlighted
            })
            .title("Author");
        let para = Paragraph::new(self.author.clone()).block(block);
        frame.render_widget(para, layout[0]);

        let block = base_block
            .clone()
            .border_style(if matches!(self.selected, Selected::Title) {
                style.highlighted
            } else {
                style.not_highlighted
            })
            .title("Title");
        let para = Paragraph::new(self.title.clone()).block(block);
        frame.render_widget(para, layout[1]);

        let block = base_block
            .clone()
            .border_style(if matches!(self.selected, Selected::Description) {
                style.highlighted
            } else {
                style.not_highlighted
            })
            .title("Description");
        let para = Paragraph::new(self.description.clone()).block(block);
        frame.render_widget(para, layout[2]);
    }

    fn handle_input(&mut self, key: crossterm::event::KeyCode) -> bool {
        match key {
            KeyCode::Esc => return true,
            KeyCode::Tab => self.selected = self.selected.next(),
            KeyCode::BackTab => self.selected = self.selected.prev(),
            KeyCode::Backspace => {
                match self.selected {
                    Selected::Author => self.author.pop(),
                    Selected::Title => self.title.pop(),
                    Selected::Description => self.description.pop(),
                };
            }
            KeyCode::Char(c) => {
                match self.selected {
                    Selected::Author => self.author.push(c),
                    Selected::Title => self.title.push(c),
                    Selected::Description => self.description.push(c),
                };
            }
            _ => (),
        }
        false
    }
}

#[derive(Default, Debug)]
enum Selected {
    #[default]
    Author,
    Title,
    Description,
}

impl Selected {
    pub fn next(&self) -> Self {
        match self {
            Self::Author => Self::Title,
            Self::Title => Self::Description,
            Self::Description => Self::Author,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Self::Author => Self::Description,
            Self::Title => Self::Author,
            Self::Description => Self::Title,
        }
    }
}
