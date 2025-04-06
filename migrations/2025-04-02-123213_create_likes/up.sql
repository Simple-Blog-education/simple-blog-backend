CREATE TABLE post_likes(
    user_id UUID NOT NULL references users(id),
    post_id UUID NOT NULL references posts(id),
    CONSTRAINT pk_post_like PRIMARY KEY (user_id, post_id)
);

CREATE TABLE comment_likes(
    user_id UUID NOT NULL references users(id),
    comment_id UUID NOT NULL references comments(id),
    CONSTRAINT pk_comment_like PRIMARY KEY (user_id, comment_id)
);