use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, serde::rfc3339};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Mod{
    pub id: i32,
    pub name_en: String,
    pub name_kor: String,
    pub display_en: String,
    pub display_kor: String,
    pub req_item_level: i32,
    pub weight: Option<i16>,
    pub tier: Option<i16>,
    pub deprecated_version: Option<String>,
    #[serde(with = "rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "rfc3339::option")]
    pub updated_at: Option<OffsetDateTime>,
}

impl Mod {
    pub fn new(id: i32, name_en: String, name_kor: String, display_en: String, display_kor: String, req_item_level: i32, 
        weight: Option<i16>, tier: Option<i16>, deprecated_version: Option<String>, created_at: OffsetDateTime, updated_at: Option<OffsetDateTime>) -> Self 
    {
        Self { id, name_en, name_kor, display_en, display_kor, req_item_level, weight, tier, deprecated_version, created_at, updated_at }
    }
}