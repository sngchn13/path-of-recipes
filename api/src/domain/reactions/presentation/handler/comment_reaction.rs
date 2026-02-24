use std::sync::Arc;

use axum::{Json, extract::{Path, State}, http::StatusCode};

use crate::{app::AnonId, domain::reactions::{ReactionError, ReactionService, Subject, presentation::dto::NewReactionRequest, service::NewReactionCommand}};

// post /comments/{comment_id}/reactions
#[axum::debug_handler]
pub async fn post_new_comment_reaction(
    State(reaction_service): State<Arc<dyn ReactionService>>,
    anon_id: AnonId,
    Path(comment_id): Path<i32>,
    Json(reaction_request): Json<NewReactionRequest>
) -> Result<StatusCode, ReactionError>{
    let anon_id = anon_id.0;
    let reaction_command = NewReactionCommand{ subject: Subject::Comment(comment_id), anon_id, reaction_type: reaction_request.reaction_type };

    reaction_service.save_reaction(reaction_command).await?;

    Ok(StatusCode::CREATED)
}