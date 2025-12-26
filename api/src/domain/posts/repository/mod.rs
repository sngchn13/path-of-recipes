use entity::posts;
use sea_orm::{ActiveValue::Set, DatabaseTransaction, EntityTrait};
use uuid::Uuid;

use crate::{domain::posts::{Post, error::PostRepositoryError, presentation::NewPost, repository::orm_to_domain::to_post}, shared::hash_password};


pub async fn save_post(txn: &DatabaseTransaction , new_post: &NewPost) -> Result<Post, PostRepositoryError>{
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

mod orm_to_domain {

    use entity::{ posts::Model as PostOrm };

    use crate::{domain::posts::Post, shared::{fixed_offset_opt_to_offset_opt, fixed_offset_to_offset_datetime}};

    pub fn to_post(orm: PostOrm)
        -> Post {
            Post::new(
                orm.id,
                orm.title,
                orm.content,
                orm.password,
                orm.view,
                orm.like,
                orm.dislike,
                fixed_offset_to_offset_datetime(orm.created_at),
                fixed_offset_opt_to_offset_opt(orm.updated_at)
            )
        }

}