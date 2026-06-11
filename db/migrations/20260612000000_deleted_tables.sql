PRAGMA foreign_keys = ON;

CREATE TABLE deleted_users (
    user_id INTEGER NOT NULL PRIMARY KEY,
    nickname TEXT NOT NULL,
    role INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    deleted_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE deleted_user_auths (
    user_auth_id INTEGER NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    identity_type INTEGER NOT NULL,
    identifier TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    deleted_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE deleted_access_logs (
    access_log_id INTEGER NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    request TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    deleted_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE deleted_rivers (
    river_id INTEGER NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    river_name TEXT NOT NULL,
    waypoint JSON NOT NULL,
    description TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    deleted_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE deleted_river_tracks (
    river_track_id INTEGER NOT NULL PRIMARY KEY,
    river_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    track_name TEXT NOT NULL,
    description TEXT NOT NULL,
    track JSON NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    deleted_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE deleted_river_waypoints (
    river_waypoint_id INTEGER NOT NULL PRIMARY KEY,
    river_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    waypoint_name TEXT NOT NULL,
    description TEXT NOT NULL,
    waypoint JSON NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    deleted_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);