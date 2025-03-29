pub mod create_river;
pub mod create_river_track;
pub mod create_river_waypoint;
pub mod delete_river;
pub mod delete_river_track;
pub mod delete_river_waypoint;
pub mod get_me;
pub mod get_river;
pub mod list_access_logs;
pub mod list_rivers;
pub mod list_users;

#[derive(
    Debug,
    serde::Deserialize,
    serde::Serialize,
    Clone,
    PartialEq,
    derive_more::TryInto,
    derive_more::From,
)]
#[serde(tag = "type")]
#[serde(rename_all = "PascalCase")]
pub enum Request {
    GetMe(crate::api::get_me::Request),
    ListUsers(crate::api::list_users::Request),
    ListAccessLogs(crate::api::list_access_logs::Request),
    ListRivers(crate::api::list_rivers::Request),
    GetRiver(crate::api::get_river::Request),
    CreateRiver(crate::api::create_river::Request),
    DeleteRiver(crate::api::delete_river::Request),
    CreateRiverWaypoint(crate::api::create_river_waypoint::Request),
    DeleteRiverWaypoint(crate::api::delete_river_waypoint::Request),
    CreateRiverTrack(crate::api::create_river_track::Request),
    DeleteRiverTrack(crate::api::delete_river_track::Request),
}

#[derive(
    Debug,
    serde::Deserialize,
    serde::Serialize,
    Clone,
    PartialEq,
    derive_more::TryInto,
    derive_more::From,
)]
#[serde(tag = "type")]
#[serde(rename_all = "PascalCase")]
pub enum Response {
    GetMe(crate::api::get_me::Response),
    ListUser(crate::api::list_users::Response),
    ListAccessLogs(crate::api::list_access_logs::Response),
    ListRivers(crate::api::list_rivers::Response),
    GetRiver(crate::api::get_river::Response),
    CreateRiver(crate::api::create_river::Response),
    DeleteRiver(crate::api::delete_river::Response),
    CreateRiverWaypoint(crate::api::create_river_waypoint::Response),
    DeleteRiverWaypoint(crate::api::delete_river_waypoint::Response),
    CreateRiverTrack(crate::api::create_river_track::Response),
    DeleteRiverTrack(crate::api::delete_river_track::Response),
    Error(ErrorKind),
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
#[serde(tag = "errorType")]
#[serde(rename_all = "PascalCase")]
pub enum ErrorKind {
    PermissionDenied,
    InvalidRequest,
}

impl Request {
    #[tracing::instrument(level = "trace")]
    pub fn check_permission(&self, user: &crate::user::User) -> bool {
        if user.role == 0 {
            // admin
            return true;
        }
        if user.role == 1 {
            // default user
            let flag = match self {
                crate::api::Request::GetMe(..) => true,
                crate::api::Request::ListUsers(..) => false,
                crate::api::Request::ListAccessLogs(..) => false,
                crate::api::Request::ListRivers(..) => true,
                crate::api::Request::GetRiver(..) => true,
                crate::api::Request::CreateRiver(..) => false,
                crate::api::Request::DeleteRiver(..) => false,
                crate::api::Request::DeleteRiverWaypoint(..) => true,
                crate::api::Request::CreateRiverWaypoint(..) => true,
                crate::api::Request::CreateRiverTrack(..) => true,
                crate::api::Request::DeleteRiverTrack(..) => true,
            };
            return flag;
        }
        // unreachable
        false
    }

    #[allow(dead_code)]
    pub fn to_request_type_string(&self) -> Result<String, anyhow::Error> {
        use anyhow::Context;
        let json = serde_json::to_value(self)?;
        let json = json.pointer(".type").context("type river not found")?;
        let txt = json.as_str().context("type river is not a string")?;
        Ok(txt.to_string())
    }
}

impl Response {
    #[allow(dead_code)]
    pub fn to_response_type_string(&self) -> Result<String, anyhow::Error> {
        use anyhow::Context;
        let json = serde_json::to_value(self)?;
        let json = json.pointer(".type").context("type river not found")?;
        let txt = json.as_str().context("type river is not a string")?;
        Ok(txt.to_string())
    }
}
