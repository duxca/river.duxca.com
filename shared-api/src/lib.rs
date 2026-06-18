use leptos::prelude::*;

#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct ServerApiContext {
    pub db: sqlx::SqlitePool,
    pub user: model::user::User,
}

#[cfg(feature = "ssr")]
async fn call_api<T>(req: impl Into<model::api::Request>) -> Result<T, ServerFnError>
where
    T: TryFrom<model::api::Response>,
    T::Error: std::fmt::Display,
{
    let ctx =
        use_context::<ServerApiContext>().ok_or_else(|| ServerFnError::new("login required"))?;
    let res = service::handler(&ctx.db, &ctx.user, req.into())
        .await
        .map_err(ServerFnError::new)?;
    res.try_into().map_err(ServerFnError::new)
}

#[cfg(feature = "ssr")]
async fn call_api_result<T>(
    req: impl Into<model::api::Request>,
) -> Result<Result<T, model::api::ErrorKind>, ServerFnError>
where
    T: TryFrom<model::api::Response>,
    T::Error: std::fmt::Display,
{
    let ctx =
        use_context::<ServerApiContext>().ok_or_else(|| ServerFnError::new("login required"))?;
    let res = service::handler(&ctx.db, &ctx.user, req.into())
        .await
        .map_err(ServerFnError::new)?;
    match res {
        model::api::Response::Error(kind) => Ok(Err(kind)),
        res => res.try_into().map(Ok).map_err(ServerFnError::new),
    }
}

#[server(prefix = "/api", endpoint = "get_me", input = leptos::server_fn::codec::Json)]
pub async fn get_me() -> Result<model::api::get_me::Response, ServerFnError> {
    call_api(model::api::get_me::Request {}).await
}

#[server(prefix = "/api", endpoint = "list_users", input = leptos::server_fn::codec::Json)]
pub async fn list_users(
    offset: Option<i64>,
    limit: Option<i64>,
) -> Result<model::api::list_users::Response, ServerFnError> {
    call_api(model::api::list_users::Request { offset, limit }).await
}

#[server(prefix = "/api", endpoint = "list_access_logs", input = leptos::server_fn::codec::Json)]
pub async fn list_access_logs(
    offset: Option<i64>,
    limit: Option<i64>,
    user_id: Option<i64>,
) -> Result<model::api::list_access_logs::Response, ServerFnError> {
    call_api(model::api::list_access_logs::Request {
        offset,
        limit,
        user_id,
    })
    .await
}

#[server(prefix = "/api", endpoint = "list_rivers", input = leptos::server_fn::codec::Json)]
pub async fn list_rivers() -> Result<model::api::list_rivers::Response, ServerFnError> {
    call_api(model::api::list_rivers::Request {}).await
}

#[server(prefix = "/api", endpoint = "get_river", input = leptos::server_fn::codec::Json)]
pub async fn get_river(river_id: i64) -> Result<model::api::get_river::Response, ServerFnError> {
    call_api(model::api::get_river::Request { river_id }).await
}

#[server(prefix = "/api", endpoint = "create_river", input = leptos::server_fn::codec::Json)]
pub async fn create_river(
    name: String,
    latitude: f64,
    longitude: f64,
) -> Result<model::api::create_river::Response, ServerFnError> {
    call_api(model::api::create_river::Request {
        name,
        latitude,
        longitude,
    })
    .await
}

#[server(prefix = "/api", endpoint = "delete_river", input = leptos::server_fn::codec::Json)]
pub async fn delete_river(
    river_id: i64,
) -> Result<Result<model::api::delete_river::Response, model::api::ErrorKind>, ServerFnError> {
    call_api_result(model::api::delete_river::Request { river_id }).await
}

#[server(prefix = "/api", endpoint = "create_river_waypoint", input = leptos::server_fn::codec::Json)]
pub async fn create_river_waypoint(
    river_id: i64,
    name: String,
    latitude: f64,
    longitude: f64,
) -> Result<model::api::create_river_waypoint::Response, ServerFnError> {
    call_api(model::api::create_river_waypoint::Request {
        river_id,
        name,
        latitude,
        longitude,
    })
    .await
}

#[server(prefix = "/api", endpoint = "delete_river_waypoint", input = leptos::server_fn::codec::Json)]
pub async fn delete_river_waypoint(
    river_waypoint_id: i64,
) -> Result<Result<model::api::delete_river_waypoint::Response, model::api::ErrorKind>, ServerFnError>
{
    call_api_result(model::api::delete_river_waypoint::Request { river_waypoint_id }).await
}

#[server(prefix = "/api", endpoint = "create_river_track", input = leptos::server_fn::codec::Json)]
pub async fn create_river_track(
    river_id: i64,
    track_name: String,
    description: String,
    track: Vec<(f64, f64)>,
) -> Result<model::api::create_river_track::Response, ServerFnError> {
    call_api(model::api::create_river_track::Request {
        river_id,
        track_name,
        description,
        track,
    })
    .await
}

#[server(prefix = "/api", endpoint = "delete_river_track", input = leptos::server_fn::codec::Json)]
pub async fn delete_river_track(
    river_track_id: i64,
) -> Result<Result<model::api::delete_river_track::Response, model::api::ErrorKind>, ServerFnError>
{
    call_api_result(model::api::delete_river_track::Request { river_track_id }).await
}

#[server(prefix = "/api", endpoint = "delete_me", input = leptos::server_fn::codec::Json)]
pub async fn delete_me(
    nickname_confirm: String,
    confirm_delete: bool,
) -> Result<Result<model::api::delete_me::Response, model::api::ErrorKind>, ServerFnError> {
    call_api_result(model::api::delete_me::Request {
        nickname_confirm,
        confirm_delete,
    })
    .await
}
