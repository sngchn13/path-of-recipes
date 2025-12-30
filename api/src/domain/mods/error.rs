use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModRepositoryError {
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),

    #[error("Failed to find a mod_group for mod id '{0}'.")]
    ModGroupNotFound(String),

    #[error("Failed to find a mod_generation_type for mod_group_id '{0}'.")]
    ModGenerationTypeNotFound(String),

    #[error("Failed to find a section for mod_group_id '{0}'.")]
    SectionNotFound(String),

    #[error("Failed to find a section for mod_group_id '{0}'.")]
    ItemTypeNotFound(String),

    #[error("Failed to find families for mod_group_id '{0}'.")]
    FamiliesNotFound(String),

    #[error("Failed to find tags for mod_group_id '{0}'.")]
    TagsNotFound(String),
}