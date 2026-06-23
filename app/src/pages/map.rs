use leptos::prelude::*;
use leptos_leaflet::prelude::*;
use shared_api::{create_river_track, get_river, list_rivers};

const DEFAULT_CENTER: Position = Position {
    lat: 35.362_222,
    lng: 138.731_389,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MapLayer {
    Gsi,
    RedRelief,
    Osm,
    Hillshade,
    Blank,
    SeamlessPhoto,
}

impl MapLayer {
    const ALL: [Self; 6] = [
        Self::Gsi,
        Self::RedRelief,
        Self::Osm,
        Self::Hillshade,
        Self::Blank,
        Self::SeamlessPhoto,
    ];

    fn value(self) -> &'static str {
        match self {
            Self::Gsi => "gsi",
            Self::RedRelief => "red-relief",
            Self::Osm => "osm",
            Self::Hillshade => "hillshade",
            Self::Blank => "blank",
            Self::SeamlessPhoto => "seamless-photo",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Gsi => "地理院タイル",
            Self::RedRelief => "赤色立体図",
            Self::Osm => "OpenStreetMap",
            Self::Hillshade => "陰影起伏図",
            Self::Blank => "白地図",
            Self::SeamlessPhoto => "航空写真",
        }
    }

    fn from_value(value: &str) -> Self {
        match value {
            "red-relief" => Self::RedRelief,
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
            Self::RedRelief => "https://cyberjapandata.gsi.go.jp/xyz/sekishoku/{z}/{x}/{y}.png",
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

fn draft_track_points(positions: &[Position]) -> Vec<(f64, f64)> {
    positions
        .iter()
        .map(|position| (position.lat, position.lng))
        .collect()
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
    let selected_river_id = RwSignal::new(None::<i64>);
    let editing = RwSignal::new(false);
    let draft_track = RwSignal::new(Vec::<Position>::new());
    let show_track_form = RwSignal::new(false);
    let track_name = RwSignal::new(String::new());
    let track_description = RwSignal::new(String::new());
    let form_error = RwSignal::new(None::<String>);
    let submit_result = RwSignal::new(None::<String>);
    let create_track = Action::new(move |input: &(i64, String, String, Vec<Position>)| {
        let (river_id, track_name, description, positions) = input.clone();
        async move {
            create_river_track(
                river_id,
                track_name,
                description,
                draft_track_points(&positions),
            )
            .await
        }
    });

    Effect::new(move |_| {
        map_ready.set(true);
    });

    Effect::new(move |_| {
        if selected_river_id.get().is_some() {
            return;
        }
        if let Some(Ok(data)) = map_data.get()
            && let Some(river) = data.rivers.first()
        {
            selected_river_id.set(Some(river.river_id));
        }
    });

    Effect::new(move |_| match create_track.value().get() {
        Some(Ok(response)) => {
            draft_track.set(Vec::new());
            show_track_form.set(false);
            track_name.set(String::new());
            track_description.set(String::new());
            form_error.set(None);
            submit_result.set(Some(format!(
                "経路を投稿しました #{}",
                response.river_track_id
            )));
            map_data.refetch();
        }
        Some(Err(_)) => {
            form_error.set(Some("経路の投稿に失敗しました。".to_string()));
        }
        None => {}
    });

    let begin_edit = move |_| {
        editing.set(true);
        show_track_form.set(false);
        form_error.set(None);
        submit_result.set(None);
    };
    let cancel_edit = move |_| {
        editing.set(false);
        show_track_form.set(false);
        draft_track.set(Vec::new());
        form_error.set(None);
    };
    let undo_point = move |_| {
        draft_track.update(|points| {
            points.pop();
        });
    };
    let open_track_form = move |_| {
        form_error.set(None);
        if selected_river_id.get().is_none() {
            form_error.set(Some("投稿先の川を選択してください。".to_string()));
            return;
        }
        if draft_track.get().len() < 2 {
            form_error.set(Some("経路は2点以上指定してください。".to_string()));
            return;
        }
        show_track_form.set(true);
    };
    let submit_track = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        form_error.set(None);

        let Some(river_id) = selected_river_id.get() else {
            form_error.set(Some("投稿先の川を選択してください。".to_string()));
            return;
        };
        let name = track_name.get().trim().to_owned();
        if name.is_empty() {
            form_error.set(Some("経路名を入力してください。".to_string()));
            return;
        }
        let points = draft_track.get();
        if points.len() < 2 {
            form_error.set(Some("経路は2点以上指定してください。".to_string()));
            return;
        }

        create_track.dispatch((river_id, name, track_description.get(), points));
    };

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
            <div class="map-workspace">
                <Show
                    when=move || map_ready.get()
                    fallback=|| view! { <div class="map-loading">"地図を読み込み中..."</div> }
                >
                    <RiverMap
                        map_data
                        map_layer
                        show_waypoints
                        show_tracks
                        editing
                        draft_track
                    />
                </Show>
                <div class="track-editor-panel">
                    <label class="track-editor-select">
                        <span>"投稿先"</span>
                        <Suspense fallback=|| view! { <select disabled=true><option>"読み込み中..."</option></select> }>
                            {move || {
                                let rivers = map_data
                                    .get()
                                    .and_then(|result| result.ok())
                                    .map(|data| data.rivers)
                                    .unwrap_or_default();
                                let has_rivers = !rivers.is_empty();
                                view! {
                                    <select
                                        prop:value=move || selected_river_id
                                            .get()
                                            .map(|id| id.to_string())
                                            .unwrap_or_default()
                                        disabled=!has_rivers
                                        on:change=move |ev| {
                                            let value = event_target_value(&ev);
                                            selected_river_id.set(value.parse::<i64>().ok());
                                        }
                                    >
                                        <option value="">"川を選択"</option>
                                        {rivers
                                            .into_iter()
                                            .map(|river| view! {
                                                <option value=river.river_id.to_string()>
                                                    {river.river_name}
                                                </option>
                                            })
                                            .collect_view()}
                                    </select>
                                }
                            }}
                        </Suspense>
                    </label>
                    <div class="track-editor-actions">
                        <button type="button" on:click=begin_edit disabled=move || editing.get()>
                            "編集"
                        </button>
                        <button
                            class="secondary"
                            type="button"
                            on:click=undo_point
                            disabled=move || draft_track.get().is_empty()
                        >
                            "1点戻す"
                        </button>
                        <button
                            class="secondary"
                            type="button"
                            on:click=cancel_edit
                            disabled=move || !editing.get() && draft_track.get().is_empty()
                        >
                            "クリア"
                        </button>
                        <button
                            type="button"
                            on:click=open_track_form
                            disabled=move || draft_track.get().len() < 2 || create_track.pending().get()
                        >
                            "投稿"
                        </button>
                    </div>
                    <p>
                        {move || if editing.get() {
                            "編集モード: 地図をクリックして経路点を追加"
                        } else {
                            "編集ボタンで経路作成を開始"
                        }}
                    </p>
                    <p>{move || format!("{} 点", draft_track.get().len())}</p>
                    {move || submit_result.get().map(|message| view! {
                        <p class="track-editor-success" role="status">{message}</p>
                    })}
                    {move || form_error.get().map(|message| view! {
                        <p class="track-editor-error" role="alert">{message}</p>
                    })}
                    <Show when=move || show_track_form.get()>
                        <form class="track-submit-form" on:submit=submit_track>
                            <label>
                                <span>"経路名"</span>
                                <input
                                    type="text"
                                    bind:value=track_name
                                    required=true
                                    autocomplete="off"
                                    placeholder="例: 上流からキャンプ場まで"
                                />
                            </label>
                            <label>
                                <span>"説明"</span>
                                <textarea
                                    bind:value=track_description
                                    rows="3"
                                    placeholder="任意"
                                ></textarea>
                            </label>
                            <button type="submit" disabled=move || create_track.pending().get()>
                                {move || if create_track.pending().get() {
                                    "投稿中..."
                                } else {
                                    "経路を投稿"
                                }}
                            </button>
                        </form>
                    </Show>
                </div>
            </div>
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
    editing: RwSignal<bool>,
    draft_track: RwSignal<Vec<Position>>,
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
                        events=MapEvents::new().mouse_click(move |event| {
                            if !editing.get_untracked() {
                                return;
                            }
                            let lat_lng = event.lat_lng();
                            draft_track.update(|points| {
                                points.push(Position {
                                    lat: lat_lng.lat(),
                                    lng: lat_lng.lng(),
                                });
                            });
                        })
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
                            draft_track
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
    draft_track: RwSignal<Vec<Position>>,
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
        {move || {
            let positions = draft_track.get();
            let draft_markers = positions
                .iter()
                .copied()
                .enumerate()
                .map(|(index, position)| view! {
                    <Marker position=position>
                        <Popup>
                            <strong>{format!("経路点 {}", index + 1)}</strong>
                        </Popup>
                    </Marker>
                })
                .collect_view();
            let draft_line = if positions.len() >= 2 {
                view! {
                    <Polyline
                        positions=positions.clone()
                        color="#d9332f"
                        weight=4.0
                        opacity=0.9
                    />
                }
                .into_any()
            } else {
                ().into_any()
            };

            view! {
                {draft_markers}
                {draft_line}
            }
        }}
        <div class="map-crosshair" aria-hidden="true"></div>
    }
    .into_any()
}
