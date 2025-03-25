#[derive(
    serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq
)]
#[cfg_attr(feature = "sql", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct River {
    pub river_id: i64,
    pub river_name: String,
    // [lat, long]
    pub waypoint: String,
    pub created_at: i64,
}

#[derive(
    serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq,
)]
#[cfg_attr(feature = "sql", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct RiverTrack {
    pub river_track_id: i64,
    pub river_id: i64,
    pub user_id: i64,
    pub track_name: String,
    pub description: String,
    // Array<[lat, long]>
    pub track: String,
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
    pub waypoint: String,
    pub created_at: i64,
    pub updated_at: i64,
}

// #[derive(Debug, serde::Deserialize, Clone, PartialEq, Eq, frunk::LabelledGeneric)]
// pub struct RiverCsv {
//     pub river_name: String,
//     pub description: String,
//     pub route: String,
// }

// impl From<River> for RiverCsv {
//     fn from(csv: River) -> Self {
//         frunk::labelled::transform_from(csv)
//     }
// }


// #[derive(Debug, serde::Deserialize, Clone, PartialEq, frunk::LabelledGeneric)]
// pub struct RiverSpotCsv {
//     pub river_name: String,
//     pub waypoint_name: String,
//     pub latitude: f64,
//     pub longitude: f64,
//     pub waypoint_type: SpotType,
// }

// #[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
// pub enum SpotType {
//     Port,
//     Bridge,
//     Flow,
//     Landmrk,
//     Rapids,
//     Note,
//     Station,
//     RoadsideStation,
//     Parking,
//     Toilet,
//     Other,
// }

// #[test]
// fn csv() {
//     let mut rdr = csv::ReaderBuilder::new()
//         .delimiter(b',')
//         .quote(b'"')
//         .has_headers(false)
//         .trim(csv::Trim::All)
//         .from_path("../server/river_waypoint.csv")
//         .unwrap();
//     for result in rdr.deserialize::<RiverSpotCsv>() {
//         let record = result.unwrap();
//         println!("{:?}", record);
//     }
// }
