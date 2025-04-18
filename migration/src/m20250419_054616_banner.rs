use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Banner::Table)
                    .if_not_exists()
                    .col(pk_auto(Banner::Id))
                    .col(text(Banner::Url).not_null())
                    .col(boolean(Banner::IsShow).default(false))
                    .col(string(Banner::LocalPath).string_len(255))
                    .col(date_time(Banner::CreateTime).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
      manager
            .drop_table(Table::drop().table(Banner::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Banner {
    Table,
    Id,
    Url,
    IsShow,
    LocalPath,
    CreateTime,
}
