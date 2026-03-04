// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "roletype"))]
    pub struct Roletype;
}

diesel::table! {
    comment_likes (user_id, comment_id) {
        user_id -> Uuid,
        comment_id -> Uuid,
    }
}

diesel::table! {
    comments (id) {
        id -> Uuid,
        post_id -> Uuid,
        user_id -> Uuid,
        text -> Varchar,
    }
}

diesel::table! {
    post_likes (user_id, post_id) {
        user_id -> Uuid,
        post_id -> Uuid,
    }
}

diesel::table! {
    posts (id) {
        id -> Uuid,
        user_id -> Uuid,
        header -> Varchar,
        text -> Varchar,
        create_date -> Timestamptz,
        edit_date -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Roletype;

    users (id) {
        id -> Uuid,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        reg_date -> Timestamptz,
        role -> Roletype,
    }
}

diesel::joinable!(comment_likes -> comments (comment_id));
diesel::joinable!(comment_likes -> users (user_id));
diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(post_likes -> posts (post_id));
diesel::joinable!(post_likes -> users (user_id));
diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(comment_likes, comments, post_likes, posts, users,);
