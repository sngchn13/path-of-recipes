use std::collections::HashSet;

use entity::{graphs, posts};
use migration::Expr;
use sea_orm::{ActiveValue::Set, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, ExprTrait, FromQueryResult, JoinType, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait, prelude::DateTimeWithTimeZone, raw_sql};
use tracing::warn;
use uuid::Uuid;

use crate::{domain::{posts::{Post, PostError, PostOrder, presentation::NewPost}, reactions::ReactionType}, shared::{Page, Pageable, Pagination, SortDirection, fixed_offset_opt_to_offset_opt, fixed_offset_to_offset_datetime, hash_password, to_post}};


pub async fn save_post(txn: &DatabaseTransaction, new_post: &NewPost) -> Result<Post, PostError>{
    let id = Uuid::new_v4();
    let hashed_password = hash_password(&new_post.password)?;

    let post = posts::ActiveModel {
        id: Set(id),
        title: Set(new_post.title.clone()),
        content: Set(new_post.content.clone()),
        password: Set(hashed_password),
        ..Default::default()
    };
    
    let saved = posts::Entity::insert(post).exec_with_returning(txn).await?;

    Ok(to_post(saved))
}

pub async fn find_post_by_id(post_id: Uuid, conn: &DatabaseConnection) -> Result<Option<Post>, PostError>{
    Ok(posts::Entity::find_by_id(post_id).one(conn).await?.and_then(|v| Some(to_post(v))))
}

pub async fn find_posts_by_node_id(node_ids: HashSet<i32>, pagination: Pagination, order_by: PostOrder, direction: SortDirection, conn: &DatabaseConnection) 
    -> Result<Page<(Post, i32)>, PostError>
{
    if node_ids.is_empty() { return Ok(Page::new(Vec::new(), Pageable { page: pagination.page, size: pagination.size, total_items: 0, total_pages: 0 })); }

    let order_by = match order_by {
        PostOrder::ViewCount => "p.view_count",
        PostOrder::LikeCount => "p.like_count",
        PostOrder::CreatedAt => "p.created_at",
    };
    let page = pagination.page;
    let size = pagination.size;

    let post_with_node_ids = match direction {
        SortDirection::Desc => PostWithNodeId::find_by_statement(raw_sql!(
            Postgres,
            r#"SELECT p.id, p.title, p.content, p.password, p.view_count, p.like_count, p.dislike_count, p.created_at, p.updated_at, n.id as node_id
            FROM nodes n 
                JOIN graphs g ON n.graph_id = g.id
                JOIN posts p ON g.post_id = p.id
            WHERE n.id IN ({..node_ids})
            ORDER BY {order_by} DESC, p.id DESC
            "#
        )).paginate(conn, size),
        SortDirection::Asc => PostWithNodeId::find_by_statement(raw_sql!(
            Postgres,
            r#"SELECT p.id, p.title, p.content, p.password, p.view_count, p.like_count, p.dislike_count, p.created_at, p.updated_at, n.id as node_id
            FROM nodes n 
                JOIN graphs g ON n.graph_id = g.id
                JOIN posts p ON g.post_id = p.id
            WHERE n.id IN ({..node_ids})
            ORDER BY {order_by} ASC, p.id ASC
            "#
            )).paginate(conn, size),
    };

    let posts: Vec<(Post, i32)> = post_with_node_ids.fetch_page(page.0).await?.into_iter().map(|pn| pn.divide()).collect();
    let total_items = post_with_node_ids.num_items().await?;
    let total_pages = post_with_node_ids.num_pages().await?;
    let pageable = Pageable { page, size, total_items, total_pages};

    let paged_posts = Page::new(posts, pageable);

    Ok(paged_posts)
}

pub async fn find_all_posts(pagination: Pagination, order_by: PostOrder, direction: SortDirection, conn: &DatabaseConnection) -> Result<Page<Post>, PostError> {
    let order_by = match order_by {
        PostOrder::ViewCount => posts::Column::ViewCount,
        PostOrder::LikeCount => posts::Column::LikeCount,
        PostOrder::CreatedAt => posts::Column::CreatedAt,
    };

    let direction = match direction {
        SortDirection::Desc => Order::Desc,
        SortDirection::Asc => Order::Asc,
    };

    let page = pagination.page.0;
    let size = pagination.size;

    let paginated = posts::Entity::find()
        .join(JoinType::InnerJoin, posts::Relation::Graphs.def())
        .join(JoinType::InnerJoin, graphs::Relation::Nodes.def())
        .distinct()
        .order_by(order_by, direction).paginate(conn, size);

    let posts: Vec<Post> = paginated.fetch_page(page).await?.into_iter().map(|p| to_post(p)).collect();
    let pageable = Pageable{ page: pagination.page, size, total_items: paginated.num_items().await?, total_pages: paginated.num_pages().await? };

    Ok(Page::new(posts, pageable))
}

pub async fn increase_post_reaction_count(post_id: Uuid, reaction_type: ReactionType, conn: &DatabaseConnection) -> Result<(), PostError>{
    match reaction_type {
        ReactionType::Like => {
            posts::Entity::update_many()
                .col_expr(posts::Column::LikeCount, Expr::col(posts::Column::LikeCount).add(1))
                .filter(posts::Column::Id.eq(post_id))
                .exec(conn)
                .await?;
        },
        ReactionType::Dislike => {
            posts::Entity::update_many()
                .col_expr(posts::Column::DislikeCount, Expr::col(posts::Column::DislikeCount).add(1))
                .filter(posts::Column::Id.eq(post_id))
                .exec(conn)
                .await?;
        },
    };

    Ok(())
}

pub async fn increase_post_view_count(post_id: Uuid, conn: &DatabaseConnection) {
    let res = posts::Entity::update_many()
        .col_expr(posts::Column::ViewCount, Expr::col(posts::Column::ViewCount).add(1))
        .filter(posts::Column::Id.eq(post_id))
        .exec(conn)
        .await;

    if let Err(e) = res {
        warn!("Failed to increment view count. post_id={} error={}", post_id, e);
    }
}

#[derive(Debug, FromQueryResult)]
struct PostWithNodeId {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub password: String,
    pub view_count: i32,
    pub like_count: i32,
    pub dislike_count: i32,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub node_id: i32,
}

impl PostWithNodeId {
    pub fn divide(self) -> (Post, i32) {
        (
            Post{
                id: self.id,
                title: self.title,
                content: self.content,
                password: self.password,
                view_count: self.view_count,
                like_count: self.like_count,
                dislike_count: self.dislike_count,
                created_at: fixed_offset_to_offset_datetime(self.created_at),
                updated_at: fixed_offset_opt_to_offset_opt( self.updated_at),
            },
            self.node_id
        )
    }
}