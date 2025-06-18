//! Main application home component managing the river map interface.
//!
//! # Usage
//!
//! ```rust
//! use yew::prelude::*;
//!
//! #[function_component(App)]
//! pub fn app() -> Html {
//!     let user = model::user::User {
//!         user_id: 1,
//!         user_name: "John Doe".to_string(),
//!         // ... other user fields
//!     };
//!
//!     html! {
//!         <Home user={user} />
//!     }
//! }
//! ```
//!
//! ## Features
//! - Interactive map with multiple layers
//! - Sidebar with user settings and controls
//! - Multiple editing modes (Home, Search, AddRoute, AddWaypoint)
//! - Floating action buttons for different operations
//! - Dialog-based forms for adding rivers, routes, and waypoints
//! - Real-time data fetching and display of rivers, waypoints, and tracks
//!
//! ## Edit Modes
//! - **Home**: Default view with action buttons
//! - **Search**: River search and selection dialog
//! - **AddRoute**: Route creation with point-by-point editing
//! - **AddWaypoint**: Waypoint creation at current map center

use yew::prelude::*;

#[derive(Debug, PartialEq, Clone)]
enum EditMode {
    Home,
    Search,
    AddRoute(AddRouteMode),
    AddWaypoint,
}

#[derive(Debug, PartialEq, Clone, Default)]
struct AddRouteMode {
    state: AddRouteModeState,
    track: Vec<(f64, f64)>,
    distance: f64,
}

#[derive(Debug, PartialEq, Clone, Default)]
enum AddRouteModeState {
    #[default]
    Editing,
    Saving,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub user: model::user::User,
}

