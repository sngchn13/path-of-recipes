use serde::{Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Cursor<T> {
    pub content: Vec<T>,
    pub next_cursor: Option<i32>,
}

impl<T> Cursor<T> {
    pub fn new(content: Vec<T>, next_cursor: Option<i32> ) -> Self{
        Self { content, next_cursor}
    }
}