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

CREATE TABLE IF NOT EXISTS user_verification_codes (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    email VARCHAR COLLATE NOCASE NOT NULL,
    display_name VARCHAR NOT NULL,
    password_hash VARCHAR NOT NULL,
    code_hash VARCHAR NOT NULL,
    expires DATETIME NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS verification_codes_email_unique ON user_verification_codes (email);

-- Tsumego
CREATE TABLE IF NOT EXISTS tsumego (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR NOT NULL,
    board VARCHAR NOT NULL,
    tree VARCHAR NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS tsumego_name_unique ON tsumego (name);

CREATE TABLE IF NOT EXISTS user_tsumego_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users (id),
    tsumego_id INTEGER NOT NULL REFERENCES tsumego (id),
    last_review_date DATETIME NOT NULL,
    review_due DATETIME,
    num_reviews INTEGER NOT NULL,
    streak_length INTEGER NOT NULL,
    interval FLOAT NOT NULL,
    e_factor FLOAT NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS tsumego_stats_per_user_unique ON user_tsumego_stats (user_id, tsumego_id);
CREATE INDEX IF NOT EXISTS tsumego_stats_by_due_date ON user_tsumego_stats (review_due);

CREATE TABLE IF NOT EXISTS user_tsumego_reviews (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users (id),
    tsumego_id INTEGER NOT NULL REFERENCES tsumego (id),
    review_date DATETIME NOT NULL,
    grade INTEGER CHECK(grade BETWEEN 0 AND 3) NOT NULL
);
CREATE INDEX IF NOT EXISTS tsumego_reviews_by_date ON user_tsumego_reviews (review_date);
