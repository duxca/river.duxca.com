use stylist::yew::use_style;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum MapLayer {
    Gsi,
    Osm,
    Hillshade,
    Blank,
    // AnaglyphmapColor,
    Seamlessphoto,
}

// (lat, lng)
#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub layer: MapLayer,
    // 初期位置
    pub focus: (f64, f64),
    #[prop_or_default]
    pub waypoints: std::collections::HashMap<i64, (String, (f64, f64))>,
    #[prop_or_default]
    pub tracks: std::collections::HashMap<i64, Vec<(f64, f64)>>,
    #[prop_or_default]
    pub on_move: Option<Callback<(f64, f64)>>,
}

#[function_component(Map)]
pub fn map_component(
    Props {
        layer,
        waypoints,
        tracks,
        focus,
        on_move,
    }: &Props,
) -> Html {
    // 初回のみ
    let node_ref = NodeRef::default();
    let map_state = use_state(|| None);
    use_effect_with((), {
        let node_ref = node_ref.clone();
        let focus = *focus;
        let layer = *layer;
        let map_state = map_state.clone();
        let on_move = on_move.clone();
        move |()| {
            let (lat, lng) = focus;
            let map = {
                let div = node_ref.cast::<web_sys::HtmlDivElement>().unwrap();
                let opt = leaflet::MapOptions::default();
                opt.set_zoom_control(false);
                leaflet::Map::new_with_element(&div, &opt)
            };

            // 初期位置
            map.set_view(&leaflet::LatLng::new(lat, lng), 12.0);

            let zoom_control = {
                let opt = js_sys::Object::new();
                js_sys::Reflect::set(&opt, &JsValue::from("position"), &JsValue::from("topright"))
                    .unwrap();
                ZoomControl::new(&opt.unchecked_into())
            };
            zoom_control.add_to(&map);

            let scale_control = {
                let opt = js_sys::Object::new();
                js_sys::Reflect::set(
                    &opt,
                    &JsValue::from("position"),
                    &JsValue::from("bottomleft"),
                )
                .unwrap();
                ScaleControl::new(&opt.unchecked_into())
            };
            scale_control.add_to(&map);

            let layer_control = {
                let gsi = {
                    let opt = leaflet::TileLayerOptions::new();
                    opt.set_attribution("<a href='https://maps.gsi.go.jp/development/ichiran.html' target='_blank'>地理院タイル</a>".to_string());
                    leaflet::TileLayer::new_options(
                        "https://cyberjapandata.gsi.go.jp/xyz/std/{z}/{x}/{y}.png",
                        &opt,
                    )
                };
                let osm = {
                    let opt = leaflet::TileLayerOptions::new();
                    opt.set_attribution(
                        r#"© <a href="http://www.openstreetmap.org/copyright">OpenStreetMap</a>"#
                            .to_string(),
                    );
                    leaflet::TileLayer::new_options(
                        "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",
                        &opt,
                    )
                };
                let hillshademap = {
                    let opt = leaflet::TileLayerOptions::new();
                    opt.set_attribution("<a href='https://maps.gsi.go.jp/development/ichiran.html' target='_blank'>地理院タイル</a>".to_string());
                    leaflet::TileLayer::new_options(
                        "https://cyberjapandata.gsi.go.jp/xyz/hillshademap/{z}/{x}/{y}.png",
                        &opt,
                    )
                };
                let blank = {
                    let opt = leaflet::TileLayerOptions::new();
                    opt.set_attribution("<a href='https://maps.gsi.go.jp/development/ichiran.html' target='_blank'>地理院タイル</a>".to_string());
                    leaflet::TileLayer::new_options(
                        "https://cyberjapandata.gsi.go.jp/xyz/blank/{z}/{x}/{y}.png",
                        &opt,
                    )
                };
                // let anaglyphmap_color = {
                //     let opt = leaflet::TileLayerOptions::new();
                //     opt.set_attribution("<a href='https://maps.gsi.go.jp/development/ichiran.html' target='_blank'>地理院タイル</a>".to_string());
                //     leaflet::TileLayer::new_options(
                //         "https://cyberjapandata.gsi.go.jp/xyz/anaglyphmap_color/{z}/{x}/{y}.png",
                //         &opt,
                //     )
                // };
                let seamlessphoto = {
                    let opt = leaflet::TileLayerOptions::new();
                    opt.set_attribution("<a href='https://maps.gsi.go.jp/development/ichiran.html' target='_blank'>地理院タイル</a>".to_string());
                    leaflet::TileLayer::new_options(
                        "https://cyberjapandata.gsi.go.jp/xyz/seamlessphoto/{z}/{x}/{y}.jpg",
                        &opt,
                    )
                };
                match layer {
                    MapLayer::Gsi => gsi.add_to(&map),
                    MapLayer::Osm => osm.add_to(&map),
                    MapLayer::Hillshade => hillshademap.add_to(&map),
                    MapLayer::Blank => blank.add_to(&map),
                    // MapLayer::AnaglyphmapColor => anaglyphmap_color.add_to(&map),
                    MapLayer::Seamlessphoto => seamlessphoto.add_to(&map),
                };
                // 生 object でしか設定できない
                let layers = js_sys::Object::new();
                js_sys::Reflect::set(
                    &layers,
                    &JsValue::from("OpenStreetMap"),
                    &JsValue::from(osm),
                )
                .unwrap();
                js_sys::Reflect::set(&layers, &JsValue::from("地理院タイル"), &JsValue::from(gsi))
                    .unwrap();
                js_sys::Reflect::set(
                    &layers,
                    &JsValue::from("航空写真"),
                    &JsValue::from(seamlessphoto),
                )
                .unwrap();
                js_sys::Reflect::set(
                    &layers,
                    &JsValue::from("陰影起伏図"),
                    &JsValue::from(hillshademap),
                )
                .unwrap();
                js_sys::Reflect::set(&layers, &JsValue::from("白地図"), &JsValue::from(blank))
                    .unwrap();
                // js_sys::Reflect::set(
                //     &layers,
                //     &JsValue::from("立体地図（カラー）"),
                //     &JsValue::from(anaglyphmap_color),
                // )
                // .unwrap();
                let overlays = js_sys::Object::new();
                let opt = js_sys::Object::new();
                js_sys::Reflect::set(
                    &opt,
                    &JsValue::from("position"),
                    &JsValue::from("bottomleft"),
                )
                .unwrap();
                LayersControl::new(&layers, &overlays, &opt)
            };
            layer_control.add_to(&map);

            let cb = Closure::<_>::new({
                let map = map.clone();
                let on_move = on_move.clone();
                move |_| {
                    let latlng = map.get_center();
                    gloo::console::log!(
                        "map move: lat={}, lng={}",
                        latlng.lat(),
                        latlng.lng()
                    );
                    if let Some(on_move) = on_move.as_ref() {
                        on_move.emit((latlng.lat(), latlng.lng()));
                    }
                }
            });
            map.add_event_listener("move", &cb);
            cb.forget(); // Closure を保持しておく

            map_state.set(Some(map.clone()));
            move || {
                let Some(map) = map_state.as_ref() else {
                    return;
                };
                map.remove();
            }
        }
    });

    // on_move が変化したら設定変更のみ
    use_effect_with(on_move.clone(), {
        move |_on_move| {
            log::warn!("on_move changing is not implemented yet");
        }
    });

    // 再描画なし
    use_effect_with(*focus, {
        let map_state = map_state.clone();
        move |(lat, lng)| {
            let Some(map) = map_state.as_ref() else {
                return;
            };
            let latlng = map.get_center();
            if latlng.lat() == *lat && latlng.lng() == *lng {
                return;
            }
            map.set_view(&leaflet::LatLng::new(*lat, *lng), map.get_zoom());
            // destructor なし
        }
    });

    // 描画中の marker の一覧
    let markers_state = use_mut_ref(std::collections::HashMap::<i64, leaflet::Marker>::new);
    // waypoints が変化したら再描画
    use_effect_with(waypoints.clone(), {
        let map_state = map_state.clone();
        move |waypoints| {
            let Some(map) = map_state.as_ref() else {
                return;
            };
            let mut markers = markers_state.borrow_mut();
            // waypoints に id がないものは削除
            for (id, marker) in markers.iter() {
                if !waypoints.contains_key(id) {
                    map.remove_layer(marker);
                }
            }
            // waypoints に id があるが markers にないものは追加
            for (id, (name, (lat, lng))) in waypoints {
                if markers.contains_key(id) {
                    // すでにあるのでスキップ
                    continue;
                }
                let icon = {
                    let opt = leaflet::DivIconOptions::new();
                    opt.set_html(name.clone());
                    opt.set_icon_size(leaflet::Point::new(100.0, 20.0));
                    opt.set_icon_anchor(leaflet::Point::new(0.0, 20.0));
                    leaflet::DivIcon::new(&opt)
                };
                let marker = {
                    let opt = leaflet::MarkerOptions::new();
                    opt.set_icon(leaflet::Icon::from(icon));
                    leaflet::Marker::new_with_options(&leaflet::LatLng::new(*lat, *lng), &opt)
                };
                marker.add_to(map);
                markers.insert(*id, marker.clone());
            }
        }
    });

    // 描画中の polyline の一覧
    let polylines_state = use_mut_ref(std::collections::HashMap::<i64, leaflet::Polyline>::new);
    // tracks が変化したら再描画
    use_effect_with(tracks.clone(), {
        let map_state = map_state.clone();
        move |tracks| {
            gloo::console::log!("tracks changed: {:?}", tracks);
            let Some(map) = map_state.as_ref() else {
                return;
            };
            let mut polylines = polylines_state.borrow_mut();
            // tracks に id がないものは削除
            for (id, polyline) in polylines.iter() {
                if !tracks.contains_key(id) {
                    map.remove_layer(polyline);
                }
            }
            // tracks に id があるが polylines にないものは追加
            for (id, track) in tracks {
                if polylines.contains_key(id) {
                    // すでにあるのでスキップ
                    continue;
                }
                let track = track
                    .iter()
                    .cloned()
                    .map(|(lat, lng)| wasm_bindgen::JsValue::from(leaflet::LatLng::new(lat, lng)))
                    .collect::<js_sys::Array>();
                let polyline = {
                    let opt = leaflet::PolylineOptions::new();
                    opt.set_color("red".to_string());
                    opt.set_weight(5.0);
                    opt.set_opacity(0.5);
                    leaflet::Polyline::new_with_options(&track, &opt)
                };
                polyline.add_to(map);
                polylines.insert(*id, polyline.clone());
            }
        }
    });

    // この VDOM に変化なければ再描画されない
    let map_style = use_style!(
        r#"
        position: absolute;
        padding: 0px;
        margin: 0px;
        top: 0px;
        left: 0px;
        height: 100%;
        width: 100%;
    "#
    );
    let crosshair_style = use_style!(
        r#"
        z-index: 1000;
        position: absolute;
        top: 50%;
        left: 50%;
        width: 20px;
        height: 20px;
        margin-left: -10px; /* 十字の中心を合わせるためのマージン */
        margin-top: -10px;
        pointer-events: none;
        &::before,
        &::after {
            content: '';
            position: absolute;
            background-color: red;
        }
        &::before {
            width: 2px;
            height: 20px;
            left: 50%;
            transform: translateX(-50%);
        }
        &::after {
            width: 20px;
            height: 2px;
            top: 50%;
            transform: translateY(-50%);
        }
    "#
    );
    html! {
        <>
        <div id="map" class={map_style} ref={node_ref}></div>
        <div class={crosshair_style}></div>
        </>
    }
}

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(extends = leaflet::Control, js_namespace = ["L", "Scale"])]
    pub type ScaleControl;

    #[wasm_bindgen(js_namespace = ["L", "control"], js_name = "scale")]
    fn constructor_scale(options: &ScaleOptions) -> ScaleControl;

    #[wasm_bindgen(extends = js_sys::Object , js_name = ScaleOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[wasm_bindgen(extends = leaflet::Control)]
    pub type ScaleOptions;
}

