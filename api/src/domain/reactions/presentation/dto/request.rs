use serde::Deserialize;

use crate::domain::reactions::ReactionType;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewReactionRequest {
    pub reaction_type: ReactionType
}
