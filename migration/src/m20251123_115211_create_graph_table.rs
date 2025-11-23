use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Graph::Table)
                    .if_not_exists()
                    .col(pk_auto(Graph::Id))
                    .col(text(Graph::Description).null())
                    .col(string(Graph::Lifecycle))
                    .col(integer(Graph::PostId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_id")
                            .from("graphs", "post_id")
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

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Graph {
    #[sea_orm(iden = "graphs")]
    Table,
    Id,
    Description,
    Lifecycle,
    PostId,
}
