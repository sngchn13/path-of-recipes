use std::fmt;

use entity::sea_orm_active_enums;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


pub struct Graph {
    pub id: i32,
    pub description: Option<String>,
    pub verification: Verification,
    pub post_id: Uuid,
}

impl Graph {
    pub fn new(id: i32, description: Option<String>, verification: Verification, post_id: Uuid) -> Self {
        Self { id, description, verification: verification, post_id }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub enum Verification {
    Pending,
    Valid,
    Invalid,
}

impl Verification {

    pub fn from(value: sea_orm_active_enums::Verification) -> Self{
        match value {
            sea_orm_active_enums::Verification::Pending => Verification::Pending,
            sea_orm_active_enums::Verification::Valid => Verification::Valid,
            sea_orm_active_enums::Verification::Invalid => Verification::Invalid,
        }
    }
}

impl fmt::Display for Verification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Verification::Pending => "pending",
            Verification::Valid => "valid",
            Verification::Invalid => "invalid"
        };

        write!(f, "{}", s)
    }
}