impl ScaleControl {
    #[must_use]
    pub fn new(options: &ScaleOptions) -> Self {
        constructor_scale(options)
    }
}

impl Default for ScaleOptions {
    fn default() -> Self {
        js_sys::Object::new().unchecked_into()
    }
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = leaflet::Control, js_namespace = ["L", "Control"])]
    pub type LayersControl;

    #[wasm_bindgen(js_namespace = ["L", "control"], js_name = "layers")]
    fn constructor_layers(
        layers: &js_sys::Object,
        overlays: &js_sys::Object,
        options: &js_sys::Object,
    ) -> LayersControl;
}

impl LayersControl {
    #[must_use]
    pub fn new(
        layers: &js_sys::Object,
        overlays: &js_sys::Object,
        options: &js_sys::Object,
    ) -> Self {
        constructor_layers(layers, overlays, options)
    }
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = leaflet::Control, js_namespace = ["L", "Control"])]
    pub type ZoomControl;

    #[wasm_bindgen(js_namespace = ["L", "control"], js_name = "zoom")]
    fn constructor_zoom(options: &js_sys::Object) -> ZoomControl;
}

impl ZoomControl {
    #[must_use]
    pub fn new(options: &js_sys::Object) -> Self {
        constructor_zoom(options)
    }
}
