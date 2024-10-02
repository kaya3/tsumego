-- initial password: "test"
INSERT INTO users (id, email, display_name, password_hash, is_admin) VALUES
    (1, 'email@example.com', 'kaya3', '$argon2id$v=19$m=19456,t=2,p=1$O4Z+20rppDMN73R9JqfimQ$aRXJZTxyqK58o3/TMlBDxaP60RLmOr6bYXHxIVtJ5vI', 1)
    ON CONFLICT DO NOTHING;
