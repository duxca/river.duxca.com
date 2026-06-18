use leptos::prelude::*;
use leptos_leaflet::prelude::*;
use shared_api::{get_river, list_rivers};

const DEFAULT_CENTER: Position = Position {
    lat: 35.362_222,
    lng: 138.731_389,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MapLayer {
    Gsi,
    Osm,
    Hillshade,
    Blank,
    SeamlessPhoto,
}

impl MapLayer {
    const ALL: [Self; 5] = [
        Self::Gsi,
        Self::Osm,
        Self::Hillshade,
        Self::Blank,
        Self::SeamlessPhoto,
    ];

    fn value(self) -> &'static str {
        match self {
            Self::Gsi => "gsi",
            Self::Osm => "osm",
            Self::Hillshade => "hillshade",
            Self::Blank => "blank",
            Self::SeamlessPhoto => "seamless-photo",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Gsi => "地理院タイル",
            Self::Osm => "OpenStreetMap",
            Self::Hillshade => "陰影起伏図",
            Self::Blank => "白地図",
            Self::SeamlessPhoto => "航空写真",
        }
    }

    fn from_value(value: &str) -> Self {
        match value {
            "osm" => Self::Osm,
            "hillshade" => Self::Hillshade,
            "blank" => Self::Blank,
            "seamless-photo" => Self::SeamlessPhoto,
            _ => Self::Gsi,
        }
    }

    fn tile_url(self) -> &'static str {
        match self {
            Self::Gsi => "https://cyberjapandata.gsi.go.jp/xyz/std/{z}/{x}/{y}.png",
            Self::Osm => "https://tile.openstreetmap.org/{z}/{x}/{y}.png",
            Self::Hillshade => "https://cyberjapandata.gsi.go.jp/xyz/hillshademap/{z}/{x}/{y}.png",
            Self::Blank => "https://cyberjapandata.gsi.go.jp/xyz/blank/{z}/{x}/{y}.png",
            Self::SeamlessPhoto => {
                "https://cyberjapandata.gsi.go.jp/xyz/seamlessphoto/{z}/{x}/{y}.jpg"
            }
        }
    }

    fn attribution(self) -> &'static str {
        match self {
            Self::Osm => {
                "&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"
            }
            _ => {
                "<a href=\"https://maps.gsi.go.jp/development/ichiran.html\" target=\"_blank\">地理院タイル</a>"
            }
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct MapData {
    rivers: Vec<model::river::River>,
    waypoints: Vec<model::river::RiverWaypoint>,
    tracks: Vec<model::river::RiverTrack>,
}

fn river_position(river: &model::river::River) -> Option<Position> {
    let (lat, lng): (f64, f64) = serde_json::from_value(river.waypoint.clone()).ok()?;
    Some(Position { lat, lng })
}

fn waypoint_position(waypoint: &model::river::RiverWaypoint) -> Option<Position> {
    let (lat, lng): (f64, f64) = serde_json::from_value(waypoint.waypoint.clone()).ok()?;
    Some(Position { lat, lng })
}

fn track_positions(track: &model::river::RiverTrack) -> Option<Vec<Position>> {
    let points: Vec<(f64, f64)> = serde_json::from_value(track.track.clone()).ok()?;
    Some(
        points
            .into_iter()
            .map(|(lat, lng)| Position { lat, lng })
            .collect(),
    )
}

async fn load_map_data() -> Result<MapData, leptos::prelude::ServerFnError> {
    let response = list_rivers().await?;
    let mut waypoints = Vec::new();
    let mut tracks = Vec::new();

    for river in &response.rivers {
        let mut detail = get_river(river.river_id).await?;
        waypoints.append(&mut detail.waypoints);
        tracks.append(&mut detail.tracks);
    }

    Ok(MapData {
        rivers: response.rivers,
        waypoints,
        tracks,
    })
}

#[component]
pub fn MapPage() -> impl IntoView {
    let map_data = Resource::new(|| (), |_| load_map_data());
    let map_layer = RwSignal::new(MapLayer::Gsi);
    let show_waypoints = RwSignal::new(true);
    let show_tracks = RwSignal::new(true);
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
                <RiverControls
                    map_data
                    map_layer
                    show_waypoints
                    show_tracks
                />
            </header>
            <Show
                when=move || map_ready.get()
                fallback=|| view! { <div class="map-loading">"地図を読み込み中..."</div> }
            >
                <RiverMap
                    map_data
                    map_layer
                    show_waypoints
                    show_tracks
                />
            </Show>
        </div>
    }
}

