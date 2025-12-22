CREATE DATABASE streaky;

CREATE TABLE users (
    id UUID PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT now()
);

CREATE TABLE todos (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    streak INT NOT NULL DEFAULT 0,
    last_completed DATE,
    streak_required INT NOT NULL DEFAULT 0,
    created_at TIMESTAMP DEFAULT now()
);

CREATE TYPE todo_status AS ENUM ('active', 'paused', 'completed');

ALTER TABLE todos 
ADD COLUMN status todo_status NOT NULL DEFAULT 'active';

CREATE TABLE todo_completions (
    id UUID PRIMARY KEY,
    todo_id UUID REFERENCES todos(id) ON DELETE CASCADE,
    completed_at TIMESTAMP DEFAULT now()
);

CREATE TABLE commitments (
    id UUID PRIMARY KEY,
    todo_id UUID,
    stake_amount BIGINT,
    vault_pubkey TEXT,
    start_date DATE,
    last_checkin DATE,
    streak_current INT,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT now()
);