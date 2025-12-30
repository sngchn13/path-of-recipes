use entity::posts;
use sea_orm::{ActiveValue::Set, DatabaseConnection, DatabaseTransaction, EntityTrait};
use uuid::Uuid;

use crate::{domain::{graphs::Graph, posts::{Post, error::PostRepositoryError, presentation::NewPost}}, shared::{hash_password, to_post}};


pub async fn save_post(txn: &DatabaseTransaction, new_post: &NewPost) -> Result<Post, PostRepositoryError>{
    let id = Uuid::new_v4();
    let hashed_password = hash_password(&new_post.password)?;

    let post = posts::ActiveModel {
        id: Set(id),
        title: Set(new_post.title.clone()),
        content: Set("".to_string()),
        password: Set(hashed_password),
        ..Default::default()
    };
    
    let saved = posts::Entity::insert(post).exec_with_returning(txn).await?;

    Ok(to_post(saved))
}

pub async fn find_post_by_id(post_id: Uuid, conn: &DatabaseConnection) -> Result<Option<Post>, PostRepositoryError>{
    Ok(posts::Entity::find_by_id(post_id).one(conn).await?.and_then(|v| Some(to_post(v))))
}