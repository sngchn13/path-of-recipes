use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Edge::Table)
                    .if_not_exists()
                    .col(pk_auto(Edge::Id))
                    .col(text(Edge::Note).null())
                    .col(integer(Edge::SourceId).null())
                    .col(integer(Edge::TargetId).null())
                    .col(integer(Edge::GraphId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_source_id")
                            .from("edges", "source_id")
                            .to("nodes", "id")
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_target_id")
                            .from("edges", "target_id")
                            .to("nodes", "id")
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_graph_id")
                            .from("edges", "graph_id")
                            .to("graphs", "id")
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
            .drop_table(Table::drop().table(Edge::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Edge {
    #[sea_orm(iden = "edges")]
    Table,
    Id,
    Note,
    SourceId,
    TargetId,
    GraphId,
}
