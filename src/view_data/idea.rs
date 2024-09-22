use sea_orm::{ConnectOptions, Database, DbErr};
use sea_orm::{EntityTrait, QueryOrder};
use std::ops::{Index, IndexMut};
use std::rc::Rc;

use crate::entities::{idea, prelude::Idea as eIdea};

use super::counter::Counter;

#[derive(Debug)]
pub struct Idea {
    pub selected: Option<usize>,
    ideas: Vec<IdeaType>,
    counter: Rc<Counter>,
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
    pub async fn new(conn_opts: &ConnectOptions, counter: Rc<Counter>) -> Result<Self, DbErr> {
        let db = Database::connect(conn_opts.clone()).await?;
        let ideas = eIdea::find()
            .order_by_asc(idea::Column::Time)
            .all(&db)
            .await?
            .into_iter()
            .map(IdeaType::InDb)
            .collect();
        Ok(Self {
            selected: None,
            ideas,
            counter,
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
        let Some(counter) = Rc::get_mut(&mut self.counter) else {
            panic!("I don't even know how.")
        };
        self.ideas.push(IdeaType::new_future(counter, idea));
        self.counter.get()
    }

    pub fn iter(
        &self,
    ) -> std::iter::Map<std::slice::Iter<IdeaType>, fn(&IdeaType) -> &idea::Model> {
        self.ideas.iter().map(IdeaType::get_entry)
    }

    pub fn inserted(&mut self, id: usize) -> Result<(), ()> {
        match self
            .ideas
            .iter_mut()
            .find(|x| matches!(x, IdeaType::NotInDbYet(dbid, _) if *dbid == id))
        {
            None => return Err(()),
            Some(x) => x,
        }
        .convert_to_db();
        Ok(())
    }

    pub fn refresh(&mut self,  _conn_opts: &ConnectOptions) {
        todo!("This should refresh the Databases inside of here")
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
    pub fn get_entry(&self) -> &idea::Model {
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

    /// Converts self to a database entry. 
    /// This happens unchecked and the id associated with it will be forgotten
    pub fn convert_to_db(&mut self) {
        if let Self::NotInDbYet(_, x) = self {
            *self = Self::InDb(x.clone());
        }
    }

    fn new_future(counter: &mut Counter, idea: idea::Model) -> Self {
        Self::NotInDbYet(counter.next(), idea)
    }
}
