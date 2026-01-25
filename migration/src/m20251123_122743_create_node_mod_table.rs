use sea_orm_migration::{prelude::{extension::postgres::Type, *}, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        //enum타입 생성
        manager.create_type(
            Type::create()
            .as_enum(Placement::Table)
            .values([
                Placement::Implicit,
                Placement::Explicit,
                Placement::Enchantment,
                Placement::Others,
            ])
            .to_owned(),
        )
        .await?;

        manager
            .create_table(
                Table::create()
                    .table(NodeMod::Table)
                    .if_not_exists()
                    .col(integer(NodeMod::NodeId))
                    .col(integer(NodeMod::ModId))
                    .col(
                        ColumnDef::new(NodeMod::Placement)
                            .custom(Placement::Table)
                            .not_null(),
                    )
                    .col(boolean(NodeMod::Fractured).default(false))
                    .primary_key(
                        Index::create()
                            .col(NodeMod::NodeId)
                            .col(NodeMod::ModId)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_node_id")
                            .from(NodeMod::Table, NodeMod::NodeId)
                            .to("nodes", "id")
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_mod_id")
                            .from(NodeMod::Table, NodeMod::ModId)
                            .to("mods", "id")
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

        manager
            .drop_type(Type::drop().name(Placement::Table).to_owned()).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum NodeMod {
    #[sea_orm(iden = "node_mods")]
    Table,
    NodeId,
    ModId,
    Placement,
    Fractured,
}

#[derive(DeriveIden)]
pub enum Placement {
    Table,
    Implicit,
    Explicit,
    Enchantment,
    Others,
}
