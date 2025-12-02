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
                    .col(string(Mod::NameEn))
                    .col(string(Mod::NameKor))
                    .col(string(Mod::DisplayEn))
                    .col(string(Mod::DisplayKor))
                    .col(unsigned(Mod::ReqItemLevel))
                    .col(small_unsigned(Mod::Weight).null())
                    .col(tiny_unsigned(Mod::Tier).null())
                    .col(integer(Mod::ModGroupId))
                    .col(string(Mod::DeprecatedVersion).null())
                    .col(timestamp_with_time_zone(Mod::CreatedAt))
                    .col(timestamp_with_time_zone(Mod::UpdatedAt).null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_mof_group_id")
                            .from("mods", "mod_group_id")
                            .to("mod_groups", "id")
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .index(
                        Index::create()
                            .name("idx_tier_mod_group_id")
                            .col(Mod::Tier)
                            .col(Mod::ModGroupId)
                            .unique()
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
    NameEn,
    NameKor,
    DisplayEn,
    DisplayKor,
    ReqItemLevel,
    Weight,
    Tier,
    ModGroupId,
    DeprecatedVersion,
    CreatedAt,
    UpdatedAt,
}
