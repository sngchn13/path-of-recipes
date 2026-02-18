use entity::comments;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, ExprTrait, Order, PaginatorTrait, QueryFilter, QueryOrder};

use crate::{domain::comments::{CreateCommentInput, SearchCommentInput, entity::Comment, error::CommentError}, shared::{Page, Pageable}};

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

pub async fn find_all_comments_by_condition(conn: &DatabaseConnection, condition: SearchCommentInput) -> Result<Page<Comment>, CommentError>{
    
    let order_by = match condition.order_by {
        super::presentation::CommentOrder::CreatedAt => comments::Column::CreatedAt,
        super::presentation::CommentOrder::LikeCount => comments::Column::LikeCount,
        super::presentation::CommentOrder::ReplyCount => comments::Column::ReplyCount,
    };

    let direction = match condition.direction {
        crate::shared::SortDirection::Desc => Order::Desc,
        crate::shared::SortDirection::Asc => Order::Asc,
    };

    let post_id = condition.post_id;

    let page_size = condition.pagination.size;
    let page = condition.pagination.page;
    
    let paginator = comments::Entity::find()
        .filter(comments::Column::PostId.eq(post_id).and(comments::Column::ParentId.is_null()))
        .order_by(order_by, direction)
        .paginate(conn, page_size);

    let items = paginator.fetch_page(page.0).await?;
    let items = items.into_iter().map(|m| Comment::from(m)).collect();

    let temp = paginator.num_items_and_pages().await?;
    let total_items = temp.number_of_items;
    let total_pages = temp.number_of_pages;
    let pageable = Pageable { page, size: page_size, total_items, total_pages };

    let paged_comments = Page::new(items, pageable);
    Ok(paged_comments)
}