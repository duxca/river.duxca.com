#![allow(unused_imports)]
use crate::components::map_component::{MapComponent, MapLayer};
use gloo::console;
use gloo::utils::format::JsValueSerdeExt;
use wasm_bindgen::prelude::*;
// use web_sys::HtmlInputElement;
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
    last_point: Option<(f64, f64)>,
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
    let forcus = use_state(|| (35.3622222, 138.7313889));
    let edit_mode = use_state(|| EditMode::Home {});
    let selected_river_id = use_state(|| None);
    let rivers = use_state(|| Vec::<model::river::River>::new());
    let river_waypoints = use_state(|| Vec::<model::river::RiverWaypoint>::new());
    // map_stateは不要になったので削除

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
                    forcus.set((latitude, longitude));
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
        move |(lat, lng): (f64, f64)| {
            console::log!(lat, lng);
            forcus.set((lat, lng));
            // web_sys::window()
            //     .unwrap()
            //     .location()
            //     .set_hash(&format!("{},{}", forcus.latitude, forcus.longitude))
            //     .unwrap();
        }
    });
    // let on_add_route_point = Callback::from({
    //     let edit_mode = edit_mode.clone();
    //     move |(lat, lng)| {
    //         if let EditMode::AddRoute(ref mut o) = (*edit_mode).clone() {
    //             console::log!(lat, lng);

    //             // 新しいポイントを追加
    //             o.layers.push(std::rc::Rc::new(O(leaflet::Layer::from(
    //                 JsValue::from_str("marker"),
    //             ))));

    //             if let Some(pt_old) = o.last_point.as_ref() {
    //                 // 距離を計算（実際のマップオブジェクトなしで距離を計算する方法）
    //                 let lat1 = pt_old.latitude.to_radians();
    //                 let lon1 = pt_old.longitude.to_radians();
    //                 let lat2 = pt.latitude.to_radians();
    //                 let lon2 = pt.longitude.to_radians();

    //                 let r = 6371.0; // 地球の半径（km）
    //                 let dlon = lon2 - lon1;
    //                 let dlat = lat2 - lat1;

    //                 let a = (dlat / 2.0).sin().powi(2)
    //                     + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    //                 let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    //                 let distance = r * c * 1000.0; // メートルに変換

    //                 o.distance += distance;
    //                 o.layers.push(std::rc::Rc::new(O(leaflet::Layer::from(
    //                     JsValue::from_str("line"),
    //                 ))));
    //             }

    //             o.last_point = Some(pt);
    //             edit_mode.set(EditMode::AddRoute(o.clone()));
    //         }
    //     }
    // });

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
                            let (lat, lng) = serde_json::from_value::<(f64, f64)>(
                                res.waypoints[0].waypoint.clone(),
                            )
                            .unwrap();
                            forcus.set((lat, lng));
                        }
                    }
                });
            }
        }
    });

    // let on_add_river = Callback::from({
    //     move |(river_name, pt): (String, Point)| {
    //         console::log!(&river_name, pt.latitude, pt.longitude);

    //         // TODO マーカーのグループ化 ex. LayerGroup
    //         wasm_bindgen_futures::spawn_local({
    //             let river_name = river_name.clone();
    //             let pt = pt.clone();
    //             async move {
    //                 let res = crate::api::call::<model::api::create_river::Response>(
    //                     model::api::create_river::Request {
    //                         name: river_name,
    //                         latitude: pt.latitude,
    //                         longitude: pt.longitude,
    //                     },
    //                 )
    //                 .await
    //                 .unwrap();
    //             }
    //         });
    //     }
    // });
    let waypoints = vec![];
    // river_waypoints
    //     .iter()
    //     .map(|wpt| {
    //         let (lat, long) = serde_json::from_value::<(f64, f64)>(wpt.waypoint.clone()).unwrap();
    //         (
    //             wpt.waypoint_name.clone(),
    //             Point {
    //                 latitude: lat,
    //                 longitude: long,
    //             },
    //         )
    //     })
    //     .collect::<Vec<(String, Point)>>();
    let tracks = vec![];
    // river_waypoints
    //     .iter()
    //     .map(|wpt| {
    //         let (lat, long) = serde_json::from_value::<(f64, f64)>(wpt.waypoint.clone()).unwrap();
    //         (
    //             wpt.river_id,
    //             Point {
    //                 latitude: lat,
    //                 longitude: long,
    //             },
    //         )
    //     })
    //     .collect::<Vec<(i64, Point)>>();
    let onclick_add_route_cb = Callback::from({
        let edit_mode = edit_mode.clone();
        move |_| todo!()
    });
    let onclick_add_waypoint_cb = Callback::from({
        let edit_mode = edit_mode.clone();
        move |_| todo!()
    });
    let onclick_add_river_cb = Callback::from({
        let edit_mode = edit_mode.clone();
        move |_| todo!()
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
                    layer={MapLayer::GSI}
                    forcus={*forcus}
                    tracks={tracks}
                    waypoints={waypoints}
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
                            {{forcus.0}}
                            {{"."}}
                            {{forcus.1}}
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
