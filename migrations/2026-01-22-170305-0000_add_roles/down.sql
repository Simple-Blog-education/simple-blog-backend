-- This file should undo anything in `up.sql`
DROP TYPE roleType;
ALTER TABLE users
DROP COLUMN role;
