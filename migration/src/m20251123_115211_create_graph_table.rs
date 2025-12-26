use sea_orm_migration::{prelude::{extension::postgres::Type, *}, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        
        manager.create_type(
            Type::create()
            .as_enum(Verification::Table)
            .values([
                Verification::Pending,
                Verification::Valid,
                Verification::Invalid,
            ])
            .to_owned(),
        ).await?;
        
        manager
            .create_table(
                Table::create()
                    .table(Graph::Table)
                    .if_not_exists()
                    .col(pk_auto(Graph::Id))
                    .col(text(Graph::Description).null())
                    .col(
                        ColumnDef::new(Graph::Verification)
                            .custom(Verification::Table)
                            .not_null(),
                    )
                    .col(uuid(Graph::PostId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_id")
                            .from(Graph::Table, Graph::PostId)
                            .to("posts", "id")
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
            .drop_table(Table::drop().table(Graph::Table).to_owned())
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .name(Verification::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Graph {
    #[sea_orm(iden = "graphs")]
    Table,
    Id,
    Description,
    Verification,
    PostId,
}

#[derive(DeriveIden)]
enum Verification {
    Table,
    Pending,
    Valid,
    Invalid,
}
