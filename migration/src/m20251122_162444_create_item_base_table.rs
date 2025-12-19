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
                    .col(string(ItemBase::Name).unique_key())
                    .col(string(ItemBase::DisplayEn).null())
                    .col(string(ItemBase::DisplayKor).null())
                    .col(integer(ItemBase::ItemTypeId))
                    .col(uuid(ItemBase::ImageId).null()) //todo null삭제
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_item_type_id")
                            .from(ItemBase::Table, ItemBase::ItemTypeId)
                            .to("item_types", "id")
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_image_id")
                            .from(ItemBase::Table, ItemBase::ImageId)
                            .to("images", "id")
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
    Name,
    DisplayEn,
    DisplayKor,
    ItemTypeId,
    ImageId,
}