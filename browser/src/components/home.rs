//     <fieldset>
            //         <legend>{"Home"}</legend>
            //         <div>
            //             <label>
            //                 {"川:"}
            //                 <select id="river" size="1">
            //                     <option value="0">{"---"}</option>
            //                     {
            //                         rivers.iter().map(|river|{
            //                             html!{
            //                                 <option value={river.river_id.to_string()}>{&river.river_name}</option>
            //                             }
            //                         }).collect::<Html>()
            //                     }
            //                 </select>
            //             </label>
            //         </div>
            //         <div><button onclick={props.onclick_go_to_river.clone()}>{"go"}</button></div>
            //     </fieldset>

// use gloo::utils::format::JsValueSerdeExt;
use crate::components::sidebar_component::SidebarComponent;
use gloo::console;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew::suspense::{use_future, use_future_with, UseFutureHandle};


#[derive(Debug, PartialEq, Clone, Eq)]
enum PageState {
    LoggedOut,
    LoggedIn(model::user::User, EditMode),
}

#[derive(Debug, PartialEq, Clone, Eq)]
enum EditMode {
    Home,
    // AddRoute(AddRouteMode),
    // AddWaypoint,
    // AddRiver,
}

#[derive(Debug, PartialEq, Clone, Default, Eq)]
struct AddRouteMode {
    track: Vec<(
        ordered_float::OrderedFloat<f64>,
        ordered_float::OrderedFloat<f64>,
    )>,
    distance: ordered_float::OrderedFloat<f64>,
}

#[hook]
fn use_user() -> yew::suspense::SuspensionResult<Option<model::user::User>> {
    let s = use_future(|| async {
        let res =
            crate::api::call::<model::api::get_me::Response>(model::api::get_me::Request {}).await;
        match res {
            Ok(res) => Some(res.user),
            Err(_) => None,
        }
    })?;
    Ok((*s).clone())
}

#[function_component(Home)]
#[allow(clippy::redundant_closure)]
pub fn home() -> HtmlResult {
    let page_state = use_state_eq(|| PageState::LoggedOut);
    // Fuji
    let forcus = use_state_eq(|| (35.3622222, 138.7313889));
    // let side_menu_state = use_state(|| components::sidebar_component::SideMenuState::Closed);
    // let rivers = use_state(|| Vec::<model::river::River>::new());
    // let river_waypoints = use_state(|| Vec::<model::river::RiverWaypoint>::new());
    // let river_tracks = use_state(|| Vec::<model::river::RiverTrack>::new());

    // 初回のみログインチェック
    let user =  use_user()?;
    if let Some(user) = &user {
        page_state.set(PageState::LoggedIn(user.clone(), EditMode::Home));
    }

    let on_move = Callback::from({
        let forcus = forcus.clone();
        move |(lat, lng): (f64, f64)| {
            web_sys::window()
                .unwrap()
                .location()
                .set_hash(&format!("{},{}", lat, lng))
                .unwrap();
            forcus.set((lat, lng));
        }
    });
    let mut waypoints = vec![];
    let mut tracks = vec![];
    let html = match *page_state {
        PageState::LoggedOut => {
            html! {
                <>
                <form method="post" action="/login/twitter">
                    <input type="submit" value="twitter Login" />
                </form>
                <form method="post" action="/login/github">
                    <input type="submit" value="github Login" />
                </form>
                </>
            }
        }
        PageState::LoggedIn(_, _) => {
            html! {
                <crate::components::map_component::MapComponent
                    layer={crate::components::map_component::MapLayer::Gsi}
                    forcus={*forcus}
                    tracks={tracks}
                    waypoints={waypoints}
                    on_move={on_move}
                    />
            }
        }
    };
    Ok(html)
}
