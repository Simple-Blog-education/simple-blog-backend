CREATE TABLE posts(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL references users(id),
    header VARCHAR NOT NULL,
    text VARCHAR NOT NULL,
    date TIMESTAMP WITH TIME ZONE NOT NULL default CURRENT_DATE
);