#[component]
fn RiverControls(
    map_data: Resource<Result<MapData, leptos::prelude::ServerFnError>>,
    map_layer: RwSignal<MapLayer>,
    show_waypoints: RwSignal<bool>,
    show_tracks: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <div class="map-controls">
            <label>
                <span>"地図"</span>
                <select
                    on:change=move |ev| {
                        map_layer.set(MapLayer::from_value(&event_target_value(&ev)));
                    }
                >
                    {MapLayer::ALL.into_iter().map(|layer| view! {
                        <option value=layer.value() selected=move || map_layer.get() == layer>
                            {layer.label()}
                        </option>
                    }).collect_view()}
                </select>
            </label>
            <label class="map-toggle">
                <input
                    type="checkbox"
                    checked=move || show_waypoints.get()
                    on:change=move |ev| show_waypoints.set(event_target_checked(&ev))
                />
                <span>"地点"</span>
            </label>
            <label class="map-toggle">
                <input
                    type="checkbox"
                    checked=move || show_tracks.get()
                    on:change=move |ev| show_tracks.set(event_target_checked(&ev))
                />
                <span>"道程"</span>
            </label>
            <Suspense fallback=|| view! { <span class="river-count">"読み込み中..."</span> }>
                {move || match map_data.get() {
                    Some(Ok(data)) => view! {
                        <span class="river-count">{data.rivers.len()}" 件の川"</span>
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
        </div>
    }
}

#[component]
fn RiverMap(
    map_data: Resource<Result<MapData, leptos::prelude::ServerFnError>>,
    map_layer: RwSignal<MapLayer>,
    show_waypoints: RwSignal<bool>,
    show_tracks: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <Suspense fallback=|| view! { <div class="map-loading">"地図を読み込み中..."</div> }>
            {move || {
                let data = map_data.get().and_then(|result| result.ok());

                let center = data
                    .as_ref()
                    .and_then(|data| data.rivers.first())
                    .and_then(river_position)
                    .unwrap_or(DEFAULT_CENTER);

                view! {
                    <MapContainer
                        class="map"
                        center=center
                        zoom=12.0
                        scroll_wheel_zoom=true
                        set_view=true
                    >
                        {move || {
                            let layer = map_layer.get();
                            view! {
                                <TileLayer
                                    url=layer.tile_url()
                                    attribution=layer.attribution()
                                    max_zoom=18.0
                                />
                            }
                        }}
                        <MapOverlays
                            data=data
                            show_waypoints
                            show_tracks
                        />
                    </MapContainer>
                }
            }}
        </Suspense>
    }
}

#[component]
fn MapOverlays(
    data: Option<MapData>,
    show_waypoints: RwSignal<bool>,
    show_tracks: RwSignal<bool>,
) -> impl IntoView {
    let Some(data) = data else {
        return ().into_any();
    };
    let rivers = data.rivers;
    let waypoints = data.waypoints;
    let tracks = data.tracks;
    let marker_rivers = rivers.clone();
    let marker_waypoints = waypoints.clone();
    let track_data = tracks.clone();

    view! {
        {move || if show_waypoints.get() {
            let rivers = marker_rivers.clone();
            let waypoints = marker_waypoints.clone();
            view! {
                <>
                    {rivers.into_iter().filter_map(|river| {
                        let position = river_position(&river)?;
                        let name = river.river_name.clone();
                        Some(view! {
                            <Marker position=position title=name.clone()>
                                <Popup>
                                    <strong>{name}</strong>
                                </Popup>
                            </Marker>
                        })
                    }).collect_view()}
                    {waypoints.into_iter().filter_map(|waypoint| {
                        let position = waypoint_position(&waypoint)?;
                        let name = waypoint.waypoint_name.clone();
                        Some(view! {
                            <Marker position=position title=name.clone()>
                                <Popup>
                                    <strong>{name}</strong>
                                </Popup>
                            </Marker>
                        })
                    }).collect_view()}
                </>
            }.into_any()
        } else {
            ().into_any()
        }}
        {move || if show_tracks.get() {
            let tracks = track_data.clone();
            view! {
                <>
                    {tracks.into_iter().filter_map(|track| {
                        let positions = track_positions(&track)?;
                        let name = track.track_name.clone();
                        Some(view! {
                            <Polyline
                                positions=positions
                                color="#d33"
                                weight=5.0
                                opacity=0.55
                            >
                                <Popup>
                                    <strong>{name}</strong>
                                </Popup>
                            </Polyline>
                        })
                    }).collect_view()}
                </>
            }.into_any()
        } else {
            ().into_any()
        }}
        <div class="map-crosshair" aria-hidden="true"></div>
    }
    .into_any()
}
