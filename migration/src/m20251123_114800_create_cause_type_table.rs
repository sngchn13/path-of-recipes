use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CauseType::Table)
                    .if_not_exists()
                    .col(pk_auto(CauseType::Id))
                    .col(string(CauseType::Name))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CauseType::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum CauseType {
    #[sea_orm(iden = "cause_types")]
    Table,
    Id,
    Name,
}
