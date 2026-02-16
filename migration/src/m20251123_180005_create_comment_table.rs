use sea_orm_migration::{prelude::{extension::postgres::Type, *}, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        
        manager.create_type(
            Type::create()
            .as_enum(Status::Table)
            .values([
                Status::Active,
                Status::Deleted,
                Status::Hidden,
            ])
            .to_owned(),
        ).await?;

        manager
            .create_table(
                Table::create()
                    .table(Comment::Table)
                    .if_not_exists()
                    .col(pk_auto(Comment::Id))
                    .col(uuid(Comment::PostId))
                    .col(uuid(Comment::AuthorId))
                    .col(string_null(Comment::AuthorName))
                    .col(string(Comment::Password))
                    .col(integer_null(Comment::ParentId))
                    .col(text(Comment::Content))
                    .col(ColumnDef::new(Comment::Status)
                            .custom(Status::Table)
                            .not_null()
                        )
                    .col(integer(Comment::LikeCount).default(0))
                    .col(integer(Comment::DislikeCount).default(0))
                    .col(integer(Comment::ReplyCount).default(0))
                    .col(timestamp_with_time_zone(Comment::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone_null(Comment::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_id")
                            .from(Comment::Table, Comment::PostId)
                            .to("posts", "id")
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_parent_id")
                            .from(Comment::Table, Comment::ParentId)
                            .to(Comment::Table, Comment::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Comment::Table).to_owned())
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .name(Status::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}


#[derive(DeriveIden)]
enum Status {
    Table,
    Active,
    Deleted,
    Hidden,
}

#[derive(DeriveIden)]
enum Comment {
    #[sea_orm(iden = "comments")]
    Table,
    Id,
    PostId,
    AuthorId,
    AuthorName,
    Password,
    ParentId,
    Content,
    Status,
    LikeCount,
    DislikeCount,
    ReplyCount,
    CreatedAt,
    UpdatedAt,
}
