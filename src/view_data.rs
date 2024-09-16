use sea_orm::{ConnectOptions, Database, DbErr, EntityTrait};

use crate::entities::{idea, prelude::Idea as eIdea};

#[derive(Debug)]
pub struct ViewData {
    pub idea: Idea,
}

impl ViewData {
    pub async fn new(conn_opts: &ConnectOptions) -> Result<Self, DbErr> {
        Ok(Self {
            idea: Idea::new(conn_opts).await?,
        })
    }
}

#[derive(Debug)]
pub struct Idea {
    pub selected: Option<usize>,
    pub(crate) ideas: Vec<idea::Model>,
}

impl Idea {
    async fn new(conn_opts: &ConnectOptions) -> Result<Self, DbErr> {
        let db = Database::connect(conn_opts.clone()).await?;
        let ideas = eIdea::find().all(&db).await?;
        Ok(Self {
            selected: None,
            ideas,
        })
    }
}

impl Idea {
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
}
