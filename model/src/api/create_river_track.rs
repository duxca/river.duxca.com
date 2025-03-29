#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub river_id: i64,
    pub track_name: String,
    pub description: String,
    // Array<[lat, long]>
    pub track: Vec<(f64, f64)>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub river_track_id: i64,
}
