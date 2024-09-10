#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "sql", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub user_id: i64,
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "sql", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct UserIdentity {
    pub user_id: i64,
    pub identifier: String,
    pub username: String,
    pub identity_provider_name: String,
}

#[cfg(feature = "login")]
impl axum_login::AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.user_id
    }

    fn session_auth_hash(&self) -> &[u8] {
        // ヤケクソ
        unsafe {
            std::slice::from_raw_parts(
                &self.user_id as *const i64 as *const u8,
                std::mem::size_of::<i64>(),
            )
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AccessLog {
    pub access_log_id: i64,
    pub user_id: i64,
    pub request: String,
    pub created_at: i64,
}
