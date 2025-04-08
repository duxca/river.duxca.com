#![allow(unused_imports)]

// 基本的な構造体の定義に必要なuseステートメント
use gloo::utils::document;
use gloo::utils::format::JsValueSerdeExt;
use leaflet::{LatLng, Map, MapOptions, TileLayer};
use std::fmt::Debug;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{Element, HtmlDivElement, HtmlElement, Node};
use yew::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum MapLayer {
    Gsi,
    Osm,
    Hillshade,
    // Blank,
    // AnaglyphmapColor,
    Seamlessphoto,
}
// (lat, lng)
#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub layer: MapLayer,
    // 初期位置
    pub forcus: (f64, f64),
    pub waypoints: Vec<(i64, String, (f64, f64))>,
    pub tracks: Vec<(i64, Vec<(f64, f64)>)>,
    #[prop_or_default]
    pub on_move: Option<Callback<(f64, f64)>>,
}

#[function_component(MapComponent)]
pub fn map_component(
    Props {
        layer,
        waypoints,
        tracks,
        forcus,
        on_move,
    }: &Props,
) -> Html {
    let node_ref = NodeRef::default();
    let map_state = use_state(|| None);
    // 描画中のウェイポイント一覧
    let markers_state = use_state(Vec::<(i64, leaflet::Marker)>::new);
    // 描画中のトラック一覧
    let polylines_state = use_state(Vec::<(i64, leaflet::Polyline)>::new);

    // 初回のみ
    use_effect_with((), {
        let node_ref = node_ref.clone();
        let (lat, lng) = *forcus;
        let layer = *layer;
        let map_state = map_state.clone();
        let on_move = on_move.clone();
        move |()| {
            let div = node_ref.cast::<HtmlDivElement>().unwrap();
            let map = Map::new_with_element(&div, &MapOptions::default());

            // 初期位置
            map.set_view(&LatLng::new(lat, lng), 11.0);
            let gsi = {
                let opt = leaflet::TileLayerOptions::new();
                opt.set_attribution("<a href='https://maps.gsi.go.jp/development/ichiran.html' target='_blank'>地理院タイル</a>".to_string());
                TileLayer::new_options(
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
                TileLayer::new_options("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", &opt)
            };
            let hillshademap = {
                let opt = leaflet::TileLayerOptions::new();
                opt.set_attribution("<a href='https://maps.gsi.go.jp/development/ichiran.html' target='_blank'>地理院タイル</a>".to_string());
                TileLayer::new_options(
                    "https://cyberjapandata.gsi.go.jp/xyz/hillshademap/{z}/{x}/{y}.png",
                    &opt,
                )
            };
            // let blank = {
            //     let opt = leaflet::TileLayerOptions::new();
            //     opt.set_attribution("<a href='https://maps.gsi.go.jp/development/ichiran.html' target='_blank'>地理院タイル</a>".to_string());
            //     TileLayer::new_options(
            //         "https://cyberjapandata.gsi.go.jp/xyz/blank/{z}/{x}/{y}.png",
            //         &opt,
            //     )
            // };
            // let anaglyphmap_color = {
            //     let opt = leaflet::TileLayerOptions::new();
            //     opt.set_attribution("<a href='https://maps.gsi.go.jp/development/ichiran.html' target='_blank'>地理院タイル</a>".to_string());
            //     TileLayer::new_options(
            //         "https://cyberjapandata.gsi.go.jp/xyz/anaglyphmap_color/{z}/{x}/{y}.png",
            //         &opt,
            //     )
            // };
            let seamlessphoto = {
                let opt = leaflet::TileLayerOptions::new();
                opt.set_attribution("<a href='https://maps.gsi.go.jp/development/ichiran.html' target='_blank'>地理院タイル</a>".to_string());
                TileLayer::new_options(
                    "https://cyberjapandata.gsi.go.jp/xyz/seamlessphoto/{z}/{x}/{y}.jpg",
                    &opt,
                )
            };
            match layer {
                MapLayer::Gsi => gsi.add_to(&map),
                MapLayer::Osm => osm.add_to(&map),
                MapLayer::Hillshade => hillshademap.add_to(&map),
                // MapLayer::Blank => blank.add_to(&map),
                // MapLayer::AnaglyphmapColor => anaglyphmap_color.add_to(&map),
                MapLayer::Seamlessphoto => seamlessphoto.add_to(&map),
            };

            let control = {
                // 生 object でしか設定できない
                let opt = js_sys::Object::new();
                js_sys::Reflect::set(&opt, &JsValue::from("OpenStreetMap"), &JsValue::from(osm))
                    .unwrap();
                js_sys::Reflect::set(&opt, &JsValue::from("地理院タイル"), &JsValue::from(gsi))
                    .unwrap();
                js_sys::Reflect::set(
                    &opt,
                    &JsValue::from("航空写真"),
                    &JsValue::from(seamlessphoto),
                )
                .unwrap();
                js_sys::Reflect::set(
                    &opt,
                    &JsValue::from("陰影起伏図"),
                    &JsValue::from(hillshademap),
                )
                .unwrap();
                // js_sys::Reflect::set(&opt, &JsValue::from("白地図"), &JsValue::from(blank))
                //     .unwrap();
                // js_sys::Reflect::set(
                //     &opt,
                //     &JsValue::from("立体地図（カラー）"),
                //     &JsValue::from(anaglyphmap_color),
                // )
                // .unwrap();
                LayersControl::new(&opt)
            };
            control.add_to(&map);

            let control = {
                let opt = js_sys::Object::new();
                ScaleControl::new(&opt.unchecked_into())
            };
            control.add_to(&map);

            // TODO: use_effect_with(on_move, ) を使う
            let cb = Closure::<_>::new({
                let map = map.clone();
                let on_move = on_move.clone();
                move |_| {
                    let latlng = map.get_center();
                    if let Some(on_move) = on_move.as_ref() {
                        on_move.emit((latlng.lat(), latlng.lng()));
                    }
                }
            });
            map.add_event_listener("move", &cb);
            cb.forget();

            map_state.set(Some(map.clone()));
        }
    });

    // forcusが変化したら再描画
    use_effect_with(*forcus, {
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
        }
    });

    // markers が変化したら再描画
    use_effect_with(waypoints.clone(), {
        let map_state = map_state.clone();
        move |waypoints| {
            let Some(map) = map_state.as_ref() else {
                return;
            };
            // TODO: 必要なものだけ消す
            for (_id, marker) in &*markers_state {
                map.remove_layer(marker);
            }
            // TODO: 必要分だけ追加
            let mut markers = vec![];
            for (_id, name, (lat, lng)) in waypoints {
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
                markers.push((*_id, marker.clone()));
            }
            markers_state.set(markers);
        }
    });

    // tracks が変化したら再描画
    use_effect_with(tracks.clone(), {
        let map_state = map_state.clone();
        move |tracks| {
            let Some(map) = map_state.as_ref() else {
                return;
            };
            // TODO: 必要な分だけ削除
            for (_id, polyline) in &*polylines_state {
                map.remove_layer(polyline);
            }
            // TODO: 必要な分だけ追加
            let mut polylines = vec![];
            for (id, track) in tracks {
                let track = track
                    .iter()
                    .cloned()
                    .map(|(lat, lng)| wasm_bindgen::JsValue::from(LatLng::new(lat, lng)))
                    .collect::<js_sys::Array>();
                let opt = leaflet::PolylineOptions::new();
                let polyline = leaflet::Polyline::new_with_options(&track, &opt);
                polyline.add_to(map);
                polylines.push((*id, polyline.clone()));
            }
            polylines_state.set(polylines);
        }
    });

    // この VDOM に変化なければ再描画されない
    html! {
        <>
            <div id="map" ref={node_ref}>
            </div>
            <div class="crosshair">
            </div>
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
    fn constructor_layers(options: &js_sys::Object) -> LayersControl;
}

impl LayersControl {
    #[must_use]
    pub fn new(options: &js_sys::Object) -> Self {
        constructor_layers(options)
    }
}
