use sea_orm_migration::{prelude::{extension::postgres::Type, *}, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager.create_type(
            Type::create()
            .as_enum(Rarity::Table)
            .values([
                Rarity::Normal,
                Rarity::Magic,
                Rarity::Rare,
            ])
            .to_owned(),
        ).await?;

        manager
            .create_table(
                Table::create()
                    .table(Node::Table)
                    .if_not_exists()
                    .col(pk_auto(Node::Id))
                    .col(text(Node::Context))
                    .col(
                        ColumnDef::new(Node::Rarity)
                            .custom(Rarity::Table)
                            .not_null(),
                    )
                    .col(unsigned(Node::ItemLevel))
                    .col(unsigned(Node::Quality).default(0))
                    .col(unsigned(Node::Flags).default(0))
                    .col(unsigned(Node::Influences).default(0))
                    .col(text(Node::Note).null())
                    .col(float(Node::PositionX))
                    .col(float(Node::PositionY))
                    .col(integer(Node::ItemBaseId))
                    .col(integer(Node::GraphId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_item_base_id")
                            .from(Node::Table, Node::ItemBaseId)
                            .to("item_bases", "id")
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_graph_id")
                            .from(Node::Table, Node::GraphId)
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
            .drop_table(Table::drop().table(Node::Table).to_owned())
            .await?;

        manager
            .drop_type(
                Type::drop()
                .name(Rarity::Table)
                .to_owned(),
            ).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Node {
    #[sea_orm(iden = "nodes")]
    Table,
    Id,
    Context,
    Rarity,
    ItemLevel,
    Quality,
    Flags,
    Influences,
    Note,
    PositionX,
    PositionY,
    ItemBaseId,
    GraphId,
}

#[derive(DeriveIden)]
enum Rarity {
    Table,
    Normal,
    Magic,
    Rare,
}
