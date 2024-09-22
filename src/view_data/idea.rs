use sea_orm::{ConnectOptions, Database, DbErr};
use sea_orm::{EntityTrait, QueryOrder};
use std::ops::{Index, IndexMut};

use crate::entities::{idea, prelude::Idea as eIdea};

#[derive(Debug)]
pub struct Idea {
    pub selected: Option<usize>,
    ideas: Vec<IdeaType>,
    idea_counter: Counter,
}

impl Index<usize> for Idea {
    type Output = idea::Model;

    fn index(&self, idx: usize) -> &Self::Output {
        self.ideas[idx].get_entry()
    }
}

impl IndexMut<usize> for Idea {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.ideas[index].get_entry_mut()
    }
}

impl Idea {
    pub async fn new(conn_opts: &ConnectOptions) -> Result<Self, DbErr> {
        let db = Database::connect(conn_opts.clone()).await?;
        let ideas = eIdea::find()
            .order_by_desc(idea::Column::Time)
            .all(&db)
            .await?
            .into_iter()
            .map(IdeaType::InDb)
            .collect();
        Ok(Self {
            selected: None,
            ideas,
            idea_counter: Counter::default(),
        })
    }

    pub fn up(&mut self) {
        if self.ideas.is_empty() {
            self.selected = None;
        } else {
            self.selected = Some(self.selected.map_or(0, |x| x + 1) % self.ideas.len());
        }
    }

    pub fn down(&mut self) {
        if self.ideas.is_empty() {
            self.selected = None;
        } else {
            self.selected =
                Some(self.selected.map_or(0, |x| x + self.ideas.len() - 1) % self.ideas.len());
        }
    }

    pub fn new_future(&mut self, idea: idea::Model) -> usize {
        self.ideas
            .push(IdeaType::new_future(&mut self.idea_counter, idea));
        self.idea_counter.get()
    }

    pub fn iter(
        &self,
    ) -> std::iter::Map<std::slice::Iter<IdeaType>, fn(&IdeaType) -> &idea::Model> {
        self.ideas.iter().map(IdeaType::get_entry)
    }
}

#[derive(Debug)]
pub enum IdeaType {
    /// For entries already in the db
    InDb(idea::Model),
    /// For futures that are currently awaited to be pushed
    NotInDbYet(usize, idea::Model),
}

impl IdeaType {
    pub fn get_entry<'a>(&'a self) -> &'a idea::Model {
        match self {
            IdeaType::InDb(ref x) => x,
            IdeaType::NotInDbYet(_, ref x) => x,
        }
    }

    pub fn get_entry_mut(&mut self) -> &mut idea::Model {
        match self {
            IdeaType::InDb(ref mut x) => x,
            IdeaType::NotInDbYet(_, ref mut x) => x,
        }
    }

    pub fn to_db(&mut self) {
        if let Self::NotInDbYet(_, x) = self {
            *self = Self::InDb(x.clone());
        }
    }

    fn new_future(counter: &mut Counter, idea: idea::Model) -> Self {
        Self::NotInDbYet(counter.next(), idea)
    }
}

#[derive(Debug)]
struct Counter {
    counter: usize,
}

impl Default for Counter {
    fn default() -> Self {
        Self { counter: 0 }
    }
}

impl Counter {
    fn next(&mut self) -> usize {
        self.counter += 1;
        self.counter - 1
    }

    pub fn get(&self) -> usize {
        self.counter
    }
}
