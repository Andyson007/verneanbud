//! The popup that appears when you want to insert a new idea into the db
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use futures::FutureExt;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};
use sea_orm::{
    sqlx::types::chrono, ActiveValue, ColumnTrait, ConnectOptions, Database, EntityTrait,
    QueryFilter,
};

use crate::{
    entities::{idea, prelude::Idea, sea_orm_active_enums::Issuekind},
    popups::Popup,
    style::Style,
    view_data::{idea::IdeaType, ViewData},
};

use super::Action;

#[derive(Default, Clone, Debug)]
pub(crate) struct EditPopup {
    pub(crate) author: String,
    pub(crate) title: String,
    pub(crate) description: String,
    selected: Selected,
    id: i32,
}

impl Popup for EditPopup {
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

        let para = Paragraph::new(self.author.clone())
            .block(block)
            .wrap(Wrap { trim: false });
        frame.render_widget(para, layout[0]);

        let block = base_block
            .clone()
            .border_style(if matches!(self.selected, Selected::Title) {
                style.highlighted
            } else {
                style.not_highlighted
            })
            .title("Title");
        let para = Paragraph::new(self.title.clone())
            .block(block)
            .wrap(Wrap { trim: false });
        frame.render_widget(para, layout[1]);

        let block = base_block
            .clone()
            .border_style(if matches!(self.selected, Selected::Description) {
                style.highlighted
            } else {
                style.not_highlighted
            })
            .title("Description");
        let para = Paragraph::new(self.description.clone())
            .block(block)
            .wrap(Wrap { trim: false });
        frame.render_widget(para, layout[2]);
    }

    fn handle_input<'a>(&mut self, key: &crossterm::event::KeyEvent) -> Action<'a> {
        match key {
            KeyEvent {
                code: KeyCode::Char('w'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                *self.get_str_handle() = self
                    .get_str_handle()
                    .rsplit_once(' ')
                    .map_or(String::new(), |x| x.0.to_string());
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
                    let id = self.id;
                    return Action::Db(Box::new(
                        move |view_data: &mut ViewData, conn_opts: ConnectOptions| {
                            let to_insert = idea::Model {
                                id,
                                title: cloned.title.clone(),
                                description: cloned.description.clone(),
                                author: cloned.author.clone(),
                                solved: false,
                                kind,
                                time: chrono::Local::now().naive_local(),
                            };
                            let action_id = view_data.idea.edit_idea(&to_insert)?;

                            let to_insert_active_model = idea::ActiveModel {
                                title: ActiveValue::Set(to_insert.title.clone()),
                                description: ActiveValue::Set(to_insert.description.clone()),
                                author: ActiveValue::Set(to_insert.author.clone()),
                                solved: ActiveValue::Set(to_insert.solved),
                                kind: ActiveValue::Set(to_insert.kind),
                                id: ActiveValue::Unchanged(to_insert.id),
                                ..Default::default()
                            };
                            Some((
                                action_id,
                                (
                                    async move {
                                        let db = Database::connect(conn_opts).await?;

                                        Idea::update(to_insert_active_model)
                                            .filter(idea::Column::Id.eq(id))
                                            .exec(&db)
                                            .await?;
                                        Ok(None)
                                    }
                                    .boxed(),
                                    Box::new(
                                        move |view_data: &mut ViewData, new_id: Option<i32>| {
                                            assert!(new_id.is_none());
                                            let _ = view_data.idea.completed(action_id, |x| {
                                                let entry = x.0.get_entry_mut();
                                                entry.author = to_insert.author;
                                                entry.title = to_insert.title;
                                                entry.description = to_insert.description;
                                            });
                                        },
                                    ),
                                ),
                            ))
                        },
                    ));
                }
                KeyCode::Enter => self.get_str_handle().push('\n'),
                _ => (),
            },
        }
        Action::Nothing
    }
}

impl EditPopup {
    pub fn new(previous: &IdeaType) -> Self {
        let entry = previous.0.get_entry();
        Self {
            author: entry.author.clone(),
            title: entry.title.clone(),
            description: entry.description.clone(),
            selected: Selected::Author,
            id: entry.id,
        }
    }

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
    pub const fn next(&self) -> Self {
        match self {
            Self::Author => Self::Title,
            Self::Title => Self::Description,
            Self::Description => Self::Author,
        }
    }

    pub const fn prev(&self) -> Self {
        match self {
            Self::Author => Self::Description,
            Self::Title => Self::Author,
            Self::Description => Self::Title,
        }
    }
}
