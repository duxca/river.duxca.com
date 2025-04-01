#![allow(unused_imports)]
use gloo::console;
use gloo::utils::document;
use gloo::utils::format::JsValueSerdeExt;
use leaflet::{LatLng, Map, MapOptions, TileLayer};
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlDivElement, HtmlElement, Node};
use yew::html::ImplicitClone;
use yew::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    pub longitude: f64,
    pub latitude: f64,
}

impl ImplicitClone for Point {}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub markers: Vec<(String, Point)>,
    pub forcus: Point,
    pub on_ready: Callback<Map>,
    pub on_move: Callback<Point>,
}

#[function_component(MapComponent)]
pub fn map_component(
    Props {
        markers,
        forcus,
        on_ready,
        on_move,
    }: &Props,
) -> Html {
    let node_ref = NodeRef::default();
    let map_state = use_state(|| None);
    // 初回のみ
    use_effect_with((), {
        let node_ref = node_ref.clone();
        let forcus = *forcus;
        let map_state = map_state.clone();
        let on_ready = on_ready.clone();
        let on_move = on_move.clone();
        move |()| {
            let div = node_ref.cast::<HtmlDivElement>().unwrap();
            let map = Map::new_with_element(&div, &MapOptions::default());
            map.set_view(&LatLng::new(forcus.latitude, forcus.longitude), 11.0);
            let opt = leaflet::TileLayerOptions::new();
            opt.set_attribution("<a href='https://maps.gsi.go.jp/development/ichiran.html' target='_blank'>地理院タイル</a>".to_string());
            let gsi = TileLayer::new_options(
                "https://cyberjapandata.gsi.go.jp/xyz/std/{z}/{x}/{y}.png",
                &opt,
            );
            let opt = leaflet::TileLayerOptions::new();
            opt.set_attribution(
                r#"© <a href="http://www.openstreetmap.org/copyright">OpenStreetMap</a>"#
                    .to_string(),
            );
            let osm =
                TileLayer::new_options("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", &opt);
            gsi.add_to(&map);
            let opt = leaflet::TileLayerOptions::new();
            opt.set_attribution("<a href='https://maps.gsi.go.jp/development/ichiran.html' target='_blank'>地理院タイル</a>".to_string());
            let hillshademap = TileLayer::new_options(
                "https://cyberjapandata.gsi.go.jp/xyz/hillshademap/{z}/{x}/{y}.png",
                &opt,
            );
            let opt = leaflet::TileLayerOptions::new();
            opt.set_attribution("<a href='https://maps.gsi.go.jp/development/ichiran.html' target='_blank'>地理院タイル</a>".to_string());
            let blank = TileLayer::new_options(
                "https://cyberjapandata.gsi.go.jp/xyz/blank/{z}/{x}/{y}.png",
                &opt,
            );
            let opt = leaflet::TileLayerOptions::new();
            opt.set_attribution("<a href='https://maps.gsi.go.jp/development/ichiran.html' target='_blank'>地理院タイル</a>".to_string());
            let anaglyphmap_color = TileLayer::new_options(
                "https://cyberjapandata.gsi.go.jp/xyz/anaglyphmap_color/{z}/{x}/{y}.png",
                &opt,
            );
            let opt = leaflet::TileLayerOptions::new();
            opt.set_attribution("<a href='https://maps.gsi.go.jp/development/ichiran.html' target='_blank'>地理院タイル</a>".to_string());
            let seamlessphoto = TileLayer::new_options(
                "https://cyberjapandata.gsi.go.jp/xyz/seamlessphoto/{z}/{x}/{y}.jpg",
                &opt,
            );

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
            js_sys::Reflect::set(&opt, &JsValue::from("白地図"), &JsValue::from(blank)).unwrap();
            js_sys::Reflect::set(
                &opt,
                &JsValue::from("立体地図（カラー）"),
                &JsValue::from(anaglyphmap_color),
            )
            .unwrap();
            let control = LayersControl::new(&opt);
            control.add_to(&map);

            let opt = Object::new();
            let control = ScaleControl::new(&opt.unchecked_into());
            control.add_to(&map);

            let cb = Closure::<_>::new({
                let map = map.clone();
                let on_move = on_move.clone();
                move |_| {
                    let latlng = map.get_center();
                    on_move.emit(Point {
                        latitude: latlng.lat(),
                        longitude: latlng.lng(),
                    });
                }
            });
            map.add_event_listener("move", &cb);
            cb.forget();

            map_state.set(Some(map.clone()));
            on_ready.emit(map.clone());
        }
    });

    // forcusが変化したら再描画
    use_effect_with(*forcus, {
        let map_state = map_state.clone();
        move |forcus| {
            if let Some(map) = map_state.as_ref() {
                web_sys::window()
                    .unwrap()
                    .location()
                    .set_hash(&format!("{},{}", forcus.latitude, forcus.longitude))
                    .unwrap();
                map.set_view(
                    &leaflet::LatLng::new(forcus.latitude, forcus.longitude),
                    11.0,
                );
            }
        }
    });

    use_effect_update({
        let map_state = map_state.clone();
        move || {
            let markers = markers.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let Some(map) = map_state.as_ref() else {
                    return;
                };
                // マーカーを削除
                // let markers = map.get_layers();
                // for marker in markers.iter() {
                //     map.remove_layer(marker);
                // }
                // マーカーを追加
                for (
                    name,
                    Point {
                        latitude,
                        longitude,
                    },
                ) in markers
                {
                    // let marker = leaflet::Marker::new(&LatLng::new(point.latitude, point.longitude));
                    // marker.bind_popup(name);
                    // marker.add_to(map);
                    // let opt = leaflet::IconOptions::new();
                    // opt.set_icon_url("marker-red.png".to_string());
                    // opt.set_icon_size(leaflet::Point::new(25.0, 41.0));
                    // opt.set_icon_anchor(leaflet::Point::new(12.0, 40.0));
                    // opt.set_popup_anchor(leaflet::Point::new(0.0, -40.0));
                    // let my_icon = leaflet::Icon::new(&opt);
                    // let opt = leaflet::MarkerOptions::new();
                    // opt.set_icon(my_icon);
                    // leaflet::Marker::new_with_options(
                    // let p = leaflet::Popup::new(&leaflet::PopupOptions::default(), None);
                    // p.set_content(
                    //     &JsValue::from_serde(&serde_json::json!(name)).unwrap(),
                    // );
                    // let (latitude, longitude) =
                    // serde_json::from_value::<(f64, f64)>(point.clone()).unwrap();
                    let opt = leaflet::DivIconOptions::new();
                    // opt.set_icon_size(leaflet::Point::new(25.0, 41.0));
                    opt.set_html(name.clone());
                    let icon = leaflet::DivIcon::new(&opt);
                    let opt = leaflet::MarkerOptions::new();
                    opt.set_icon(leaflet::Icon::from(icon));
                    leaflet::Marker::new_with_options(
                        &leaflet::LatLng::new(latitude, longitude),
                        &opt,
                    )
                    // .icon(icon)
                    // .bind_popup(&p)
                    // .open_popup()
                    .add_to(map);
                    return;
                }
            });
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

use js_sys::Object;
use leaflet::Control;
use yew_hooks::use_effect_update;
#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(extends = Control, js_namespace = ["L", "Scale"])]
    pub type ScaleControl;

    #[wasm_bindgen(js_namespace = ["L", "control"], js_name = "scale")]
    fn constructor_scale(options: &ScaleOptions) -> ScaleControl;

    #[wasm_bindgen(extends = Object , js_name = ScaleOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[wasm_bindgen(extends = Control)]
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
        Object::new().unchecked_into()
    }
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = Control, js_namespace = ["L", "Control"])]
    pub type LayersControl;

    #[wasm_bindgen(js_namespace = ["L", "control"], js_name = "layers")]
    fn constructor_layers(options: &Object) -> LayersControl;
}

impl LayersControl {
    #[must_use]
    pub fn new(options: &Object) -> Self {
        constructor_layers(options)
    }
}
