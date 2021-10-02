-- This file should undo anything in `up.sql`

-- Sigh. All this because sqlite does not support DROP COLUMN, yet.
-- Support for that is in v3.35.0 onwards, we have v3.31.0..
-- Although we could just do nothing? Let the column be but remove it from src/models.rs
CREATE TEMPORARY TABLE m_bak(id,ts,kg);
INSERT INTO m_bak SELECT id,ts,kg from massaa;
DROP TABLE massaa;
CREATE TABLE massaa (
    id integer primary key,
    ts timestamp default current_timestamp,
    kg float CHECK(kg > 0)
);
INSERT INTO massaa SELECT id,ts,kg from m_bak;
DROP TABLE m_bak;
