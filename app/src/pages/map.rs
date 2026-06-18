use leptos::prelude::*;
use leptos_leaflet::prelude::*;
use shared_api::{create_river_track, list_rivers};

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
    GsiHillshade,
    GsiBlank,
    GsiSeamlessphoto,
}

impl BaseLayer {
    fn from_value(value: &str) -> Self {
        match value {
            "gsi-standard" => Self::GsiStandard,
            "gsi-red-relief" => Self::GsiRedRelief,
            "gsi-hillshade" => Self::GsiHillshade,
            "gsi-blank" => Self::GsiBlank,
            "gsi-seamlessphoto" => Self::GsiSeamlessphoto,
            _ => Self::Osm,
        }
    }

    const fn value(self) -> &'static str {
        match self {
            Self::Osm => "osm",
            Self::GsiStandard => "gsi-standard",
            Self::GsiRedRelief => "gsi-red-relief",
            Self::GsiHillshade => "gsi-hillshade",
            Self::GsiBlank => "gsi-blank",
            Self::GsiSeamlessphoto => "gsi-seamlessphoto",
        }
    }

    const fn url(self) -> &'static str {
        match self {
            Self::Osm => "https://tile.openstreetmap.org/{z}/{x}/{y}.png",
            Self::GsiStandard => "https://cyberjapandata.gsi.go.jp/xyz/std/{z}/{x}/{y}.png",
            Self::GsiRedRelief => "https://cyberjapandata.gsi.go.jp/xyz/sekishoku/{z}/{x}/{y}.png",
            Self::GsiHillshade => {
                "https://cyberjapandata.gsi.go.jp/xyz/hillshademap/{z}/{x}/{y}.png"
            }
            Self::GsiBlank => "https://cyberjapandata.gsi.go.jp/xyz/blank/{z}/{x}/{y}.png",
            Self::GsiSeamlessphoto => {
                "https://cyberjapandata.gsi.go.jp/xyz/seamlessphoto/{z}/{x}/{y}.jpg"
            }
        }
    }

    const fn attribution(self) -> &'static str {
        match self {
            Self::Osm => {
                "&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"
            }
            Self::GsiStandard
            | Self::GsiRedRelief
            | Self::GsiHillshade
            | Self::GsiBlank
            | Self::GsiSeamlessphoto => GSI_ATTRIBUTION,
        }
    }
}

fn river_position(river: &model::river::River) -> Option<Position> {
    let (lat, lng): (f64, f64) = serde_json::from_value(river.waypoint.clone()).ok()?;
    Some(Position { lat, lng })
}

fn track_points(positions: &[Position]) -> Vec<(f64, f64)> {
    positions
        .iter()
        .map(|position| (position.lat, position.lng))
        .collect()
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
            create_river_track(river_id, track_name, description, track_points(&positions)).await
        }
    });

    Effect::new(move |_| {
        map_ready.set(true);
    });

    Effect::new(move |_| {
        if selected_river_id.get().is_some() {
            return;
        }
        if let Some(Ok(response)) = rivers.get()
            && let Some(river) = response.rivers.first()
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
                            <option value="gsi-hillshade">"陰影起伏図"</option>
                            <option value="gsi-blank">"白地図"</option>
                            <option value="gsi-seamlessphoto">"航空写真"</option>
                        </select>
                    </label>
                    <label class="map-layer-select">
                        <span>"投稿先"</span>
                        <Suspense fallback=|| view! { <select disabled=true><option>"読み込み中..."</option></select> }>
                            {move || {
                                let rivers = rivers
                                    .get()
                                    .and_then(|result| result.ok())
                                    .map(|response| response.rivers)
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
                    <RiverStatus rivers/>
                </div>
            </header>
            <div class="map-workspace">
                <Show
                    when=move || map_ready.get()
                    fallback=|| view! { <div class="map-loading">"地図を読み込み中..."</div> }
                >
                    <RiverMap
                        rivers
                        base_layer
                        editing
                        draft_track
                    />
                </Show>
                <div class="track-editor-panel">
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
    editing: RwSignal<bool>,
    draft_track: RwSignal<Vec<Position>>,
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
                            let layer = base_layer.get();
                            view! {
                                <TileLayer
                                    url=layer.url()
                                    attribution=layer.attribution()
                                />
                            }
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
                        {markers}
                    </MapContainer>
                }
            }}
        </Suspense>
    }
}
