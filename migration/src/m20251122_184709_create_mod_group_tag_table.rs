use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ModTag::Table)
                    .if_not_exists()
                    .col(integer(ModTag::ModGroupId))
                    .col(integer(ModTag::TagId))
                    .primary_key(
                        Index::create()
                            .col(ModTag::ModGroupId)
                            .col(ModTag::TagId)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_mod_group_id")
                            .from("mod_tags", "mod_group_id")
                            .to("mod_groups", "id")
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tag_id")
                            .from("mod_tags", "tag_id")
                            .to("tags", "id")
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
            .drop_table(Table::drop().table(ModTag::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ModTag {
    #[sea_orm(iden = "mod_group_tags")]
    Table,
    ModGroupId,
    TagId,
}
