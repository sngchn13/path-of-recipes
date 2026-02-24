use sea_orm::{DatabaseConnection};
use tracing::warn;

use crate::domain::{comments::increase_comment_reaction_count, posts::increase_post_reaction_count, reactions::{ReactionError, repository::save_reaction, service::{NewReactionCommand, ReactionService}}};

pub struct ReactionServiceImpl{
    database_connection: DatabaseConnection,
}

impl ReactionServiceImpl {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self { database_connection }
    }
}

#[async_trait::async_trait]
impl ReactionService for ReactionServiceImpl {
    async fn save_reaction(&self, new_reaction: NewReactionCommand) -> Result<(), ReactionError> {

        let conn = self.database_connection.clone();

        let subject = new_reaction.subject.clone();
        let reaction_type = new_reaction.reaction_type.clone();
        save_reaction(new_reaction, &conn).await?;

        tokio::spawn(async move {
            match subject {
                crate::domain::reactions::Subject::Post(post_id) => {
                    if let Err(e) = increase_post_reaction_count(post_id, reaction_type, &conn).await {
                        warn!(error = %e, post_id = %post_id, "increase reaction_count failed");
                    } 
                },
                crate::domain::reactions::Subject::Comment(comment_id) => {
                    if let Err(e) = increase_comment_reaction_count(comment_id, reaction_type, &conn).await {
                        warn!(error = %e, comment_id = %comment_id, "increase reaction_count failed");
                    }
                },
            };
        });

        Ok(())
    }
}