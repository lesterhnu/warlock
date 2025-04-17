use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Uploads::Table)
                    .if_not_exists()
                    .col(pk_auto(Uploads::Id))
                    .col(string(Uploads::FileName).string_len(255))
                    .col(string(Uploads::FileExtName).string_len(255))
                    .col(string(Uploads::FileSourceName).string_len(255))
                    .col(date_time(Uploads::CreateTime).default(Expr::current_timestamp()))
                    .index(Index::create().name("idx_uploads_file_name").col(Uploads::FileName))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {


        manager
            .drop_table(Table::drop().table(Uploads::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Uploads {
    Table,
    Id,
    FileName,
    FileExtName,
    FileSourceName,
    CreateTime,
}
