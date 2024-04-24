-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
    'dbec5e8d-2748-4068-a02d-9354020e36eb',
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1$6Ogi5jk9uSH3WtxvlaCl3g$i1LiNaI+CA/HP9E7B6j0uTAYe7QzIbr49wBllXJGGK0'
)
