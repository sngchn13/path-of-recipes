use uuid::Uuid;

use crate::domain::reactions::{ReactionType, Subject};

#[derive(Debug)]
pub struct NewReactionCommand{
    pub subject: Subject,
    pub anon_id: Uuid,
    pub reaction_type: ReactionType
}