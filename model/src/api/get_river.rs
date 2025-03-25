#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub river_id: i64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub river: Option<crate::river::River>,
    pub waypoints: Vec<crate::river::RiverWaypoint>,
    pub tracks: Vec<crate::river::RiverTrack>,
}
