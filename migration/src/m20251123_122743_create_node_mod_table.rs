use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(NodeMod::Table)
                    .if_not_exists()
                    .col(integer(NodeMod::NodeId))
                    .col(integer(NodeMod::ModId))
                    .col(string(NodeMod::PlacementTypeId))
                    .primary_key(
                        Index::create()
                            .col(NodeMod::NodeId)
                            .col(NodeMod::ModId)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_node_id")
                            .from("node_mods", "node_id")
                            .to("nodes", "id")
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_mod_id")
                            .from("node_mods", "mod_id")
                            .to("mods", "id")
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_placement_type_id")
                            .from("node_mods", "placement_type_id")
                            .to("placement_types", "id")
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
            .drop_table(Table::drop().table(NodeMod::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum NodeMod {
    #[sea_orm(iden = "node_mods")]
    Table,
    NodeId,
    ModId,
    PlacementTypeId,
}
