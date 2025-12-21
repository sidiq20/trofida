CREATE DATABASE streaky;

CREATE TABLE users (
    id UUID PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT now()
)

CREATE TABLE todos (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    streak INT NOT NULL DEFAULT 0,
    last_completed DATE,
    created_at TIMESTAMP DEFAULT now()
)

CREATE TABLE todo_completions (
    id UUID PRIMARY KEY,
    todo_id UUID REFERENCES todos(id) ON DELETE CASCADE,
    completed_at TIMESTAMP DEFAULT now()
)

