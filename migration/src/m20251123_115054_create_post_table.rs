use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(pk_uuid(Post::Id))
                    .col(string(Post::Title))
                    .col(text(Post::Content))
                    .col(string(Post::Password))
                    .col(integer(Post::View).default(1))
                    .col(integer(Post::Like).default(0))
                    .col(integer(Post::Dislike).default(0))
                    .col(timestamp_with_time_zone(Post::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone_null(Post::UpdatedAt))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Post {
    #[sea_orm(iden = "posts")]
    Table,
    Id,
    Title,
    Content,
    Password,
    View,
    Like,
    Dislike,
    CreatedAt,
    UpdatedAt,
}
