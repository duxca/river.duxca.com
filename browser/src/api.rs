#![allow(unused_imports)]

pub async fn call<
    T: TryFrom<model::api::Response, Error = impl Sync + Send + std::error::Error + 'static>
        + core::fmt::Debug,
>(
    req: impl Into<model::api::Request>,
) -> Result<T, anyhow::Error> {
    use gloo::console;
    use gloo::net::http;
    let req = Into::<model::api::Request>::into(req);
    let txt = serde_json::to_string(&req)?;
    // console::log!("Request: {}", &txt);
    let res: gloo::net::http::Response = gloo::net::http::Request::post("/api")
        .credentials(web_sys::RequestCredentials::Include)
        .mode(web_sys::RequestMode::Cors)
        .header("Accept", "application/json")
        .header("content-type", "application/json")
        .body(txt)?
        .send()
        .await?;
    if res.status() != 200 {
        let res = res.text().await?;
        // console::log!("Error: {}", &res);
        return Err(anyhow::anyhow!("{}", res));
    };
    let res = res.text().await?;
    // console::log!("Response: {}", &res);
    let res = serde_json::from_str::<model::api::Response>(&res)?;
    if let model::api::Response::Error(e) = res {
        return Err(anyhow::anyhow!("{:?}", e));
    }
    let res = res.try_into()?;
    Ok(res)
}
