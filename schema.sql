CREATE TABLE IF NOT EXISTS users(
    id UUID PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    balance UNSIGNED INT,
    password_hash TEXT NOT NULL,
);