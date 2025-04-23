DROP DATABASE IF EXISTS words;
CREATE DATABASE words;

\c words


CREATE TABLE nouns (
                       id SERIAL PRIMARY KEY,
                       noun TEXT NOT NULL
);

INSERT INTO nouns (noun) VALUES ('cat'), ('dog'), ('bird'), ('fish');

CREATE TABLE verbs (
                       id SERIAL PRIMARY KEY,
                       verb TEXT NOT NULL
);

INSERT INTO verbs (verb) VALUES ('eat'), ('f**k'), ('marry'), ('kill');