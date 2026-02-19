use sea_orm_migration::{prelude::{extension::postgres::Type, *}, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager.create_type(
            Type::create()
            .as_enum(ReactionType::Table)
            .values([
                ReactionType::Like,
                ReactionType::Dislike,
            ])
            .to_owned(),
        ).await?;

        manager
            .create_table(
                Table::create()
                    .table(Reaction::Table)
                    .if_not_exists()
                    .col(pk_auto(Reaction::Id))
                    .col(string(Reaction::Subject))
                    .col(uuid(Reaction::AnonId))
                    .col(ColumnDef::new(Reaction::ReactionType)
                        .custom(ReactionType::Table)
                        .not_null()
                    )
                    .col(timestamp_with_time_zone(Reaction::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone_null(Reaction::UpdatedAt))
                    .index(
                        Index::create()
                            .name("uq_reactions_subject_anon")
                            .unique()
                            .col(Reaction::Subject)
                            .col(Reaction::AnonId)
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Reaction::Table).to_owned())
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .name(ReactionType::Table)
                    .to_owned()
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ReactionType {
    Table,
    Like,
    Dislike,
}

#[derive(DeriveIden)]
enum Reaction {
    #[sea_orm(iden = "reactions")]
    Table,
    Id,
    Subject,
    AnonId,
    ReactionType,
    CreatedAt,
    UpdatedAt,
}
