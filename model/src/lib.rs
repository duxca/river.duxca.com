pub mod api;
pub mod river;
pub mod user;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct List<T> {
    pub offset: Option<u32>,
    pub list: T,
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "sql", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub file_id: i64,
    pub user_id: i64,
    pub content_type: String,
    pub gcs_path: String,
    pub file_size: i64,
    pub created_at: i64,
}
