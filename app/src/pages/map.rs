use leptos::prelude::*;
use leptos_leaflet::prelude::*;
use shared_api::list_rivers;

const DEFAULT_CENTER: Position = Position {
    lat: 35.362_222,
    lng: 138.731_389,
};

const GSI_ATTRIBUTION: &str = "<a href=\"https://maps.gsi.go.jp/development/ichiran.html\" target=\"_blank\">地理院タイル</a>";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BaseLayer {
    Osm,
    GsiStandard,
    GsiRedRelief,
}

impl BaseLayer {
    fn from_value(value: &str) -> Self {
        match value {
            "gsi-standard" => Self::GsiStandard,
            "gsi-red-relief" => Self::GsiRedRelief,
            _ => Self::Osm,
        }
    }

    const fn value(self) -> &'static str {
        match self {
            Self::Osm => "osm",
            Self::GsiStandard => "gsi-standard",
            Self::GsiRedRelief => "gsi-red-relief",
        }
    }

    const fn url(self) -> &'static str {
        match self {
            Self::Osm => "https://tile.openstreetmap.org/{z}/{x}/{y}.png",
            Self::GsiStandard => "https://cyberjapandata.gsi.go.jp/xyz/std/{z}/{x}/{y}.png",
            Self::GsiRedRelief => "https://cyberjapandata.gsi.go.jp/xyz/sekishoku/{z}/{x}/{y}.png",
        }
    }

    const fn attribution(self) -> &'static str {
        match self {
            Self::Osm => {
                "&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"
            }
            Self::GsiStandard | Self::GsiRedRelief => GSI_ATTRIBUTION,
        }
    }
}

fn river_position(river: &model::river::River) -> Option<Position> {
    let (lat, lng): (f64, f64) = serde_json::from_value(river.waypoint.clone()).ok()?;
    Some(Position { lat, lng })
}

#[cfg(feature = "ssr")]
async fn load_rivers() -> Result<model::api::list_rivers::Response, ServerFnError> {
    if use_context::<shared_api::ServerApiContext>().is_none() {
        return Err(ServerFnError::ServerError("login required".into()));
    }
    list_rivers().await
}

#[cfg(feature = "hydrate")]
async fn load_rivers() -> Result<model::api::list_rivers::Response, ServerFnError> {
    list_rivers().await
}

#[component]
pub fn MapPage() -> impl IntoView {
    let rivers = Resource::new(|| (), |_| load_rivers());
    let map_ready = RwSignal::new(false);
    let base_layer = RwSignal::new(BaseLayer::Osm);

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
                <div class="map-toolbar">
                    <label class="map-layer-select">
                        <span>"背景地図"</span>
                        <select
                            prop:value=move || base_layer.get().value()
                            on:change=move |ev| {
                                base_layer.set(BaseLayer::from_value(&event_target_value(&ev)));
                            }
                        >
                            <option value="osm">"OpenStreetMap"</option>
                            <option value="gsi-standard">"地理院地図"</option>
                            <option value="gsi-red-relief">"赤色立体図"</option>
                        </select>
                    </label>
                    <RiverStatus rivers/>
                </div>
            </header>
            <Show
                when=move || map_ready.get()
                fallback=|| view! { <div class="map-loading">"地図を読み込み中..."</div> }
            >
                <RiverMap rivers base_layer/>
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
    base_layer: RwSignal<BaseLayer>,
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
                        {move || {
                            let layer = base_layer.get();
                            view! {
                                <TileLayer
                                    url=layer.url()
                                    attribution=layer.attribution()
                                />
                            }
                        }}
                        {markers}
                    </MapContainer>
                }
            }}
        </Suspense>
    }
}
