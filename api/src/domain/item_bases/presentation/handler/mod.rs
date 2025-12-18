use std::sync::Arc;

use axum::{Json, extract::{Query, State}, http::StatusCode};
use serde::Deserialize;

use crate::domain::item_bases::{ItemBaseHandlerError, ItemBaseService, presentation::dto::{ItemBaseResponse, item_base_to_item_base_response}};


#[axum::debug_handler]
pub async fn get_item_bases_by_item_name(
    State(item_base_service): State<Arc<dyn ItemBaseService>>,
    Query(query): Query<GetItemBaseQuery>
) -> Result<(StatusCode, Json<Vec<ItemBaseResponse>>), ItemBaseHandlerError>
{
    let item_type_name = match query.item_type_name {
        None => Err(ItemBaseHandlerError::MissingRequiredParameter("item_type_name".to_string()))?,
        Some(v) => v,
    };

    let (item_type, item_bases) = item_base_service.get_item_type_with_bases(&item_type_name).await?;
    
    let result: Vec<ItemBaseResponse> = item_bases.into_iter()
        .map(|ib| item_base_to_item_base_response(ib, &item_type))
        .collect();
    
    Ok((StatusCode::OK, Json(result)))
}


#[derive(Deserialize)]
pub struct GetItemBaseQuery{
    pub item_type_name: Option<String>,
}