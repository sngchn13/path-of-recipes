use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ModGroup::Table)
                    .if_not_exists()
                    .col(pk_auto(ModGroup::Id))
                    .col(string(ModGroup::Value))
                    .col(string(ModGroup::Affix))
                    .col(string(ModGroup::ModOrigin))
                    .col(integer(ModGroup::ItemTypeId))
                    .col(integer(ModGroup::FamilyId).null())
                    .col(integer(ModGroup::SectionId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_item_type_id")
                            .from("mod_groups", "item_type_id")
                            .to("item_types", "id")
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_family_id")
                            .from("mod_groups", "family_id")
                            .to("families", "id")
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_section_id")
                            .from("mod_groups", "section_id")
                            .to("sections", "id")
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
            .drop_table(Table::drop().table(ModGroup::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ModGroup {
    #[sea_orm(iden = "mod_groups")]
    Table,
    Id,
    Value,
    ModOrigin,
    Affix,
    ItemTypeId,
    FamilyId,
    SectionId,
}
