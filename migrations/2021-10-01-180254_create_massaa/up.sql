-- Your SQL goes here
CREATE TABLE massaa (
    id integer primary key,
    ts timestamp default current_timestamp,
    kg float CHECK(kg > 0)
);
