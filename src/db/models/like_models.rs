use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::post_likes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostLike {
    pub user_id: Uuid,
    pub post_id: Uuid,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::comment_likes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CommentLike {
    pub user_id: Uuid,
    pub comment_id: Uuid,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::comment_likes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewCommentLike {
    pub comment_id: Uuid,
    pub user_id: Uuid,
}