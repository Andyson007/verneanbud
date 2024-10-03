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
    entities::{comment, prelude::Comment},
    popups::Popup,
    style::Style,
    view_data::ViewData,
};

use super::Action;

#[derive(Default, Clone, Debug)]
pub(crate) struct CommontPopup {
    pub(crate) author: String,
    pub(crate) content: String,
    selected: Selected,
}

impl Popup for CommontPopup {
    fn render(&self, style: Style, area: ratatui::prelude::Rect, frame: &mut Frame) {
        frame.render_widget(Clear, area);
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Fill(1)])
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
            .border_style(if matches!(self.selected, Selected::Content) {
                style.highlighted
            } else {
                style.not_highlighted
            })
            .title("Content");
        let para = Paragraph::new(self.content.clone())
            .block(block)
            .wrap(Wrap { trim: false });
        frame.render_widget(para, layout[1]);
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
                KeyCode::Enter if matches!(self.selected, Selected::Author) => {
                    let cloned = self.clone();
                    return Action::Db(Box::new(
                        move |view_data: &mut ViewData, conn_opts: ConnectOptions| {
                            let to_insert = comment::Model {
                                id: -1,
                                author: cloned.author.clone(),
                                content: cloned.content,
                                time: chrono::Local::now().naive_local(),
                                comments_on: view_data.idea.current().unwrap().0.get_entry().id,
                            };

                            let id = view_data
                                .idea
                                .new_comment(view_data.idea.selected.unwrap(), to_insert.clone());

                            let to_insert_active_model = comment::ActiveModel {
                                author: ActiveValue::Set(to_insert.author.clone()),
                                content: ActiveValue::Set(to_insert.content.clone()),
                                time: ActiveValue::Set(to_insert.time),
                                comments_on: ActiveValue::Set(to_insert.comments_on),
                                ..Default::default()
                            };
                            Some((
                                id,
                                (
                                    async move {
                                        let db = Database::connect(conn_opts).await?;

                                        let a = Comment::insert(to_insert_active_model)
                                            .exec(&db)
                                            .await?;

                                        Ok(Some(a.last_insert_id))
                                    }
                                    .boxed(),
                                    Box::new(
                                        move |view_data: &mut ViewData, new_id: Option<i32>| {
                                            let _ = view_data.idea.inserted(
                                                id,
                                                new_id.expect(
                                                    "This method cannot be called with None",
                                                ),
                                            );
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

impl CommontPopup {
    fn get_str_handle(&mut self) -> &mut String {
        match self.selected {
            Selected::Author => &mut self.author,
            Selected::Content => &mut self.content,
        }
    }
}

#[derive(Default, Debug, Clone)]
enum Selected {
    #[default]
    Author,
    Content,
}

impl Selected {
    pub const fn next(&self) -> Self {
        match self {
            Self::Author => Self::Content,
            Self::Content => Self::Author,
        }
    }

    pub const fn prev(&self) -> Self {
        match self {
            Self::Author => Self::Content,
            Self::Content => Self::Author,
        }
    }
}
