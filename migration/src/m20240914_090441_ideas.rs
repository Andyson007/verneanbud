use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Idea::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Idea::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Idea::Title).string().not_null())
                    .col(ColumnDef::new(Idea::Description).string().not_null())
                    .col(ColumnDef::new(Idea::Author).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Idea::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Idea {
    Table,
    Id,
    Author,
    Title,
    Description,
}
