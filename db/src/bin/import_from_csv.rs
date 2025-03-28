#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    use model::river::{RiverCsv, RiverTrackCsv, RiverWaypointCsv};

    let pool = db::connect("river.db").await?;
    let mut conn = pool.acquire().await?;

    // Re-import rivers from CSV
    println!("Re-importing rivers from rivers.csv");
    let mut rdr = csv::Reader::from_path("rivers.csv")?;
    for result in rdr.deserialize() {
        let record: RiverCsv = result?;
        db::rivers::create_river(
            &mut conn,
            0,
            &record.river_name,
            (record.latitude, record.longitude),
        )
        .await?;
    }

    // Re-import river tracks from CSV
    println!("Re-importing river tracks from river_tracks.csv");
    let mut rdr = csv::Reader::from_path("river_tracks.csv")?;
    for result in rdr.deserialize() {
        let record: RiverTrackCsv = result?;
        let track: Vec<(f64, f64)> = serde_json::from_str(&record.track_json)?;
        db::river_tracks::create_river_track(
            &mut conn,
            record.river_id,
            record.user_id,
            &record.track_name,
            &track,
            Some(&record.description),
        )
        .await?;
    }

    // Re-import river waypoints from CSV
    println!("Re-importing river waypoints from river_waypoints.csv");
    let mut rdr = csv::Reader::from_path("river_waypoints.csv")?;
    for result in rdr.deserialize() {
        let record: RiverWaypointCsv = result?;
        db::river_waypoints::create_river_waypoint(
            &mut conn,
            record.river_id,
            record.user_id,
            record.waypoint_name,
            (record.latitude, record.longitude),
            &record.description,
        )
        .await?;
    }

    println!("Import completed successfully!");
    Ok(())
}
