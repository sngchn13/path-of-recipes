use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Image::Table)
                    .if_not_exists()
                    .col(pk_uuid(Image::Id))
                    .col(string(Image::RelativePath))
                    .col(string(Image::Name).unique_key())
                    .col(unsigned(Image::Width))
                    .col(unsigned(Image::Height))
                    .col(unsigned(Image::Size))
                    .col(string(Image::Extension))
                    .col(string(Image::Desc))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Image::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Image {
    #[sea_orm(iden = "images")]
    Table,
    Id,
    RelativePath,
    Name,
    Width,
    Height,
    Size,
    Extension,
    Desc,
}

