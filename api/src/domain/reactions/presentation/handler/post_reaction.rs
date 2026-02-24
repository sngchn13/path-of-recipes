use std::sync::Arc;
use axum::{extract::{Json, Path, State}, http::StatusCode};
use tracing::debug;
use uuid::Uuid;

use crate::{app::AnonId, domain::reactions::{ReactionError, ReactionService, Subject, presentation::dto::NewReactionRequest, service::NewReactionCommand}};

// post /posts/{post_id}/reactions
#[axum::debug_handler]
pub async fn post_new_post_reaction(
    State(reaction_service): State<Arc<dyn ReactionService>>,
    anon_id: AnonId,
    Path(post_id): Path<Uuid>,
    Json(reaction_request): Json<NewReactionRequest>
) -> Result<StatusCode, ReactionError>{
    let anon_id = anon_id.0;
    let reaction_command = NewReactionCommand{ subject: Subject::Post(post_id), anon_id, reaction_type: reaction_request.reaction_type };

    debug!("input = {:#?}", reaction_command);
    reaction_service.save_reaction(reaction_command).await?;

    Ok(StatusCode::CREATED)
}