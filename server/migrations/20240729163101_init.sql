PRAGMA foreign_keys = ON;

CREATE TABLE roles (
    role_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    role_name TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

INSERT INTO roles (role_name) VALUES ('admin');
INSERT INTO roles (role_name) VALUES ('default');

CREATE TABLE users (
    user_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    role_id INTEGER NOT NULL DEFAULT 2,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (role_id) REFERENCES roles(role_id) ON DELETE CASCADE
);

CREATE VIEW user_roles AS
SELECT
    users.user_id,
    roles.role_name
FROM users
JOIN roles ON users.role_id = roles.role_id;

CREATE TABLE user_auths (
    user_auth_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    identity_provider_id INTEGER NOT NULL,
    identifier TEXT NOT NULL,
    username TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
    FOREIGN KEY (identity_provider_id) REFERENCES identity_providers(identity_provider_id) ON DELETE CASCADE
);

CREATE TABLE identity_providers (
    identity_provider_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    identity_provider_name TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE VIEW user_auths_with_identity_providers AS
SELECT
    user_auths.user_id,
    user_auths.identifier,
    user_auths.username,
    identity_providers.identity_provider_name
FROM user_auths
JOIN identity_providers ON user_auths.identity_provider_id = identity_providers.identity_provider_id;

INSERT INTO identity_providers (identity_provider_name) VALUES ('github');
INSERT INTO identity_providers (identity_provider_name) VALUES ('facebook');

INSERT INTO users (role_id) VALUES (1); -- admin user
INSERT INTO user_auths (user_id, identity_provider_id, identifier, username) VALUES (1, 1, '2429307', 'legokichi');

CREATE TABLE access_logs (
    access_log_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    request TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);
