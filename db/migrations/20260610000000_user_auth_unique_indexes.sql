PRAGMA foreign_keys = ON;

CREATE UNIQUE INDEX user_auths_identity_identifier_unique
ON user_auths (identity_type, identifier);

CREATE UNIQUE INDEX user_auths_user_identity_unique
ON user_auths (user_id, identity_type);
