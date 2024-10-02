pub use sea_orm_migration::prelude::*;

mod m20240922_075048_create_ideas;
mod m20241002_082310_create_comments;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240922_075048_create_ideas::Migration),
            Box::new(m20241002_082310_create_comments::Migration),
        ]
    }
}
