use yew::prelude::*;
use yew::suspense::use_future;

#[hook]
fn use_rivers() -> yew::suspense::SuspensionResult<Vec<model::river::River<(f64, f64)>>> {
    let s = use_future(|| async {
        let res = crate::api::call::<model::api::list_rivers::Response>(
            model::api::list_rivers::Request {},
        )
        .await
        .unwrap();
        let s = res
            .rivers
            .into_iter()
            .map(model::river::River::<(f64, f64)>::from)
            .collect::<Vec<_>>();
        s
    })?;
    Ok((*s).clone())
}

#[derive(Debug, PartialEq, Clone)]
enum EditMode {
    Home,
    AddRoute(AddRouteMode),
    AddWaypoint,
    AddRiver,
}

#[derive(Debug, PartialEq, Clone, Default)]
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

#[derive(Properties, PartialEq)]
pub struct Props {
    pub user: model::user::User,
}

#[function_component(Home)]
#[allow(clippy::redundant_closure)]
pub fn home(&Props { ref user }: &Props) -> HtmlResult {
    let edit_mode = use_state_eq(|| EditMode::Home);

    // let rivers = use_state(|| Vec::<model::river::River>::new());
    // let river_waypoints = use_state(|| Vec::<model::river::RiverWaypoint>::new());
    // let river_tracks = use_state(|| Vec::<model::river::RiverTrack>::new());

    // Fuji
    let focus = use_state_eq(|| (35.3622222, 138.7313889));
    let on_move = use_callback(focus.clone(), {
        move |(lat, lng): (f64, f64), focus| {
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
    let onclick_go_to_add_route = use_callback(edit_mode.clone(), {
        move |_ev: MouseEvent, edit_mode| {
            edit_mode.set(EditMode::AddRoute(AddRouteMode::default()));
        }
    });
    let onclick_go_to_add_waypoints = use_callback(edit_mode.clone(), {
        move |_ev: MouseEvent, edit_mode| {
            edit_mode.set(EditMode::AddWaypoint);
        }
    });
    let onclick_go_to_add_river = use_callback(edit_mode.clone(), {
        move |_ev: MouseEvent, edit_mode| {
            edit_mode.set(EditMode::AddRiver);
        }
    });
    let selected_river = use_state_eq(|| 0);
    let rivers = use_rivers()?;
    let rivers = rivers
        .iter()
        .map(|river| (river.river_id, river.river_name.clone()))
        .collect::<Vec<_>>();

    let waypoints = use_state_eq(std::collections::HashMap::<i64, (String, (f64, f64))>::new);
    let tracks = use_state_eq(std::collections::HashMap::<i64, Vec<(f64, f64)>>::new);

    let html = html! {
        <>
        <crate::components::map_component::MapComponent
            layer={crate::components::map_component::MapLayer::Gsi}
            focus={*focus}
            tracks={(*tracks).clone()}
            waypoints={(*waypoints).clone()}
            on_move={on_move}
        />
        <crate::components::sidebar_component::SidebarComponent>
            <button onclick={onclick_go_to_home}>{"ホーム"}</button>
            <button onclick={onclick_go_to_add_route}>{"ルート追加"}</button>
            <button onclick={onclick_go_to_add_waypoints}>{"ポイント追加"}</button>
            <button onclick={onclick_go_to_add_river}>{"川追加"}</button>
            <form method="post" action="/logout">
                <input class="control-top-left-2th" type="submit" value="Logout" />
            </form>
        </crate::components::sidebar_component::SidebarComponent>
        if *edit_mode == EditMode::Home {
            <crate::components::select_river::SelectRiver
                selected_river={*selected_river}
                rivers={rivers.clone()}
                onchange={Callback::from(|_|{})}
            />
        } else if let EditMode::AddRoute(..) = &*edit_mode {
            <crate::components::add_route::AddRoute
                selected_river={*selected_river}
                rivers={rivers.clone()}
                focus={*focus}
                onclick_add_node={Callback::from(|_|{})}
                onsave={Callback::from(|_|{})}
            />
        } else if *edit_mode == EditMode::AddWaypoint {
            <crate::components::add_waypoint::AddWaypoint
                selected_river={*selected_river}
                rivers={rivers.clone()}
                focus={*focus}
                onsave={Callback::from(|_|{})}
            />
        } else if *edit_mode == EditMode::AddRiver {
            <crate::components::add_river::AddRiver
                focus={*focus}
                onsave={Callback::from(|_|{})}
            />
        }
        </>
    };
    Ok(html)
}
