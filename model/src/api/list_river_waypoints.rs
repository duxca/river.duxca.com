#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    pub river_id: i64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub river_waypoints: Vec<crate::river::RiverWaypoint>,
    pub next: i64,
    pub total: i64,
}
