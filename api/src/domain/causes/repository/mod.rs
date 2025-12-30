use std::collections::HashMap;
use entity::causes;
use sea_orm::{DatabaseTransaction, EntityTrait};

use crate::{domain::causes::{Cause, CauseRepositoryError}, shared::to_cause};

mod dto;
pub use dto::*;


pub async fn find_all_returning_map(txn: &DatabaseTransaction) -> Result<HashMap<String, Cause>, CauseRepositoryError>{
    let models = causes::Entity::find().all(txn).await?;
    let map: HashMap<String, Cause> = models.into_iter().map(|c| (c.name.clone(), to_cause(c))).collect();

    Ok(map)
}