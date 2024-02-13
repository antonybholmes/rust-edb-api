PRAGMA journal_mode=WAL;
CREATE TABLE users (id INTEGER PRIMARY KEY ASC, user_id TEXT NOT NULL UNIQUE, name TEXT, email TEXT NOT NULL UNIQUE, password TEXT NOT NULL);
CREATE INDEX users_user_id ON users (user_id);
CREATE INDEX users_name ON users (name);
CREATE INDEX users_email ON users (email);
