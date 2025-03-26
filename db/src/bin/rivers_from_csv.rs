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

#[derive(Debug, serde::Deserialize, Clone, PartialEq)]
pub struct RiverSpotCsv {
    pub river_name: String,
    pub waypoint_name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub waypoint_type: SpotType,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum SpotType {
    Port,
    Bridge,
    Flow,
    Landmrk,
    Rapids,
    Note,
    Station,
    RoadsideStation,
    Parking,
    Toilet,
    Other,
}

fn main() {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .quote(b'"')
        .has_headers(false)
        .trim(csv::Trim::All)
        .from_path("../assets/field_spot.csv")
        .unwrap();
    let mut i = 0;
    let rows = rdr
        .deserialize::<RiverSpotCsv>()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let mut last_name = String::new();
    for record in &rows {
        if last_name != record.river_name {
            println!(
                "INSERT INTO rivers (river_name, waypoint) VALUES ('{}', json_array({},{}));",
                record.river_name, record.longitude, record.latitude
            );
            i += 1;
            last_name = record.river_name.clone();
        }
        println!(
            "INSERT INTO river_waypoints (river_id, user_id, waypoint_name, waypoint, description) VALUES ({}, {}, '{}', json_array({},{}), '');",
            i, 1, record.waypoint_name, record.longitude, record.latitude
        );
    }
}
//let mut rdr = csv::ReaderBuilder::new()
//    .delimiter(b',')
//    .quote(b'"')
//    .has_headers(false)
//    .trim(csv::Trim::All)
//    .from_path("./field_spot.csv")?;
//for result in rdr.deserialize::<model::field::FieldSpotCsv>() {
//    let spot = result?;
//    let mut conn = pool.acquire().await?;
//    println!("{:?}", spot);
//    crate::db::field::upsert_field_spot(
//        &mut *conn,
//        spot.field_name,
//        spot.spot_name,
//        spot.longitude,
//        spot.latitude,
//    )
//    .await?;
//}
