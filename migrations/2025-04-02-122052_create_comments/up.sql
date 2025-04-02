CREATE TABLE comments(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    post_id UUID NOT NULL references posts(id),
    user_id UUID NOT NULL references users(id),
    text VARCHAR NOT NULL
);
