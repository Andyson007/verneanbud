use sea_orm_migration::prelude::*;

use crate::m20240922_075048_create_ideas::Idea;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Comment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Comment::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Comment::Author).string().not_null())
                    .col(ColumnDef::new(Comment::Time).timestamp().not_null())
                    .col(ColumnDef::new(Comment::Content).string().not_null())
                    .col(ColumnDef::new(Comment::CommentsOn).unsigned().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-comment-idea")
                            .from(Comment::Table, Comment::CommentsOn)
                            .to(Idea::Table, Idea::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Comment::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Comment {
    Table,
    Id,
    Time,
    Author,
    Content,
    CommentsOn,
}
