use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Mod::Table)
                    .if_not_exists()
                    .col(pk_auto(Mod::Id))
                    .col(string(Mod::Name))
                    .col(string(Mod::ModOrigin))
                    .col(string(Mod::Affix))
                    .col(small_unsigned(Mod::Weight).null())
                    .col(unsigned(Mod::ReqItemLv))
                    .col(string(Mod::DisplayEn))
                    .col(string(Mod::DisplayKor))
                    .col(tiny_unsigned(Mod::Tier).null())
                    .col(integer(Mod::TierGroupId).null())
                    .col(integer(Mod::ItemTypeId))
                    .col(integer(Mod::FamilyId).null())
                    .col(integer(Mod::SectionId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tier_group_id")
                            .from("mods", "tier_group_id")
                            .to("tier_groups", "id")
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_item_type_id")
                            .from("mods", "item_type_id")
                            .to("item_types", "id")
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_family_id")
                            .from("mods", "family_id")
                            .to("families", "id")
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_section_id")
                            .from("mods", "section_id")
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
            .drop_table(Table::drop().table(Mod::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Mod {
    #[sea_orm(iden = "mods")]
    Table,
    Id,
    Name,
    ModOrigin,
    Affix,
    Weight,
    ReqItemLv,
    DisplayEn,
    DisplayKor,
    Tier,
    TierGroupId,
    ItemTypeId,
    FamilyId,
    SectionId,
}
