use bcrypt::BcryptError;
use uuid::Uuid;

use crate::{domain::comments::presentation::CommentOrder, shared::{Pagination, PaginationQuery, SortDirection, hash_password}};

#[derive(Debug)]
pub struct CreateCommentInput {
    pub post_id: Uuid,
    pub author_id: Uuid,
    pub author_name: Option<String>,
    pub password: Password,
    pub parent_id: Option<i32>,
    pub content: String,
}

pub enum Password {
    Plain(String),
    Hashed(String),
}

impl std::fmt::Debug for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Password::Plain(_) => write!(f, "Password::Plain(**redacted**)"),
            Password::Hashed(_) => write!(f, "Password::Hashed(**redacted**)"),
        }
    }
}

impl CreateCommentInput {
    pub fn new( post_id: Uuid, author_id: Uuid, author_name: Option<String>, plain_password: String, parent_id: Option<i32>, content: String)  -> Self {
        Self { post_id, author_id, author_name, password: Password::Plain(plain_password), parent_id, content }
    }

    pub fn encode_password(self) -> Result<Self, BcryptError> {
        let encoded_pwd = self.password.encode()?;
        Ok( Self { post_id: self.post_id, author_id: self.author_id, author_name: self.author_name, password: encoded_pwd, parent_id: self.parent_id, content: self.content } )
    }
}


impl Password {
    pub fn new(raw_value: String) -> Self {
        Password::Plain(raw_value)
    }

    pub fn encode(self) -> Result<Self, BcryptError> {
        match self {
            Password::Plain(raw) => {
                let hashed = hash_password(&raw)?;
                Ok(Password::Hashed(hashed))
            },
            Password::Hashed(_) => Ok(self),
        }
    }

    pub fn get_value(self) -> String {
        match self {
            Password::Plain(s) => s,
            Password::Hashed(s) => s,
        }
    }
}

#[derive(Debug)]
pub struct SearchCommentInput{
    pub post_id: Uuid,
    pub pagination: Pagination,
    pub order_by: CommentOrder,
    pub direction: SortDirection,
}