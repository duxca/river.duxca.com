#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Request {}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub user: Option<crate::user::User>,
}
