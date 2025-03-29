#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "sql", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct River {
    pub user_id: i64,
    pub river_id: i64,
    pub river_name: String,
    // [lat, long]
    pub waypoint: serde_json::Value,
    pub description: String,
    pub created_at: i64,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "sql", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct RiverTrack {
    pub river_track_id: i64,
    pub river_id: i64,
    pub user_id: i64,
    pub track_name: String,
    pub description: String,
    // Array<[lat, long]>
    pub track: serde_json::Value,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "sql", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct RiverWaypoint {
    pub river_waypoint_id: i64,
    pub river_id: i64,
    pub user_id: i64,
    pub waypoint_name: String,
    pub description: String,
    // [lat, long]
    pub waypoint: serde_json::Value,
    pub created_at: i64,
    pub updated_at: i64,
}
