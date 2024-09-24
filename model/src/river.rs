#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "sql", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct River {
    pub river_id: i64,
    pub name: String,
    // pub created_at: i64,
    // pub updated_at: i64,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "sql", derive(sqlx::FromRow))]
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

#[derive(Debug, serde::Deserialize)]
pub struct RiverCsv {
    pub field_name: String,
    pub point_name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub kind: Option<RiverKind>,
}

#[derive(Debug, serde::Deserialize)]
pub enum RiverKind {
    Port,
    Bridge,
    Rapids,
    Flow,
    Station,
    Parking,
    Toilet,
    RoadsideStation,
    Other,
}

#[test]
fn csv() {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .quote(b'"')
        .has_headers(false)
        .trim(csv::Trim::All)
        .from_path("../server/rivers.csv")
        .unwrap();
    for result in rdr.deserialize::<RiverCsv>() {
        let record = result.unwrap();
        println!("{:?}", record);
    }
}
