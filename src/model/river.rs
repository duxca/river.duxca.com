#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct River {
    pub river_id: i64,
    pub name: String,
    // pub created_at: i64,
    // pub updated_at: i64,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct RiverWaypoint {
    pub river_waypoint_id: i64,
    pub river_id: i64,
    pub name: String,
    // pub description: String,
    pub latitude: f64,
    pub longitude: f64,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub elevation: Option<f64>,
    // pub created_at: i64,
    // pub updated_at: i64,
}
