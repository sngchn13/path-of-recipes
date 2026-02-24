use sea_orm::{DatabaseConnection, TransactionTrait};
use tracing::info;

use crate::{domain::comments::{ReplyCommentCondition, SearchCommentInput, entity::Comment, error::CommentError, repository::{find_all_comments_by_condition, find_all_comments_by_parent_id, increase_reply_count, save_comment}, service::{CommentService, dto::CreateCommentInput}}, shared::{Cursor, Page}};


pub struct CommentServiceImpl{
    database_connection: DatabaseConnection,
}

impl CommentServiceImpl {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self { database_connection }
    }
}

#[async_trait::async_trait]
impl CommentService for CommentServiceImpl {
    async fn save_comment(&self, new_comment: CreateCommentInput) -> Result<i32, CommentError>{
        info!("new_comment={:#?}", new_comment);
        let new_comment = new_comment.encode_password()?;

        let txn = self.database_connection.begin().await?;
        let comment = save_comment(&txn, new_comment).await?;
        
        if let Some(parent_id) = comment.parent_id {
            increase_reply_count(&txn, parent_id).await?;
        }

        txn.commit().await?;

        Ok(comment.id)
    }

    async fn search_comments(&self, condition: SearchCommentInput) -> Result<Page<Comment>, CommentError>{
        find_all_comments_by_condition(&self.database_connection, condition).await
    }

    async fn search_reply_comments(&self, condition: ReplyCommentCondition) -> Result<Cursor<Comment>, CommentError>{
        let conn =  self.database_connection.clone();
        find_all_comments_by_parent_id(condition.parent_id, condition.size, condition.after_id, &conn).await
    }
}