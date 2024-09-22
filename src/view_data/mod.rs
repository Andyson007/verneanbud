use sea_orm::{ConnectOptions, DbErr};

mod idea;

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
        Ok(Self {
            idea: Idea::new(conn_opts).await?,
        })
    }

    pub async fn refresh(&mut self, conn_opts: &ConnectOptions) -> Result<(), DbErr> {
        self.idea = Idea::new(conn_opts).await?;
        Ok(())
    }
}
