-- Your SQL goes here
CREATE TYPE roleType AS ENUM ('Admin', 'User');
ALTER TABLE users
ADD role roleType DEFAULT 'User';
