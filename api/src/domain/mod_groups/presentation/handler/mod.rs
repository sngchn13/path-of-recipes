use std::sync::Arc;

use axum::{Json, extract::{Query, State}, http::StatusCode};
use serde::Deserialize;

use crate::domain::mod_groups::{ModGroupService, error::ModGroupHandlerError, presentation::dto::ModGroupResponse};

#[axum::debug_handler]
pub async fn get_mod_groups_by_item_type(
    State(mod_group_service): State<Arc<dyn ModGroupService>>, Query(query): Query<ModGroupQuery>) 
    -> Result<(StatusCode, Json<Vec<ModGroupResponse>>), ModGroupHandlerError >
{
    match  query.item_type_name{
        None => Err(ModGroupHandlerError::MissingRequiredParameter("item_type".to_string())),
        Some(item_type_name) => {
            let result = mod_group_service.calculate_mod_groups_for_item_type(&item_type_name).await?;
            let result = result.into_iter().map(|mg| ModGroupResponse::from(mg)).collect();

            Ok((StatusCode::OK, Json(result)))
        },
    }
}

#[derive(Deserialize)]
pub struct ModGroupQuery {
    pub item_type_name: Option<String>,
}