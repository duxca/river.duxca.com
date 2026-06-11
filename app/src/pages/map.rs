use leptos::prelude::*;
use leptos_leaflet::prelude::*;
use shared_api::list_rivers;

const DEFAULT_CENTER: Position = Position {
    lat: 35.362_222,
    lng: 138.731_389,
};

fn river_position(river: &model::river::River) -> Option<Position> {
    let (lat, lng): (f64, f64) = serde_json::from_value(river.waypoint.clone()).ok()?;
    Some(Position { lat, lng })
}

#[component]
pub fn MapPage() -> impl IntoView {
    let rivers = Resource::new(|| (), |_| list_rivers());
    let map_ready = RwSignal::new(false);

    Effect::new(move |_| {
        map_ready.set(true);
    });

    view! {
        <div class="app-shell">
            <header class="topbar">
                <div>
                    <h1>"river.duxca.com"</h1>
                    <p>"Leptos + Leaflet"</p>
                </div>
                <RiverStatus rivers/>
            </header>
            <Show
                when=move || map_ready.get()
                fallback=|| view! { <div class="map-loading">"地図を読み込み中..."</div> }
            >
                <RiverMap rivers/>
            </Show>
        </div>
    }
}

#[component]
fn RiverStatus(
    rivers: Resource<Result<model::api::list_rivers::Response, leptos::prelude::ServerFnError>>,
) -> impl IntoView {
    view! {
        <Suspense fallback=|| view! { <span class="river-count">"読み込み中..."</span> }>
            {move || match rivers.get() {
                Some(Ok(response)) => view! {
                    <span class="river-count">{response.rivers.len()}" 件の川"</span>
                }
                .into_any(),
                Some(Err(_)) => view! {
                    <span class="river-count river-count--error">
                        "川データを取得できません（ログインが必要かもしれません）"
                    </span>
                }
                .into_any(),
                None => view! { <span class="river-count">"読み込み中..."</span> }.into_any(),
            }}
        </Suspense>
    }
}

#[component]
fn RiverMap(
    rivers: Resource<Result<model::api::list_rivers::Response, leptos::prelude::ServerFnError>>,
) -> impl IntoView {
    view! {
        <Suspense fallback=|| view! { <div class="map-loading">"地図を読み込み中..."</div> }>
            {move || {
                let river_data = rivers
                    .get()
                    .and_then(|result| result.ok())
                    .map(|response| response.rivers);

                let center = river_data
                    .as_ref()
                    .and_then(|rivers| rivers.first())
                    .and_then(river_position)
                    .unwrap_or(DEFAULT_CENTER);

                let markers = river_data.map(|rivers| {
                    rivers
                        .into_iter()
                        .filter_map(|river| {
                            let position = river_position(&river)?;
                            let name = river.river_name.clone();
                            Some(view! {
                                <Marker position=position>
                                    <Popup>
                                        <strong>{name}</strong>
                                    </Popup>
                                </Marker>
                            })
                        })
                        .collect_view()
                });

                view! {
                    <MapContainer
                        class="map"
                        center=center
                        zoom=12.0
                        scroll_wheel_zoom=true
                        set_view=true
                    >
                        <TileLayer
                            url="https://tile.openstreetmap.org/{z}/{x}/{y}.png"
                            attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"
                        />
                        {markers}
                    </MapContainer>
                }
            }}
        </Suspense>
    }
}
