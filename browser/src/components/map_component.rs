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
    pub forcus: Point,
    pub points: Vec<Point>,
}

#[function_component(MapComponent)]
pub fn map_component(Props { forcus, points }: &Props) -> Html {
    let node_ref = NodeRef::default();
    let map_state = use_state(|| None);
    // 初回のみ
    use_effect_with((), {
        let node_ref = node_ref.clone();
        let map_state = map_state.clone();
        let forcus = *forcus;
        move |()| {
            let div = node_ref.cast::<HtmlDivElement>().unwrap();
            let map = Map::new_with_element(&div, &MapOptions::default());
            map.set_view(&LatLng::new(forcus.latitude, forcus.longitude), 11.0); // Fuji
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

            let control = leaflet::LayersControl::new(&opt);
            control.add_to(&map);

            map_state.set(Some(map));
        }
    });
    // pointsが変化したら再描画
    use_effect_with(points.clone(), {
        let map_state = map_state.clone();
        move |points| {
            if let Some(map) = map_state.as_ref() {
                for point in points {
                    // let opt = leaflet::IconOptions::new();
                    // opt.set_icon_url("marker-red.png".to_string());
                    // opt.set_icon_size(leaflet::Point::new(25.0, 41.0));
                    // opt.set_icon_anchor(leaflet::Point::new(12.0, 40.0));
                    // opt.set_popup_anchor(leaflet::Point::new(0.0, -40.0));
                    // let my_icon = leaflet::Icon::new(&opt);
                    // let opt = leaflet::MarkerOptions::new();
                    // opt.set_icon(my_icon);
                    // leaflet::Marker::new_with_options(
                    leaflet::Marker::new(
                        &LatLng::new(point.latitude, point.longitude),
                        // &opt,
                    )
                    .add_to(map);
                }
            }
        }
    });
    // forcusが変化したら再描画
    use_effect_with(*forcus, {
        let map_state = map_state.clone();
        move |forcus| {
            if let Some(map) = map_state.as_ref() {
                map.set_view(&LatLng::new(forcus.latitude, forcus.longitude), 11.0);
            }
        }
    });
    // この VDOM に変化なければ再描画されない
    html! {
        <div id="map" ref={node_ref}>
        </div>
    }
}
