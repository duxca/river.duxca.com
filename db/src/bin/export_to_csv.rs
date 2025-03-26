#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    use csv::Writer;
    use model::river::{RiverCsv, RiverTrackCsv, RiverWaypointCsv};

    let pool = db::connect("river.db").await?;
    let mut conn = pool.acquire().await?;

    // Export rivers
    let rivers = db::rivers::list_rivers_all(&mut conn).await?;
    let mut wtr = Writer::from_writer(std::io::stdout());
    for river in &rivers {
        let waypoint: Vec<f64> = serde_json::from_value(river.waypoint.clone())?;
        wtr.serialize(RiverCsv {
            river_id: river.river_id,
            river_name: river.river_name.clone(),
            latitude: waypoint[0],
            longitude: waypoint[1],
            created_at: river.created_at,
        })?;
    }
    wtr.flush()?;
    eprintln!("Exported rivers to stdout");

    // Export river tracks
    let mut wtr = Writer::from_writer(std::io::stdout());
    for river in &rivers {
        let tracks = db::river_tracks::list_river_tracks_all(&mut conn, river.river_id).await?;
        for track in tracks {
            wtr.serialize(RiverTrackCsv {
                river_track_id: track.river_track_id,
                river_id: track.river_id,
                user_id: track.user_id,
                track_name: track.track_name,
                description: track.description,
                track_json: track.track.to_string(),
                created_at: track.created_at,
                updated_at: track.updated_at,
            })?;
        }
    }
    wtr.flush()?;
    eprintln!("Exported river tracks to stdout");

    // Export river waypoints
    let mut wtr = Writer::from_writer(std::io::stdout());
    for river in &rivers {
        let waypoints =
            db::river_waypoints::list_river_waypoints_all(&mut conn, river.river_id).await?;
        for waypoint in waypoints {
            let wp: Vec<f64> = serde_json::from_value(waypoint.waypoint)?;
            wtr.serialize(RiverWaypointCsv {
                river_waypoint_id: waypoint.river_waypoint_id,
                river_id: waypoint.river_id,
                user_id: waypoint.user_id,
                waypoint_name: waypoint.waypoint_name,
                description: waypoint.description,
                latitude: wp[0],
                longitude: wp[1],
                created_at: waypoint.created_at,
                updated_at: waypoint.updated_at,
            })?;
        }
    }
    wtr.flush()?;
    eprintln!("Exported river waypoints to stdout");

    Ok(())
}
