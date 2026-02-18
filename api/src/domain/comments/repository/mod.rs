use entity::comments;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseTransaction, EntityTrait};

use crate::domain::comments::{CreateCommentInput, entity::Comment, error::CommentError};

pub async fn save_comment(txn: &DatabaseTransaction, new_comment: CreateCommentInput) -> Result<Comment, CommentError>{
    let model = comments::ActiveModel {
        post_id: Set(new_comment.post_id),
        author_id: Set(new_comment.author_id),
        author_name: Set(new_comment.author_name),
        password: Set(new_comment.password.get_value()),
        parent_id: Set(new_comment.parent_id),
        content: Set(new_comment.content),
        status: Set(entity::sea_orm_active_enums::Status::Active),
        ..Default::default()
    };

    let saved = comments::Entity::insert(model).exec_with_returning(txn).await?;

    Ok(Comment::from(saved))
}

pub async fn increase_reply_count(txn: &DatabaseTransaction, comment_id: i32) -> Result<Comment, CommentError>{
    let comment = comments::Entity::find_by_id(comment_id).one(txn).await?.ok_or(CommentError::ParentCommentNotFound(comment_id.to_string()))?;
    let current_reply_count = comment.reply_count;
    
    let mut comment: comments::ActiveModel = comment.into();
    comment.reply_count = Set(current_reply_count + 1);
    let comment = comment.update(txn).await?;

    Ok(Comment::from(comment))
}