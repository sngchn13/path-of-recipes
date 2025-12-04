use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ModGroupFamily::Table)
                    .if_not_exists()
                    .col(integer(ModGroupFamily::ModGroupId))
                    .col(integer(ModGroupFamily::FamilyId))
                    .primary_key(
                        Index::create()
                            .col(ModGroupFamily::ModGroupId)
                            .col(ModGroupFamily::FamilyId)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_mod_group_id")
                            .from("mod_group_families", "mod_group_id")
                            .to("mod_groups", "id")
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_family_id")
                            .from("mod_group_families", "family_id")
                            .to("families", "id")
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
            .drop_table(Table::drop().table(ModGroupFamily::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ModGroupFamily {
    #[sea_orm(iden = "mod_group_families")]
    Table,
    ModGroupId,
    FamilyId,
}
