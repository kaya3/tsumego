CREATE TABLE IF NOT EXISTS challenges (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users (id),
    challenge VARCHAR NOT NULL,
    code_hash VARCHAR NOT NULL,
    expires DATETIME NOT NULL
);

ALTER TABLE users ADD COLUMN require_email_verification BOOLEAN NOT NULL DEFAULT 1;
ALTER TABLE users ADD COLUMN require_password_change BOOLEAN NOT NULL DEFAULT 0;
UPDATE users SET require_email_verification = 0;

DROP TABLE IF EXISTS user_verification_codes;
