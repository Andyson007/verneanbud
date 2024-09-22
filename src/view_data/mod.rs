use sea_orm::{ConnectOptions, DbErr};
use std::rc::Rc;

mod counter;
mod idea;

use counter::Counter;
use idea::Idea;
// use crate::entities::{idea, prelude::Idea as eIdea};

/// `ViewData` is used to store information that is neccessary for the
/// UI navigation to function
#[derive(Debug)]
pub struct ViewData {
    pub idea: Idea,
}

impl ViewData {
    pub async fn new(conn_opts: &ConnectOptions) -> Result<Self, DbErr> {
        let counter = Rc::new(Counter::default());
        Ok(Self {
            idea: Idea::new(conn_opts, Rc::clone(&counter)).await?,
        })
    }

    pub async fn refresh(&mut self, conn_opts: &ConnectOptions) -> Result<(), DbErr> {
        self.idea.refresh(conn_opts);
        Ok(())
    }
}
