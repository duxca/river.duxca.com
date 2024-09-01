use gloo::console;

pub async fn get_me() -> Result<model::api::get_me::Response, ()> {
    use gloo::utils::format::JsValueSerdeExt;
    let txt = wasm_bindgen::JsValue::from(
        serde_json::to_string(&model::api::Request::from(model::api::get_me::Request {})).unwrap(),
    );
    let res: gloo::net::http::Response = gloo::net::http::Request::post("/api")
        .credentials(web_sys::RequestCredentials::Include)
        .mode(web_sys::RequestMode::Cors)
        .header("Accept", "application/json")
        .header("content-type", "application/json")
        .body(txt)
        .unwrap()
        .send()
        .await
        .unwrap();
    if res.status() != 200 {
        console::log!("Error: {:?}", res.text().await.unwrap());
        return Err(());
    };
    let res = res.json::<model::api::Response>().await.unwrap();
    console::log!(
        "Response: {:?}",
        wasm_bindgen::JsValue::from_serde(&res).unwrap()
    );
    let res = model::api::get_me::Response::try_from(res).unwrap();
    Ok(res)
}

pub async fn call<T: TryFrom<model::api::Response, Error=impl core::fmt::Debug> + core::fmt::Debug>(
    req: impl Into<model::api::Request>,
) -> Result<T, anyhow::Error> {
    use gloo::utils::format::JsValueSerdeExt;
    let req = Into::<model::api::Request>::into(req);
    let txt = wasm_bindgen::JsValue::from(serde_json::to_string(&req).unwrap());
    console::log!("Request: {:?}", &txt);
    let res: gloo::net::http::Response = gloo::net::http::Request::post("/api")
        .credentials(web_sys::RequestCredentials::Include)
        .mode(web_sys::RequestMode::Cors)
        .header("Accept", "application/json")
        .header("content-type", "application/json")
        .body(txt)
        .unwrap()
        .send()
        .await?;
    if res.status() != 200 {
        let res = res.text().await?;
        console::log!("Error: {:?}", &res);
        return Err(anyhow::anyhow!("{}", res));
    };
    let res = res.json::<model::api::Response>().await?;
    console::log!(
        "Response: {:?}",
        wasm_bindgen::JsValue::from_serde(&res).unwrap()
    );
    if let model::api::Response::Error(e) = res {
        return Err(anyhow::anyhow!("{:?}", e));
    }
    let res = res.try_into().unwrap();
    Ok(res)
}
