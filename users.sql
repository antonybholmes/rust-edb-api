PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;

CREATE TABLE users (
    id INTEGER PRIMARY KEY ASC, 
    user_id TEXT NOT NULL UNIQUE, 
    name TEXT NOT NULL, 
    email TEXT NOT NULL UNIQUE, 
    password TEXT NOT NULL,
    otp TEXT,
    is_verified BOOLEAN NOT NULL DEFAULT 0,
    created_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL);
CREATE INDEX users_user_id ON users (user_id);
CREATE INDEX users_name ON users (name);
CREATE INDEX users_email ON users (email);
CREATE INDEX users_otp ON users (otp);

CREATE TRIGGER users_updated_trigger AFTER UPDATE ON users
BEGIN
      update users SET updated_on = CURRENT_TIMESTAMP WHERE id=NEW.id;
END;
