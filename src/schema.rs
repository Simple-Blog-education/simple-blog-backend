// @generated automatically by Diesel CLI.

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
        create_date -> Timestamptz,
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
    users (id) {
        id -> Uuid,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        reg_date -> Timestamptz,
        role -> Text,
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
