-- Users and sessions
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    email VARCHAR COLLATE NOCASE NOT NULL,
    display_name VARCHAR NOT NULL,
    password_hash VARCHAR NOT NULL,
    is_admin BOOLEAN NOT NULL DEFAULT 0
);
CREATE UNIQUE INDEX IF NOT EXISTS users_email_unique ON users (email COLLATE NOCASE);

CREATE TABLE IF NOT EXISTS sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users (id),
    token_hash VARCHAR NOT NULL,
    expires DATETIME NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS sessions_token_hash_unique ON sessions (token_hash);

-- Tsumego
CREATE TABLE IF NOT EXISTS tsumego (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR NOT NULL,
    board VARCHAR NOT NULL,
    tree VARCHAR NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS tsumego_name_unique on tsumego (name);

CREATE TABLE IF NOT EXISTS user_tsumego_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_id INTEGER NOT NULL REFERENCES user (id),
    tsumego_id INTEGER NOT NULL REFERENCES tsumego (id),
    in_rotation BOOLEAN NOT NULL,
    review_due DATETIME NOT NULL,
    streak_length INTEGER NOT NULL,
    interval FLOAT NOT NULL,
    e_factor FLOAT NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS tsumego_stats_per_user_unique on user_tsumego_stats (user_id, tsumego_id);
