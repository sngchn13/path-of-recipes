use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub enum Rarity {
    #[serde(alias = "normal")]
    NORMAL,
    #[serde(alias = "magic")]
    MAGIC,
    #[serde(alias = "rare")]
    RARE,
}