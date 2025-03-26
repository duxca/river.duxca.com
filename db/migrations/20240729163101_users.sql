PRAGMA foreign_keys = ON;

CREATE TABLE users (
    user_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    nickname TEXT NOT NULL,
    -- 0: admin
    -- 1: user
    role INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE user_auths (
    user_auth_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    -- 0: github
    -- 1: facebok
    -- 2: twitter
    identity_type INTEGER NOT NULL,
    -- github: bigint str
    -- facebok: bigint str
    -- twitter: bigint str
    identifier TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

CREATE TABLE access_logs (
    access_log_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    request TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

-- add admin user
INSERT INTO users (nickname, role) VALUES ('legokichi', 0);
INSERT INTO user_auths (user_id, identity_type, identifier) VALUES (1, 0, '2429307'); -- github
