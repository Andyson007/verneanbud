//! The popup that appears when you want to insert a new idea into the db
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use futures::FutureExt;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};
use sea_orm::{sqlx::types::chrono, ActiveValue, ConnectOptions, Database, EntityTrait};

use crate::{
    entities::{idea, prelude::Idea, sea_orm_active_enums::Issuekind},
    popups::Popup,
    style::Style,
    view_data::ViewData,
};

use super::Action;

#[derive(Default, Clone, Debug)]
pub(crate) struct IdeaPopup {
    pub(crate) author: String,
    pub(crate) title: String,
    pub(crate) description: String,
    selected: Selected,
}

impl Popup for IdeaPopup {
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

    fn handle_input<'a>(&mut self, key: crossterm::event::KeyEvent) -> Action<'a> {
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
                    return Action::Db(Box::new(
                        move |view_data: &mut ViewData, conn_opts: ConnectOptions| {
                            let to_insert = idea::Model {
                                id: 0,
                                title: cloned.title.clone(),
                                description: cloned.description.clone(),
                                author: cloned.author.clone(),
                                solved: false,
                                kind,
                                time: chrono::Local::now().naive_local(),
                            };
                            let id = view_data.idea.new_future(to_insert.clone());

                            let to_insert_active_model = idea::ActiveModel {
                                title: ActiveValue::Set(to_insert.title.clone()),
                                description: ActiveValue::Set(to_insert.description.clone()),
                                author: ActiveValue::Set(to_insert.author.clone()),
                                solved: ActiveValue::Set(to_insert.solved),
                                kind: ActiveValue::Set(to_insert.kind),
                                time: ActiveValue::Set(to_insert.time),
                                ..Default::default()
                            };
                            Some((
                                id,
                                (
                                    async move {
                                        let db = Database::connect(conn_opts).await?;

                                        Idea::insert(to_insert_active_model).exec(&db).await?;

                                        Ok(())
                                    }
                                    .boxed(),
                                    Box::new(move |view_data: &mut ViewData| {
                                        let _ = view_data.idea.inserted(id);
                                    }),
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
