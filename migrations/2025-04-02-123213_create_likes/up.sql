CREATE TABLE post_likes(
    user_id UUID,
    post_id UUID,
    CONSTRAINT pk_post_like PRIMARY KEY (user_id, post_id)
);

CREATE TABLE comment_likes(
    user_id UUID,
    comment_id UUID,
    CONSTRAINT pk_comment_like PRIMARY KEY (user_id, comment_id)
);