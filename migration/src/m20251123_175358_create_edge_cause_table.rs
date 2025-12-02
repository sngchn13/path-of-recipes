use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(EdgeCause::Table)
                    .if_not_exists()
                    .col(integer(EdgeCause::EdgeId))
                    .col(integer(EdgeCause::CauseId))
                    .col(integer(EdgeCause::Sequence))
                    .primary_key(
                        Index::create()
                            .col(EdgeCause::EdgeId)
                            .col(EdgeCause::CauseId)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_edge_id")
                            .from("edge_causes", "edge_id")
                            .to("edges", "id")
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_cause_id")
                            .from("edge_causes", "cause_id")
                            .to("causes", "id")
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_edge_id_sequence")
                    .table(EdgeCause::Table)
                    .col(EdgeCause::EdgeId)
                    .col(EdgeCause::Sequence)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(EdgeCause::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum EdgeCause {
    #[sea_orm(iden = "edge_causes")]
    Table,
    EdgeId,
    CauseId,
    Sequence,
}
