pub mod api;
pub mod field;
pub mod user;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct List<T> {
    pub offset: Option<u32>,
    pub list: T,
}
