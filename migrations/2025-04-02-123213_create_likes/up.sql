CREATE TABLE post_likes(
    user_id UUID NOT NULL references users(id) ON DELETE CASCADE,
    post_id UUID NOT NULL references posts(id) ON DELETE CASCADE,
    CONSTRAINT pk_post_like PRIMARY KEY (user_id, post_id)
);

CREATE TABLE comment_likes(
    user_id UUID NOT NULL references users(id) ON DELETE CASCADE,
    comment_id UUID NOT NULL references comments(id) ON DELETE CASCADE,
    CONSTRAINT pk_comment_like PRIMARY KEY (user_id, comment_id)
);