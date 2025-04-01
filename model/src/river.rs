// T=String は CSV
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "sql", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct River<T = serde_json::Value> {
    pub river_id: i64,
    pub user_id: i64,
    pub river_name: String,
    // [lat, long]
    pub waypoint: T,
    pub description: String,
    pub created_at: i64,
}

impl From<River<serde_json::Value>> for River<(f64, f64)> {
    fn from(river: River) -> Self {
        let (latitude, longitude) = serde_json::from_value::<(f64, f64)>(river.waypoint).unwrap();
        River {
            user_id: river.user_id,
            river_id: river.river_id,
            river_name: river.river_name,
            waypoint: (latitude, longitude),
            description: river.description,
            created_at: river.created_at,
        }
    }
}

impl From<River<serde_json::Value>> for River<String> {
    fn from(river: River) -> Self {
        River {
            user_id: river.user_id,
            river_id: river.river_id,
            river_name: river.river_name,
            waypoint: serde_json::to_string(&river.waypoint).unwrap(),
            description: river.description,
            created_at: river.created_at,
        }
    }
}

impl TryFrom<River<String>> for River<serde_json::Value> {
    type Error = serde_json::Error;
    fn try_from(river: River<String>) -> Result<Self, Self::Error> {
        let waypoint = serde_json::from_str(&river.waypoint)?;
        Ok(River {
            user_id: river.user_id,
            river_id: river.river_id,
            river_name: river.river_name,
            waypoint,
            description: river.description,
            created_at: river.created_at,
        })
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "sql", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct RiverTrack<T = serde_json::Value> {
    pub river_track_id: i64,
    pub river_id: i64,
    pub user_id: i64,
    pub track_name: String,
    pub description: String,
    // Array<[lat, long]>
    pub track: T,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<RiverTrack<serde_json::Value>> for RiverTrack<Vec<(f64, f64)>> {
    fn from(river_track: RiverTrack) -> Self {
        let track = serde_json::from_value::<Vec<(f64, f64)>>(river_track.track).unwrap();
        RiverTrack {
            river_track_id: river_track.river_track_id,
            river_id: river_track.river_id,
            user_id: river_track.user_id,
            track_name: river_track.track_name,
            description: river_track.description,
            track,
            created_at: river_track.created_at,
            updated_at: river_track.updated_at,
        }
    }
}

impl From<RiverTrack<serde_json::Value>> for RiverTrack<String> {
    fn from(river_track: RiverTrack) -> Self {
        RiverTrack {
            river_track_id: river_track.river_track_id,
            river_id: river_track.river_id,
            user_id: river_track.user_id,
            track_name: river_track.track_name,
            description: river_track.description,
            track: serde_json::to_string(&river_track.track).unwrap(),
            created_at: river_track.created_at,
            updated_at: river_track.updated_at,
        }
    }
}

impl TryFrom<RiverTrack<String>> for RiverTrack<serde_json::Value> {
    type Error = serde_json::Error;
    fn try_from(river_track: RiverTrack<String>) -> Result<Self, Self::Error> {
        let track = serde_json::from_str(&river_track.track)?;
        Ok(RiverTrack {
            river_track_id: river_track.river_track_id,
            river_id: river_track.river_id,
            user_id: river_track.user_id,
            track_name: river_track.track_name,
            description: river_track.description,
            track,
            created_at: river_track.created_at,
            updated_at: river_track.updated_at,
        })
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "sql", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct RiverWaypoint<T = serde_json::Value> {
    pub river_waypoint_id: i64,
    pub river_id: i64,
    pub user_id: i64,
    pub waypoint_name: String,
    pub description: String,
    // [lat, long]
    pub waypoint: T,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<RiverWaypoint<serde_json::Value>> for RiverWaypoint<(f64, f64)> {
    fn from(river_waypoint: RiverWaypoint) -> Self {
        let waypoint = serde_json::from_value::<(f64, f64)>(river_waypoint.waypoint).unwrap();
        RiverWaypoint {
            river_waypoint_id: river_waypoint.river_waypoint_id,
            river_id: river_waypoint.river_id,
            user_id: river_waypoint.user_id,
            waypoint_name: river_waypoint.waypoint_name,
            description: river_waypoint.description,
            waypoint,
            created_at: river_waypoint.created_at,
            updated_at: river_waypoint.updated_at,
        }
    }
}

impl From<RiverWaypoint<serde_json::Value>> for RiverWaypoint<String> {
    fn from(river_waypoint: RiverWaypoint) -> Self {
        RiverWaypoint {
            river_waypoint_id: river_waypoint.river_waypoint_id,
            river_id: river_waypoint.river_id,
            user_id: river_waypoint.user_id,
            waypoint_name: river_waypoint.waypoint_name,
            description: river_waypoint.description,
            waypoint: serde_json::to_string(&river_waypoint.waypoint).unwrap(),
            created_at: river_waypoint.created_at,
            updated_at: river_waypoint.updated_at,
        }
    }
}

impl TryFrom<RiverWaypoint<String>> for RiverWaypoint<serde_json::Value> {
    type Error = serde_json::Error;
    fn try_from(river_waypoint: RiverWaypoint<String>) -> Result<Self, Self::Error> {
        let waypoint = serde_json::from_str(&river_waypoint.waypoint)?;
        Ok(RiverWaypoint {
            river_waypoint_id: river_waypoint.river_waypoint_id,
            river_id: river_waypoint.river_id,
            user_id: river_waypoint.user_id,
            waypoint_name: river_waypoint.waypoint_name,
            description: river_waypoint.description,
            waypoint,
            created_at: river_waypoint.created_at,
            updated_at: river_waypoint.updated_at,
        })
    }
}
