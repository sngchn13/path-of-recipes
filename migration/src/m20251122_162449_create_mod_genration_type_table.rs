use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ModGenerationType::Table)
                    .if_not_exists()
                    .col(integer(ModGenerationType::Id).primary_key())
                    .col(string(ModGenerationType::Label).null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ModGenerationType::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ModGenerationType {
    #[sea_orm(iden = "mod_generation_types")]
    Table,
    Id,
    Label,
}
