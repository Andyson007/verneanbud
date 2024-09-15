use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use sea_orm::{ActiveValue, ConnectOptions, Database, EntityTrait};

use crate::{
    entities::{idea, prelude::Idea, sea_orm_active_enums::Issuekind},
    popups::Popup,
    style::Style,
};

use super::Action;

#[derive(Default, Clone)]
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

    fn handle_input(&mut self, key: crossterm::event::KeyEvent) -> Action {
        match key {
            KeyEvent {
                code: KeyCode::Char('w'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                *self.get_str_handle() = self
                    .get_str_handle()
                    .rsplit_once(' ')
                    .map_or(String::new(), |x| x.0.to_string())
            }
            x => match x.code {
                KeyCode::Esc => return Action::Close,
                KeyCode::Tab => self.selected = self.selected.next(),
                KeyCode::BackTab => self.selected = self.selected.prev(),
                KeyCode::Backspace => drop(self.get_str_handle().pop()),
                KeyCode::Char(c) => self.get_str_handle().push(c),
                KeyCode::Enter if matches!(self.selected, Selected::Title) => {
                    let kind = Issuekind::Issue;
                    let cloned = self.clone();
                    return Action::Db(Box::new(|conn_opts: ConnectOptions| {
                        Box::pin(async move {
                            let to_insert = idea::ActiveModel {
                                title: ActiveValue::Set(cloned.title.clone()),
                                description: ActiveValue::Set(cloned.description.clone()),
                                author: ActiveValue::Set(cloned.author.clone()),
                                solved: ActiveValue::Set(false),
                                kind: ActiveValue::Set(kind),
                                ..Default::default()
                            };

                            let db = Database::connect(conn_opts).await?;

                            Idea::insert(to_insert).exec(&db).await?;

                            Ok(())
                        })
                    }));
                }
                _ => (),
            },
        }
        Action::Nothing
    }
}

impl IdeaPopup {
    fn get_str_handle(&mut self) -> &mut String {
        match self.selected {
            Selected::Author => &mut self.author,
            Selected::Title => &mut self.title,
            Selected::Description => &mut self.description,
        }
    }
}

#[derive(Default, Debug, Clone)]
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
