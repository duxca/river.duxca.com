#[derive(
    serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, frunk::LabelledGeneric,
)]
#[cfg_attr(feature = "sql", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub field_id: i64,
    pub field_name: String,
    pub description: String,
    // Array<[long, lat]>
    pub route: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, serde::Deserialize, Clone, PartialEq, Eq, frunk::LabelledGeneric)]
pub struct FieldCsv {
    pub field_name: String,
    pub description: String,
    pub route: String,
}

impl From<Field> for FieldCsv {
    fn from(csv: Field) -> Self {
        frunk::labelled::transform_from(csv)
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, frunk::LabelledGeneric)]
#[cfg_attr(feature = "sql", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct FieldSpot {
    pub field_spot_id: i64,
    pub field_id: i64,
    pub spot_name: String,
    pub spot_type: String,
    pub description: String,
    pub latitude: f64,
    pub longitude: f64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, serde::Deserialize, Clone, PartialEq, frunk::LabelledGeneric)]
pub struct FieldSpotCsv {
    pub field_name: String,
    pub spot_name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub spot_type: SpotType,
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

#[test]
fn csv() {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .quote(b'"')
        .has_headers(false)
        .trim(csv::Trim::All)
        .from_path("../server/field_spot.csv")
        .unwrap();
    for result in rdr.deserialize::<FieldSpotCsv>() {
        let record = result.unwrap();
        println!("{:?}", record);
    }
}
