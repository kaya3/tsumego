DROP TABLE IF EXISTS challenges;

ALTER TABLE users DROP COLUMN require_password_change;
ALTER TABLE users DROP COLUMN require_email_verification;

CREATE TABLE IF NOT EXISTS user_verification_codes (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    email VARCHAR COLLATE NOCASE NOT NULL,
    display_name VARCHAR NOT NULL,
    password_hash VARCHAR NOT NULL,
    code_hash VARCHAR NOT NULL,
    expires DATETIME NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS verification_codes_email_unique ON user_verification_codes (email);
