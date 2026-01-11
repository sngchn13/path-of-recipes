use std::collections::HashMap;
use entity::causes;
use sea_orm::{DatabaseTransaction, EntityTrait};

use crate::{domain::causes::{Cause, CauseError}, shared::to_cause};

mod dto;
pub use dto::*;


pub async fn find_all_returning_map(txn: &DatabaseTransaction) -> Result<HashMap<String, Cause>, CauseError>{
    let models = causes::Entity::find().all(txn).await?;
    let map: HashMap<String, Cause> = models.into_iter().map(|c| (c.name.clone(), to_cause(c))).collect();

    Ok(map)
}