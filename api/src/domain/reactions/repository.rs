use entity::{reactions, sea_orm_active_enums::ReactionType};
use sea_orm::{ActiveValue::Set, DatabaseConnection, DbErr, EntityTrait, sea_query};

use crate::domain::reactions::{ReactionError, service::NewReactionCommand};


pub async fn save_reaction(new_reaction: NewReactionCommand, conn: &DatabaseConnection) -> Result<(), ReactionError> {
    
    let rtype = match new_reaction.reaction_type {
        crate::domain::reactions::ReactionType::Like => ReactionType::Like,
        crate::domain::reactions::ReactionType::Dislike => ReactionType::Dislike,
    };
    
    let reaction = reactions::ActiveModel {
        subject: Set(new_reaction.subject.to_string()),
        anon_id: Set(new_reaction.anon_id),
        reaction_type: Set(rtype),
        ..Default::default()
    };

    let res = reactions::Entity::insert(reaction)
        .on_conflict(
            sea_query::OnConflict::columns([reactions::Column::Subject, reactions::Column::AnonId])
                .do_nothing()
                .to_owned()
        )
        .exec(conn)
        .await;

    match res {
        Ok(_) => Ok(()),
        Err(DbErr::RecordNotInserted) => Err(ReactionError::Conflict(new_reaction.anon_id, new_reaction.subject.to_string())),
        Err(e) => Err(ReactionError::from(e)),
    }
}