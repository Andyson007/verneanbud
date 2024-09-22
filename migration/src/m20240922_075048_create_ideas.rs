use sea_orm::{ActiveEnum, DbBackend, DeriveActiveEnum, EnumIter, Schema};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = Schema::new(DbBackend::Postgres);
        manager
            .create_type(schema.create_enum_from_active_enum::<Kind>())
            .await?;
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
                    .col(ColumnDef::new(Idea::Solved).boolean().not_null())
                    .col(ColumnDef::new(Idea::Kind).custom(Kind::name()).not_null())
                    .col(ColumnDef::new(Idea::Time).timestamp().not_null())
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
    Solved,
    Kind,
    Time,
}

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "issuekind")]
enum Kind {
    #[sea_orm(string_value = "Issue")]
    Issue,
    #[sea_orm(string_value = "Improvement")]
    Improvement,
}
