mod post_comment;
mod get_comments;

pub use post_comment::post_new_comment;
pub use get_comments::{get_comments_by_conditon, CommentOrder};