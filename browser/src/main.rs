use crate::components::map_component::{City, MapComponent, Point};
use gloo::console::log;
use yew::prelude::*;
mod components;

#[derive(PartialEq, Clone)]
pub struct Cities {
    pub list: Vec<City>,
}

#[function_component(App)]
fn app() -> Html {
    let cities = Cities {
        list: vec![City {
            name: "Fuji".to_string(),
            lat: Point(35.3622222, 138.7313889),
        }],
    };
    let city = use_state(|| cities.list[0].clone());
    let select_city_cb = Callback::from({
        let city = city.clone();
        move |city_: City| {
            city.set(city_);
        }
    });
    let cities = cities.list.clone();
    let elms = cities
        .into_iter()
        .map(|city| {
            let name = city.name.clone();
            let cb = Callback::from({
                let select_city = select_city_cb.clone();
                log!("Control: {:?}", format!("{:?}", city));
                move |_| select_city.emit(city.clone())
            });
            html! {
                <button onclick={cb}>{name}</button>
            }
        })
        .collect::<Html>();
    use_effect(|| {
        wasm_bindgen_futures::spawn_local(async move {
            use gloo::utils::format::JsValueSerdeExt;
            let txt = wasm_bindgen::JsValue::from_serde(&serde_json::json!({})).unwrap();
            let res = gloo::net::http::Request::post("/api")
                .body(txt)
                .unwrap()
                .send()
                .await
                .unwrap();
            let res = res.json::<serde_json::Value>().await.unwrap();
            log!("Response: {:?}", wasm_bindgen::JsValue::from_serde(&res).unwrap());
        });
        || log!("App unmounted")
    });
    html! {
        <>
            <MapComponent city={&*city}  />
            <div class="control">
                <form method="post" action="/login">
                    <input type="submit" value="GitHub Login" />
                    <input type="hidden" name="provider" value="github" />
                </form>
                <div>
                    {elms}
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