#[function_component(Home)]
#[allow(clippy::redundant_closure)]
pub fn home(Props { user: _ }: &Props) -> HtmlResult {
    let edit_mode = use_state_eq(|| EditMode::Home);
    let selected_river = use_state_eq(|| 0);
    let rivers = use_state_eq(Vec::<(i64, String)>::new);
    // map に表示する waypoints と tracks
    let waypoints = use_state_eq(std::collections::HashMap::<i64, (String, (f64, f64))>::new);
    let tracks = use_state_eq(std::collections::HashMap::<i64, Vec<(f64, f64)>>::new);
    // map の中心座標 (デフォは富士山) (lat, lng)
    let focus = use_state_eq(|| (35.3622222, 138.7313889));

    let on_move = use_callback((), {
        // use_callback に focus を置くと都度 callback が再生成されちゃう
        let focus = focus.clone();
        move |(lat, lng): (f64, f64), ()| {
            web_sys::window()
                .unwrap()
                .location()
                .set_hash(&format!("{},{}", lat, lng))
                .unwrap();
            focus.set((lat, lng));
        }
    });

    let onclick_go_to_home = use_callback(edit_mode.clone(), {
        move |_ev: MouseEvent, edit_mode| {
            edit_mode.set(EditMode::Home);
        }
    });
    let onclick_go_to_search = use_callback(edit_mode.clone(), {
        move |_ev: MouseEvent, edit_mode| {
            edit_mode.set(EditMode::Search);
        }
    });
    let onclick_go_to_add_route = use_callback(edit_mode.clone(), {
        move |_ev: MouseEvent, edit_mode| {
            edit_mode.set(EditMode::AddRoute(AddRouteMode::default()));
            log::debug!("onclick_go_to_add_route: {:?}", edit_mode);
        }
    });
    let onclick_add_route_point = use_callback(edit_mode.clone(), {
        //let edit_mode = edit_mode.clone();
        let tracks = tracks.clone();
        let focus = focus.clone();
        move |_ev: MouseEvent, edit_mode| {
            log::debug!("Add route point clicked, {:?}", edit_mode);
            if let EditMode::AddRoute(ref add_route_mode) = **edit_mode {
                log::debug!("Add route point clicked {:?}", add_route_mode);
                if add_route_mode.state == AddRouteModeState::Editing {
                    log::debug!("Adding route point");
                    // bug
                    let (lat, lng) = *focus;
                    let mut tmp_track = add_route_mode.track.clone();
                    let mut tmp_tracks = (*tracks).clone();
                    tmp_track.push((lat, lng));
                    tmp_tracks.insert(0, tmp_track.clone());
                    tracks.set(tmp_tracks.clone());
                    edit_mode.set(EditMode::AddRoute(AddRouteMode {
                        track: tmp_track,
                        ..add_route_mode.clone()
                    }));
                    // TODO: upload the new waypoint to the server
                }
            }
        }
    });
    let onclick_saving_route = use_callback(edit_mode.clone(), {
        move |_ev: MouseEvent, edit_mode| {
            if let EditMode::AddRoute(ref add_route_mode) = **edit_mode {
                if add_route_mode.state == AddRouteModeState::Editing {
                    edit_mode.set(EditMode::AddRoute(AddRouteMode {
                        state: AddRouteModeState::Saving,
                        ..add_route_mode.clone()
                    }));
                }
            }
        }
    });
    let onclick_save_route = use_callback(edit_mode.clone(), {
        move |_selected_river_id, edit_mode| {
            // TODO: upload route data to the server
            edit_mode.set(EditMode::Home);
        }
    });
    let onclick_go_to_add_waypoints = use_callback(edit_mode.clone(), {
        move |_ev: MouseEvent, edit_mode| {
            edit_mode.set(EditMode::AddWaypoint);
        }
    });
    let onclick_save_waypoint =
        use_callback((edit_mode.clone(), waypoints.clone(), focus.clone()), {
            move |(river_id, waypoint_name): (i64, String), (edit_mode, waypoints, focus)| {
                let edit_mode = edit_mode.clone();
                let waypoints = waypoints.clone();
                let focus = focus.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    log::debug!(
                        "Saving waypoint: river_id={}, name={}",
                        river_id,
                        waypoint_name
                    );

                    if river_id == 0 {
                        log::error!("No river selected for waypoint");
                        // TODO: Show error message to user
                        edit_mode.set(EditMode::Home);
                        return;
                    }

                    let (lat, lng) = *focus;
                    // API call to create waypoint
                    let res = crate::api::call::<model::api::create_river_waypoint::Response>(
                        model::api::create_river_waypoint::Request {
                            river_id,
                            name: waypoint_name.clone(),
                            latitude: lat,
                            longitude: lng,
                        },
                    )
                    .await;
                    match res {
                        Ok(response) => {
                            log::debug!("Waypoint created successfully: {:?}", response);
                            // Add the new waypoint to the local state for immediate display
                            let mut current_waypoints = (*waypoints).clone();
                            current_waypoints
                                .insert(response.river_waypoint_id, (waypoint_name, (lat, lng)));
                            waypoints.set(current_waypoints);
                            edit_mode.set(EditMode::Home);
                        }
                        Err(e) => {
                            log::error!("Failed to create waypoint: {:?}", e);
                            // TODO: Show error message to user
                            edit_mode.set(EditMode::Home);
                        }
                    }
                });
            }
        });

    // initial fetch
    use_effect_with((), {
        let rivers = rivers.clone();
        let waypoints = waypoints.clone();
        let tracks = tracks.clone();
        move |()| {
            let rivers = rivers.clone();
            let waypoints = waypoints.clone();
            let tracks = tracks.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let res = crate::api::call::<model::api::list_rivers::Response>(
                    model::api::list_rivers::Request {},
                )
                .await
                .unwrap();
                let mut wpts = vec![];
                let mut trks = vec![];
                for river in &res.rivers {
                    let mut res = crate::api::call::<model::api::get_river::Response>(
                        model::api::get_river::Request {
                            river_id: river.river_id,
                        },
                    )
                    .await
                    .unwrap();
                    wpts.append(&mut res.waypoints);
                    trks.append(&mut res.tracks);
                }
                let tmp_river = res
                    .rivers
                    .into_iter()
                    .map(model::river::River::<(f64, f64)>::from)
                    .map(|river| (river.river_id, river.river_name.clone()))
                    .collect::<Vec<_>>();
                rivers.set(tmp_river);

                let tmp_wpts = wpts
                    .into_iter()
                    .map(|a| model::river::RiverWaypoint::<(f64, f64)>::from(a))
                    .map(|wpt| (wpt.river_waypoint_id, (wpt.waypoint_name, wpt.waypoint)))
                    .collect::<std::collections::HashMap<_, _>>();
                let tmp_tracks = trks
                    .into_iter()
                    .map(model::river::RiverTrack::<Vec<(f64, f64)>>::from)
                    .map(|track| (track.river_track_id, track.track))
                    .collect::<std::collections::HashMap<_, _>>();

                waypoints.set(tmp_wpts);
                tracks.set(tmp_tracks);
            });
        }
    });

    log::debug!("edit_mode: {:?}", edit_mode);
    log::debug!("tracks: {:?}", *tracks);
    log::debug!("waypoint: {:?}", *waypoints);

    let html = html! {
        <>
        <crate::components::map::Map
            layer={crate::components::map::MapLayer::Gsi}
            focus={*focus}
            tracks={(*tracks).clone()}
            waypoints={(*waypoints).clone()}
            on_move={on_move}
        />
        <crate::components::sidebar::Sidebar>
            // TODO:ユーザ情報を載せる
            <form method="post" action="/logout">
                <input class="control-top-left-2th" type="submit" value="Logout" />
            </form>
            <div class="settings-group">
                <h4>{"表示設定"}</h4>
                <div class="setting-item">
                    <label>
                        <input type="checkbox" checked={true} />
                        <span>{"ウェイポイントを表示"}</span>
                    </label>
                </div>
                <div class="setting-item">
                    <label>
                        <input type="checkbox" checked={true} />
                        <span>{"トラックを表示"}</span>
                    </label>
                </div>
            </div>
            <div class="settings-group">
                <h4>{"地図スタイル"}</h4>
                <div class="setting-item">
                    <select>
                        <option value="gsi" selected={true}>{"地理院タイル"}</option>
                        <option value="osm">{"OpenStreetMap"}</option>
                        <option value="hillshade">{"陰影起伏図"}</option>
                        <option value="seamlessphoto">{"航空写真"}</option>
                    </select>
                </div>
            </div>
        </crate::components::sidebar::Sidebar>
        if *edit_mode == EditMode::Home {
            <crate::components::circle_button::CircleButton onclick={onclick_go_to_add_waypoints} bottom={1} icon={crate::components::circle_button::CircleButtonIcon::Flag} />
            <crate::components::circle_button::CircleButton onclick={onclick_go_to_add_route} bottom={2} icon={crate::components::circle_button::CircleButtonIcon::Polyline} />
            <crate::components::circle_button::CircleButton onclick={onclick_go_to_search} bottom={3} icon={crate::components::circle_button::CircleButtonIcon::Search} />
        } else if let EditMode::Search = &*edit_mode {
            <crate::components::dialog::Dialog title={"川検索"} onclose={onclick_go_to_home.clone()}>
                <crate::components::select_river::SelectRiver
                    selected_river={*selected_river}
                    rivers={(*rivers).clone()}
                    onchange={Callback::from(|_|{})}
                />
            </crate::components::dialog::Dialog>
        } else if let EditMode::AddRoute(AddRouteMode{state, ..}) = &*edit_mode {
           if *state == AddRouteModeState::Editing {
                <crate::components::circle_button::CircleButton onclick={onclick_add_route_point} bottom={1} icon={crate::components::circle_button::CircleButtonIcon::Plus} />
                <crate::components::circle_button::CircleButton onclick={onclick_saving_route} bottom={2} icon={crate::components::circle_button::CircleButtonIcon::Save} />
            }else if *state == AddRouteModeState::Saving {
                <crate::components::dialog::Dialog title={"道程追加"} onclose={onclick_go_to_home.clone()}>
                    <crate::components::add_route::AddRoute
                        selected_river={*selected_river}
                        rivers={(*rivers).clone()}
                        onsave={onclick_save_route}
                    />
                </crate::components::dialog::Dialog>
            }
        } else if *edit_mode == EditMode::AddWaypoint {
            <crate::components::dialog::Dialog title={"マーカー追加"} onclose={onclick_go_to_home.clone()}>
                <crate::components::add_waypoint::AddWaypoint
                    selected_river={*selected_river}
                    rivers={(*rivers).clone()}
                    focus={*focus}
                    onsave={onclick_save_waypoint}
                />
            </crate::components::dialog::Dialog>
        }
        </>
    };
    Ok(html)
}
