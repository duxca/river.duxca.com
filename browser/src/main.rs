#![allow(unused_imports)]
use crate::components::map_component::{MapComponent, Point};
use gloo::console;
use gloo::utils::format::JsValueSerdeExt;
use model::river::RiverWaypoint;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;

mod api;
mod components;
#[derive(Debug, PartialEq, Clone, Eq)]
enum PageState {
    Loading,
    LoggedOut,
    LoggedIn(model::user::User),
}
#[derive(Debug, PartialEq, Clone)]
enum EditMode {
    Home,
    AddRoute(AddRouteMode),
    AddWaypoint,
    AddRiver(AddRiverMode),
}
#[derive(Debug, PartialEq, Clone, Default)]
struct AddRouteMode {
    last_point: Option<Point>,
    distance: f64,
    layers: Vec<std::rc::Rc<O>>,
}
#[derive(Debug, PartialEq, Clone, Default)]
struct AddRiverMode {}
#[derive(Debug, PartialEq, Clone)]
struct O(leaflet::Layer);
impl Drop for O {
    fn drop(&mut self) {
        console::log!("drop");
        self.0.remove();
    }
}

#[function_component(App)]
#[allow(clippy::redundant_closure)]
fn app() -> Html {
    let page_state = use_state(|| PageState::Loading);
    // Fuji
    let forcus = use_state(|| Point {
        latitude: 35.3622222,
        longitude: 138.7313889,
    });
    let edit_mode = use_state(|| EditMode::Home {});
    let selected_river_id = use_state(|| None);
    let rivers = use_state(|| Vec::<model::river::River>::new());
    let river_waypoints = use_state(|| Vec::<model::river::RiverWaypoint>::new());
    let map_state = use_state(|| None);

    // 初回のみログインチェック
    use_effect_with((), {
        let page_state = page_state.clone();
        move |()| {
            wasm_bindgen_futures::spawn_local(async move {
                let res = crate::api::call::<model::api::get_me::Response>(
                    model::api::get_me::Request {},
                )
                .await;
                if res.is_err() {
                    return;
                }
                page_state.set(PageState::LoggedIn(res.unwrap().user));
            });
        }
    });
    use_effect_with(page_state.clone(), {
        let rivers = rivers.clone();
        let forcus = forcus.clone();
        move |page_state| {
            let page_state = page_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if *page_state == PageState::Loading {
                    return;
                }
                if *page_state == PageState::LoggedOut {
                    return;
                }
                let url_hash = web_sys::window().unwrap().location().hash().unwrap();
                if let Some(hash) = url_hash.split('#').nth(1) {
                    let mut latlng = hash.split(',');
                    let latitude = latlng.next().unwrap().parse::<f64>().unwrap();
                    let longitude = latlng.next().unwrap().parse::<f64>().unwrap();
                    forcus.set(Point {
                        latitude,
                        longitude,
                    });
                }
                // ログインしたら川の一覧を取得
                let res = crate::api::call::<model::api::list_rivers::Response>(
                    model::api::list_rivers::Request {},
                )
                .await
                .unwrap();
                // 川の一覧を描画
                rivers.set(res.rivers);
            });
        }
    });
    use_effect_with(rivers.clone(), {
        let river_waypoints = river_waypoints.clone();
        move |rivers| {
            let rivers = rivers.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let mut list = vec![];
                for river in &*rivers {
                    let mut res = crate::api::call::<model::api::get_river::Response>(
                        model::api::get_river::Request {
                            river_id: river.river_id,
                        },
                    )
                    .await
                    .unwrap();
                    list.append(&mut res.waypoints);
                }
                // 都度再描画
                river_waypoints.set(list.clone());
            });
        }
    });



    let select_river_cb = Callback::from({
        let selected_river_id = selected_river_id.clone();
        move |ev: Event| {
            let val = ev
                .target()
                .unwrap()
                .dyn_into::<web_sys::HtmlSelectElement>()
                .unwrap()
                .value();
            // console::log!(&val);
            let river_id = val.parse::<i64>().unwrap();
            selected_river_id.set(Some(river_id));
        }
    });
    
    let on_move = Callback::from({
        let forcus = forcus.clone();
        move |pt: Point| {
            forcus.set(pt);
        }
    });
    let map_ready_cb = Callback::from({
        let map_state = map_state.clone();
        move |map: leaflet::Map| {
            map_state.set(Some(map));
        }
    });
    let onclick_add_waypoint_cb = Callback::from({
        let map_state = map_state.clone();
        let selected_river_id = selected_river_id.clone();
        move |_: MouseEvent| {
            if let Some(map) = map_state.as_ref() {
                let title = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id("waypoint_name")
                    .unwrap()
                    .dyn_ref::<HtmlInputElement>()
                    .unwrap()
                    .value();
                let latlng = map.get_center();
                let pt = Point {
                    latitude: latlng.lat(),
                    longitude: latlng.lng(),
                };
                console::log!(&title, pt.latitude, pt.longitude);
                let p = leaflet::Popup::new(&leaflet::PopupOptions::default(), None);
                p.set_content(&JsValue::from_serde(&serde_json::json!(title)).unwrap());
                leaflet::Marker::new(&leaflet::LatLng::new(pt.latitude, pt.longitude))
                    .bind_popup(&p)
                    .open_popup()
                    .add_to(map);
                // TODO マーカーのグループ化 ex. LayerGroup
                wasm_bindgen_futures::spawn_local({
                    let selected_river_id = selected_river_id.clone();
                    async move {
                        let res = crate::api::call::<model::api::create_river_waypoint::Response>(
                            model::api::create_river_waypoint::Request {
                                river_id: selected_river_id.as_ref().unwrap().clone(),
                                name: title,
                                latitude: pt.latitude,
                                longitude: pt.longitude,
                            },
                        )
                        .await
                        .unwrap();
                    }
                });
            }
        }
    });
    let onclick_add_route_cb = Callback::from({
        let map_state = map_state.clone();
        let edit_mode = edit_mode.clone();
        move |_: MouseEvent| {
            if let Some(map) = map_state.as_ref() {
                if let EditMode::AddRoute(ref mut o) = (*edit_mode).clone() {
                    let latlng = map.get_center();
                    let pt = Point {
                        latitude: latlng.lat(),
                        longitude: latlng.lng(),
                    };
                    console::log!(pt.latitude, pt.longitude);
                    let mark =
                        leaflet::Marker::new(&leaflet::LatLng::new(pt.latitude, pt.longitude))
                            .add_to(map);
                    o.layers.push(std::rc::Rc::new(O(mark)));
                    if let Some(pt_old) = o.last_point.as_ref() {
                        let a = leaflet::LatLng::new(pt_old.latitude, pt_old.longitude);
                        let b = leaflet::LatLng::new(pt.latitude, pt.longitude);
                        let arr = &[&a, &b]
                            .into_iter()
                            .map(JsValue::from)
                            .collect::<js_sys::Array>();
                        let line = leaflet::Polyline::new(arr).add_to(map);
                        o.layers.push(std::rc::Rc::new(O(line)));
                        console::log!(map.distance(&a, &b));
                        o.distance += map.distance(&a, &b);
                    }
                    o.last_point = Some(pt);
                    edit_mode.set(EditMode::AddRoute(o.clone()));
                }
            }
        }
    });

    // 選択された川が変化したら川のウェイポイントを取得してフォーカスする
    use_effect_with(selected_river_id.clone(), {
        let forcus = forcus.clone();
        move |selected_river_id| {
            if let Some(selected_river_id) = selected_river_id.as_ref() {
                wasm_bindgen_futures::spawn_local({
                    let selected_river_id = *selected_river_id;
                    async move {
                        let res = crate::api::call::<model::api::get_river::Response>(
                            model::api::get_river::Request {
                                river_id: selected_river_id,
                            },
                        )
                        .await
                        .unwrap();
                        if !res.waypoints.is_empty() {
                            // console::log!(&JsValue::from_serde(&res.river_waypoints[0]).unwrap());
                            let (latitude, longitude) = serde_json::from_value::<(f64, f64)>(
                                res.waypoints[0].waypoint.clone(),
                            )
                            .unwrap();
                            forcus.set(Point {
                                latitude,
                                longitude,
                            });
                        }
                    }
                });
            }
        }
    });

    let onclick_add_river_cb = Callback::from({
        let map_state = map_state.clone();
        move |_: MouseEvent| {
            if let Some(map) = map_state.as_ref() {
                let river_name = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id("river_name")
                    .unwrap()
                    .dyn_ref::<HtmlInputElement>()
                    .unwrap()
                    .value();
                let latlng = map.get_center();
                let pt = Point {
                    latitude: latlng.lat(),
                    longitude: latlng.lng(),
                };
                console::log!(&river_name, pt.latitude, pt.longitude);
                let p = leaflet::Popup::new(&leaflet::PopupOptions::default(), None);
                p.set_content(&JsValue::from_serde(&serde_json::json!(river_name)).unwrap());
                leaflet::Marker::new(&leaflet::LatLng::new(pt.latitude, pt.longitude))
                    .bind_popup(&p)
                    .open_popup()
                    .add_to(map);
                // TODO マーカーのグループ化 ex. LayerGroup
                wasm_bindgen_futures::spawn_local({
                    async move {
                        let res = crate::api::call::<model::api::create_river::Response>(
                            model::api::create_river::Request {
                                name: river_name,
                                latitude: pt.latitude,
                                longitude: pt.longitude,
                            },
                        )
                        .await
                        .unwrap();
                    }
                });
            }
        }
    });

    match *page_state {
        PageState::Loading => {
            html! {
                <>
                <div>{"Loading..."}</div>
                </>
            }
        }
        PageState::LoggedOut => {
            html! {
                <>
                <form method="post" action="/login/github">
                <input type="submit" value="GitHub Login" />
                </form>
                <form method="post" action="/login/facebook">
                    <input type="submit" value="Facebook Login" />
                </form>
                <form method="post" action="/login/twitter">
                    <input type="submit" value="twitter Login" />
                </form>
                </>
            }
        }
        PageState::LoggedIn(ref user) => {
            html! {
                <>
                <MapComponent
                    forcus={*forcus}
                    markers={river_waypoints.iter().map(|wpt|{
                        let (lat, long) =  serde_json::from_value::<(f64, f64)>(wpt.waypoint.clone()).unwrap();
                        (wpt.waypoint_name.clone(), Point{
                            latitude: lat,
                            longitude: long,
                        })
                    }).collect::<Vec<(String, Point)>>()}
                    on_ready={map_ready_cb}
                    on_move={on_move} />
                <div class="control">
                    <fieldset>
                        <legend>{"Edit Mode"}</legend>
                        <div>
                            <button onclick={Callback::from({
                                let edit_mode = edit_mode.clone();
                                move |_| edit_mode.set(EditMode::Home{})}
                            )}>
                                {"Home"}
                            </button>
                        </div>
                        <div>
                            <button onclick={Callback::from({
                                let edit_mode = edit_mode.clone();
                                move |_| edit_mode.set(EditMode::AddRoute(AddRouteMode::default()))
                            })}>
                                {"Route"}
                            </button>
                        </div>
                        <div>
                            <button
                                onclick={Callback::from({
                                    let edit_mode = edit_mode.clone();
                                    move |_| edit_mode.set(EditMode::AddWaypoint{
                                    })
                                })}
                            >
                                {"Waypoint"}
                            </button>
                        </div>
                        if user.role == 0 {
                            <div>
                                <button
                                    onclick={Callback::from({
                                        let edit_mode = edit_mode.clone();
                                        move |_| edit_mode.set(EditMode::AddRiver(AddRiverMode::default()))
                                    })}
                                >
                                    {"River"}
                                </button>
                            </div>
                        }
                    </fieldset>
                    if let EditMode::Home{} = *edit_mode {
                        <fieldset>
                            <legend>{"Home"}</legend>
                            <div>
                                <label>
                                    {"川:"}
                                    <select name="river" size="1" onchange={select_river_cb}>
                                        <option value="0">{"---"}</option>
                                        {
                                            rivers.iter().map(|river|{
                                                html!{
                                                    <option value={river.river_id.to_string()}>{&river.river_name}</option>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </select>
                                </label>
                            </div>
                        </fieldset>
                    } else if let EditMode::AddRoute(ref o) = *edit_mode {
                        <fieldset>
                            <legend>{"addRoute"}</legend>
                            <div><button onclick={onclick_add_route_cb}>{"add point"}</button></div>
                            <div>{format!("distance: {} m", o.distance.round() as i64)}</div>
                        </fieldset>
                    } else if let EditMode::AddWaypoint{} = *edit_mode {
                        <fieldset>
                            <legend>{"AddWaypoint"}</legend>
                            <input type="text" id="waypoint_name" />
                            <label>
                                    {"川:"}
                                    <select name="river" size="1" onchange={select_river_cb}>
                                        <option value="0">{"---"}</option>
                                        {
                                            rivers.iter().map(|river|{
                                                html!{
                                                    <option value={river.river_id.to_string()}>{&river.river_name}</option>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </select>
                                </label>
                            <div><button onclick={onclick_add_waypoint_cb}>{"add point"}</button></div>
                        </fieldset>
                    } else if let EditMode::AddRiver(ref o) = *edit_mode {
                        <fieldset>
                            <legend>{"addRiver"}</legend>
                            <div><input type="text" id={"river_name"} /></div>
                            {{forcus.latitude}}
                            {{"."}}
                            {{forcus.longitude}}
                            <div><button onclick={onclick_add_river_cb}>{"add river"}</button></div>
                        </fieldset>
                    }
                    <fieldset>
                        <legend>{"Account"}</legend>
                        <form method="post" action="/logout">
                            <input type="submit" value="Logout" />
                        </form>
                    </fieldset>
                </div>
                </>
            }
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
