use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Cause::Table)
                    .if_not_exists()
                    .col(pk_auto(Cause::Id))
                    .col(string(Cause::Name))
                    .col(integer(Cause::CauseTypeId).null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_cause_type_id")
                            .from(Cause::Table, Cause::CauseTypeId)
                            .to("cause_types", "id")
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
            .drop_table(Table::drop().table(Cause::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Cause {
    #[sea_orm(iden = "causes")]
    Table,
    Id,
    Name,
    CauseTypeId,
}
