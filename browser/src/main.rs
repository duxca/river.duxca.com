mod api;
mod components;

use crate::components::map_component::{MapComponent, Point};
use gloo::console;
use gloo::utils::format::JsValueSerdeExt;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component(App)]
#[allow(clippy::redundant_closure)]
fn app() -> Html {
    let loggedin = use_state(|| false);
    let forcus = use_state(|| {
        Point {
            latitude: 35.3622222,
            longitude: 138.7313889,
        } // Fuji
    });
    let selected_river_id = use_state(|| None);
    let rivers = use_state(|| Vec::<model::river::River>::new());
    let river_waypoints = use_state(|| Vec::<model::river::RiverWaypoint>::new());
    let select_river_cb = Callback::from({
        let selected_river_id = selected_river_id.clone();
        move |ev: Event| {
            let val = ev
                .target()
                .unwrap()
                .dyn_into::<web_sys::HtmlSelectElement>()
                .unwrap()
                .value();
            console::log!(&val);
            let river_id = val.parse::<i64>().unwrap();
            selected_river_id.set(Some(river_id));
        }
    });
    // 初回のみログインチェック
    use_effect_with((), {
        let loggedin = loggedin.clone();
        move |()| {
            wasm_bindgen_futures::spawn_local(async move {
                let res = crate::api::call::<model::api::get_me::Response>(
                    model::api::get_me::Request {},
                )
                .await;
                if let Ok(res) = res {
                    if let Some(_user) = res.user {
                        loggedin.set(true);
                    }
                }
            });
        }
    });
    // ログインしたら川の一覧を取得
    use_effect_with(loggedin.clone(), {
        let rivers = rivers.clone();
        move |loggedin| {
            if **loggedin {
                wasm_bindgen_futures::spawn_local(async move {
                    let res = crate::api::call::<model::api::list_rivers::Response>(
                        model::api::list_rivers::Request {
                            offset: None,
                            limit: Some(10000),
                        },
                    )
                    .await
                    .unwrap();
                    rivers.set(res.rivers);
                });
            }
        }
    });
    use_effect_with(rivers.clone(), {
        let river_waypoints = river_waypoints.clone();
        move |rivers| {
            wasm_bindgen_futures::spawn_local({
                let rivers = rivers.clone();
                async move {
                    let mut list = vec![];
                    for river in &**rivers {
                        let mut res =
                            crate::api::call::<model::api::list_river_waypoints::Response>(
                                model::api::list_river_waypoints::Request {
                                    offset: None,
                                    limit: Some(10000),
                                    river_id: river.river_id,
                                },
                            )
                            .await
                            .unwrap();
                        list.append(&mut res.river_waypoints);
                    }
                    river_waypoints.set(list);
                }
            });
        }
    });
    use_effect_with(selected_river_id.clone(), {
        let forcus = forcus.clone();
        move |selected_river_id| {
            if let Some(selected_river_id) = selected_river_id.as_ref() {
                wasm_bindgen_futures::spawn_local({
                    let selected_river_id = *selected_river_id;
                    async move {
                        let res = crate::api::call::<model::api::list_river_waypoints::Response>(
                            model::api::list_river_waypoints::Request {
                                offset: None,
                                limit: Some(1),
                                river_id: selected_river_id,
                            },
                        )
                        .await
                        .unwrap();
                        if !res.river_waypoints.is_empty() {
                            console::log!(&JsValue::from_serde(&res.river_waypoints[0]).unwrap());
                            forcus.set(Point {
                                latitude: res.river_waypoints[0].latitude,
                                longitude: res.river_waypoints[0].longitude,
                            });
                        }
                    }
                });
            }
        }
    });
    let points = river_waypoints
        .iter()
        .map(|p| Point {
            latitude: p.latitude,
            longitude: p.longitude,
        })
        .collect::<Vec<_>>();
    html! {
        <>
            if *loggedin {
                <MapComponent forcus={&*forcus} points={points} />
                <div class="control">
                    <form method="post" action="/logout">
                        <input type="submit" value="Logout" />
                    </form>

                    <label>
                        {"川:"}
                        <select name="river" size="1" onchange={select_river_cb}>
                            <option value="0">{"---"}</option>
                            {
                                rivers.iter().map(|river|{
                                    html!{
                                        <option value={river.river_id.to_string()}>{&river.name}</option>
                                    }
                                }).collect::<Html>()
                            }
                        </select>
                    </label>
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
    shadow_rs::shadow!(build);
    yew::Renderer::<App>::new().render();
}
