use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        
        manager
            .create_table(
                Table::create()
                    .table(ItemType::Table)
                    .if_not_exists()
                    .col(pk_auto(ItemType::Id))
                    .col(string(ItemType::Name).unique_key())
                    .to_owned()
            )
            .await?;
        manager.create_index(
            sea_query::Index::create()
            .name("idx_item_type_name")
            .table(ItemType::Table)
            .col(ItemType::Name)
            .to_owned())
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
       
        manager
            .drop_table(Table::drop().table(ItemType::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ItemType {
    #[sea_orm(iden = "item_types")]
    Table,
    Id,
    Name,
}
