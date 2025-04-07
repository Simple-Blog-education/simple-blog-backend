CREATE TABLE posts(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL references users(id),
    header VARCHAR NOT NULL,
    text VARCHAR NOT NULL,
    create_date TIMESTAMP WITH TIME ZONE NOT NULL default CURRENT_TIMESTAMP,
    edit_date TIMESTAMP WITH TIME ZONE NOT NULL default CURRENT_TIMESTAMP
);
