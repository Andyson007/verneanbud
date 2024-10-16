use core::panic;
use crossterm::event::KeyEvent;
use futures::FutureExt;
use sea_orm::{ColumnTrait, ConnectOptions, Database, DbErr, EntityTrait, QueryFilter, QueryOrder};
use std::{cmp, sync::Arc};

use crate::{
    app::DbActionReturn,
    entities::{
        comment,
        idea::{self},
        prelude::{Comment as eComment, Idea as eIdea},
    },
};

use super::{counter::Counter, db_type::DbType, search_query::SearchQuery, ViewData};

/// 0: The idea description
/// 1: The comments on the idea
/// 2: Scroll distance
pub type IdeaType = (DbType<idea::Model>, Vec<DbType<comment::Model>>, u16);

#[derive(Debug)]
pub struct Idea {
    pub selected: Option<usize>,
    ideas: Vec<IdeaType>,
    pub search_query: Option<SearchQuery>,
    counter: Arc<Counter>,
}

impl Idea {
    pub async fn new(conn_opts: &ConnectOptions, counter: Arc<Counter>) -> Result<Self, DbErr> {
        let db = Database::connect(conn_opts.clone()).await?;
        let ideas = eIdea::find()
            .find_with_related(comment::Entity)
            .order_by_asc(idea::Column::Time)
            .all(&db)
            .await?
            .into_iter()
            .map(|(a, b)| {
                let mut b: Vec<_> = b.into_iter().map(DbType::InDb).collect();
                b.sort_by_key(|x| x.get_entry().time);
                (DbType::InDb(a), b, 0)
            })
            .collect();
        Ok(Self {
            ideas,
            counter,
            selected: None,
            search_query: None,
        })
    }

    pub fn filtered_ideas(&self) -> impl DoubleEndedIterator<Item = &idea::Model> + Clone {
        self.ideas.iter().map(|x| x.0.get_entry()).filter(|x| {
            self.search_query.as_ref().map_or(true, |search_query| {
                x.title
                    .to_lowercase()
                    .starts_with(&search_query.to_string().to_lowercase())
            })
        })
    }

    pub fn up(&mut self) {
        if self.filtered_ideas().next().is_none() {
            self.selected = None;
        } else {
            self.selected = Some(
                self.selected
                    .map_or(self.filtered_ideas().count() - 1, |x| x + 1)
                    % self.filtered_ideas().count(),
            );
        }
    }

    pub fn down(&mut self) {
        if self.filtered_ideas().next().is_none() {
            self.selected = None;
        } else {
            self.selected = Some(
                self.selected
                    .map_or(self.filtered_ideas().count() - 1, |x| {
                        (x + self.filtered_ideas().count() - 1) % self.filtered_ideas().count()
                    })
                    % self.filtered_ideas().count(),
            );
        }
    }

    pub fn edit_idea(&mut self, idea: &idea::Model) -> Option<usize> {
        let Some(counter) = Arc::get_mut(&mut self.counter) else {
            panic!("I don't even know how.")
        };

        let entry = self
            .ideas
            .iter_mut()
            .find(|x| x.0.get_entry().id == idea.id)?;
        if entry.0.convert_to_db_action(counter.next()).is_err() {
            return None;
        }
        Some(self.counter.get())
    }

    pub fn new_idea(&mut self, idea: idea::Model) -> usize {
        let Some(counter) = Arc::get_mut(&mut self.counter) else {
            panic!("I don't even know how.")
        };
        self.ideas
            .push((DbType::new_future(counter.next(), idea), Vec::new(), 0));
        self.counter.get()
    }

    pub fn new_comment(&mut self, comments_on: usize, comment: comment::Model) -> usize {
        let Some(counter) = Arc::get_mut(&mut self.counter) else {
            panic!("I don't even know how.")
        };

        self.ideas[comments_on]
            .1
            .push(DbType::new_future(counter.next(), comment));

        self.counter.get()
    }

    /// Converts a pendic Db-action to to a DB element by id
    pub fn completed<C>(&mut self, id: usize, callback: C) -> Result<(), ()>
    where
        C: FnOnce(&mut IdeaType),
    {
        let Some(x) = self
            .ideas
            .iter_mut()
            .find(|x| matches!(x.0, DbType::DbActionPending(dbid, _) if dbid == id))
        else {
            return Err(());
        };
        callback(x);
        x.0.convert_to_db();

        Ok(())
    }

    #[allow(clippy::needless_pass_by_ref_mut, clippy::unused_async)]
    pub async fn refresh(&mut self, _conn_opts: &ConnectOptions) {
        todo!("This should refresh the Databases inside of here")
    }

    pub fn delete<'a>(&self) -> Option<DbActionReturn<'a>> {
        let selected = self.selected?;

        let DbType::InDb(idea::Model { id, .. }) = self.ideas[selected].0 else {
            return None;
        };

        Some(Box::new(
            move |view_data: &mut ViewData, conn_opts: ConnectOptions| {
                let idea = view_data.idea.ideas.iter_mut().find(
                    |x| matches!(x.0, DbType::InDb(idea::Model {id: model_id, ..}) if id == model_id)
                )?;

                let Some(counter) = Arc::get_mut(&mut view_data.idea.counter) else {
                    panic!()
                };
                let action_id = counter.next();
                idea.0.convert_to_db_action(action_id).unwrap();

                Some((
                    action_id,
                    (
                        async move {
                            let db = Database::connect(conn_opts).await?;

                            eComment::delete_many()
                                .filter(comment::Column::CommentsOn.eq(id))
                                .exec(&db)
                                .await?;
                            eIdea::delete_by_id(id).exec(&db).await?;

                            Ok(None)
                        }
                        .boxed(),
                        Box::new(move |view_data: &mut ViewData, new_id: Option<_>| {
                            assert!(
                                new_id.is_none(),
                                "There is probably a bug, this shouldn't be called with Some"
                            );
                            let Some(pos) = view_data.idea.ideas.iter_mut().position(
                                |x| matches!(x.0, DbType::DbActionPending(id, _) if action_id == id)
                            ) else {
                                return;
                            };

                            if view_data.idea.ideas.len() == 1 {
                                view_data.idea.selected = None;
                            }

                            if view_data.idea.ideas.len() == 1 {
                                view_data.idea.selected = None;
                            } else if let Some(ref mut selected) = view_data.idea.selected {
                                if *selected > pos {
                                    *selected -= 1;
                                }
                                // Just in case so that some weird behavior doesn't crash it
                                if *selected == view_data.idea.ideas.len() - 1 {
                                    *selected -= 1;
                                }
                            }
                            let _ = view_data.idea.ideas.remove(pos);
                        }),
                    ),
                ))
            },
        ))
    }

    pub fn handle_search(&mut self, key: &KeyEvent) -> bool {
        let Some(search_query) = self.search_query.as_mut() else {
            return false;
        };
        if !search_query.focused {
            return false;
        }
        if search_query.handle_input(key) {
            self.search_query = None;
            return true;
        }

        self.selected.map(|_| {
            let amount = self.filtered_ideas().count();
            if amount == 0 {
                self.selected = None;
            } else if let Some(ref mut selected) = self.selected {
                *selected = cmp::min(*selected, amount - 1);
            };
            false
        });

        false
    }

    pub fn current(&self) -> Option<&IdeaType> {
        Some(&self.ideas[self.selected?])
    }

    pub fn current_mut(&mut self) -> Option<&mut IdeaType> {
        Some(&mut self.ideas[self.selected?])
    }
}
