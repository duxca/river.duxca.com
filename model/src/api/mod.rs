pub mod get_me;
pub mod get_river;
// pub mod list_access_logs;
pub mod list_rivers;
// pub mod list_users;
// pub mod update_river_waypoint;
// pub mod create_river_waypoint;

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
    // CreateRiverWaypoint(crate::api::create_river_waypoint::Request),
    GetMe(crate::api::get_me::Request),
    GetRiver(crate::api::get_river::Request),
    // ListUsers(crate::api::list_users::Request),
    // ListAccessLogs(crate::api::list_access_logs::Request),
    ListRivers(crate::api::list_rivers::Request),
    // UpdateRiverWaypoint(crate::api::update_river_waypoint::Request),
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
    // CreateRiverWaypoint(crate::api::create_river_waypoint::Response),
    GetMe(crate::api::get_me::Response),
    GetRiver(crate::api::get_river::Response),
    // ListUser(crate::api::list_users::Response),
    // ListAccessLogs(crate::api::list_access_logs::Response),
    ListRivers(crate::api::list_rivers::Response),
    // ListFieildSpot(crate::api::list_river_spots::Response),
    // UpdateRiverWaypoint(crate::api::update_river_waypoint::Response),
    Error(ErrorKind),
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
#[serde(tag = "errorType")]
#[serde(rename_all = "PascalCase")]
pub enum ErrorKind {
    PermissionDenied,
    InvalidRequest,
    NotFound,
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
                // crate::api::Request::ListUsers(..) => false,
                // crate::api::Request::ListAccessLogs(..) => false,
                crate::api::Request::ListRivers(..) => true,
                crate::api::Request::GetRiver(..) => true,
                // crate::api::Request::ListRiverWaypoints(..) => true,
                // model::api::Request::CreateRiverWaypoint(..) => false,
            };
            return flag;
        }
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
