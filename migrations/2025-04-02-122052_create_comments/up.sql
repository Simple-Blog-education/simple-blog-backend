CREATE TABLE comments(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    post_id UUID NOT NULL references posts(id) ON DELETE CASCADE,
    user_id UUID NOT NULL references users(id) ON DELETE CASCADE,
    text VARCHAR NOT NULL
);
