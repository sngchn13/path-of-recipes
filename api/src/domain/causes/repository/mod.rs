use std::collections::HashMap;
use entity::causes;
use sea_orm::{DatabaseTransaction, EntityTrait};

use crate::domain::causes::{Cause, CauseRepositoryError, repository::orm_to_domain::to_cause};


pub async fn find_all_returning_map(txn: &DatabaseTransaction) -> Result<HashMap<String, Cause>, CauseRepositoryError>{
    let models = causes::Entity::find().all(txn).await?;
    let map: HashMap<String, Cause> = models.into_iter().map(|c| (c.name.clone(), to_cause(c))).collect();

    Ok(map)
}

mod orm_to_domain {

    use entity::{ causes::Model as CauseOrm };

    use crate::domain::causes::Cause;

    pub fn to_cause(orm: CauseOrm)-> Cause {
        Cause::new(orm.id, orm.name, orm.cause_type_id)
    }

}