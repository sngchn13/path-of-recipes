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
                    .col(integer(ModGroup::ModGenerationTypeId))
                    .col(integer(ModGroup::ItemTypeId))
                    .col(integer(ModGroup::SectionId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_item_type_id")
                            .from(ModGroup::Table, ModGroup::ItemTypeId)
                            .to("item_types", "id")
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_mod_generation_type_id")
                            .from(ModGroup::Table, ModGroup::ModGenerationTypeId)
                            .to("mod_generation_types", "id")
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_section_id")
                            .from(ModGroup::Table, ModGroup::SectionId)
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
    ModGenerationTypeId,
    ItemTypeId,
    SectionId,
}
