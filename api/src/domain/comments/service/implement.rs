use sea_orm::{DatabaseConnection, TransactionTrait};
use tracing::info;

use crate::{domain::comments::{PatchCommentCommand, ReplyCommentCondition, SearchCommentInput, entity::Comment, error::CommentError, repository::{find_all_comments_by_condition, find_all_comments_by_parent_id, find_by_id, increase_reply_count, save_comment, update_comment_by_id}, service::{CommentService, dto::CreateCommentInput}}, shared::{Cursor, Page, verify_password}};


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
    
    async fn patch_comment(&self, patch_comment: PatchCommentCommand) -> Result<Comment, CommentError>{
        let conn = self.database_connection.clone();
        let comment_id = patch_comment.comment_id;
        let content = patch_comment.content;
        let raw_password = patch_comment.password;

        let hashed = find_by_id(comment_id, &conn.clone()).await?.ok_or(CommentError::CommentNotFound(comment_id))?.password;

        if !verify_password(&raw_password, &hashed).map_err(|e| {
                info!("error = {}", e);
                CommentError::InvalidPassword
        })? {
            return Err(CommentError::InvalidPassword);
        }

        let res = update_comment_by_id(comment_id, content, &conn).await?;
        Ok(res)
    }
}