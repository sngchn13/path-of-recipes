use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ItemBase::Table)
                    .if_not_exists()
                    .col(pk_auto(ItemBase::Id))
                    .col(string(ItemBase::DisplayEn).null())
                    .col(string(ItemBase::DisplayKor).null())
                    .col(integer(ItemBase::ItemTypeId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_item_type_id")
                            .from("item_types", "item_type_id")
                            .to("item_bases", "id")
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned()
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ItemBase::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ItemBase {
    #[sea_orm(iden = "item_bases")]
    Table,
    Id,
    DisplayEn,
    DisplayKor,
    ItemTypeId,
}