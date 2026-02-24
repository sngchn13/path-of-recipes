mod dto;
mod implement;

pub use dto::*;
pub use implement::ReactionServiceImpl;

use crate::domain::reactions::{ReactionError};

#[async_trait::async_trait]
pub trait ReactionService: Send + Sync {
    async fn save_reaction(&self, new_reaction: NewReactionCommand) -> Result<(), ReactionError>;
}