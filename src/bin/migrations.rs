use futures::executor::block_on;
use sea_orm::{Database, DbErr};
use sea_orm_migration::MigratorTrait;

#[path ="../migrator/mod.rs"]
mod migrator;
use crate::migrator::Migrator;

const DATABASE_URL: &str = "postgres://vern:vern@localhost:5432/verneanbud";

fn main() {
    block_on(run());
}

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    Migrator::refresh(&db).await
}
