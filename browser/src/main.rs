mod api;
mod components;

use crate::components::map_component::{City, MapComponent, Point};
use gloo::console::log;
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct Cities {
    pub list: Vec<City>,
}

#[function_component(App)]
fn app() -> Html {
    let loggedin = use_state(|| false);
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
    // use_effect_with((), ...) で最初のみ実行
    use_effect_with((), {
        let loggedin = loggedin.clone();
        move |()| {
            wasm_bindgen_futures::spawn_local(async move {
                let res = crate::api::get_me().await;
                if let Ok(res) = res {
                    if let Some(_user) = res.user {
                        loggedin.set(true);
                    }
                }
            });
        }
    });
    use_effect_with(loggedin.clone(), {
        move |loggedin| {
            if **loggedin {
                wasm_bindgen_futures::spawn_local(async move {
                    use gloo::utils::format::JsValueSerdeExt;
                    let res = crate::api::call::<model::api::list_rivers::Response>(model::api::list_rivers::Request {offset:None, limit:None}).await.unwrap();
                    log!("Rivers: {:?}", wasm_bindgen::JsValue::from_serde(&res).unwrap());
                });
            }
        }
    });
    html! {
        <>
            if *loggedin {
                <MapComponent city={&*city}  />
                <div class="control">
                    <form method="post" action="/logout">
                        <input type="submit" value="Logout" />
                    </form>
                    <div>
                        {elms}
                    </div>
                </div>
            }else{
                <form method="post" action="/login">
                    <input type="submit" value="GitHub Login" />
                    <input type="hidden" name="provider" value="github" />
                </form>
            }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
