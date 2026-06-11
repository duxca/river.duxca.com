#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "sql", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub user_id: i64,
    pub nickname: String,
    // admin: 0
    // user: 1
    pub role: i64,
    pub created_at: i64,
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "sql", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct UserAuth {
    pub user_auth_id: i64,
    pub user_id: i64,
    pub identifier: String,
    // github: 0
    // facebook: 1
    // twitter: 2
    pub identity_type: i64,
    pub created_at: i64,
}

#[cfg(feature = "login")]
impl axum_login::AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.user_id
    }

    fn session_auth_hash(&self) -> &[u8] {
        // This app authenticates users only through OAuth providers, so there
        // is no local password hash or session version to compare here. User
        // deletion still invalidates the session because AuthnBackend::get_user
        // returns None.
        &[]
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDeletePreview {
    pub river_count: i64,
    pub track_count: i64,
    pub waypoint_count: i64,
    pub auth_count: i64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "sql", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct AccessLog {
    pub access_log_id: i64,
    pub user_id: i64,
    pub request: String,
    pub created_at: i64,
}
