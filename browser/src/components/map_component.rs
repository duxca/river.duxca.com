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
    pub initial_forcus: Point,
    pub map_ready: Callback<Map>,
}

#[function_component(MapComponent)]
pub fn map_component(
    Props {
        initial_forcus,
        map_ready,
    }: &Props,
) -> Html {
    let node_ref = NodeRef::default();
    let map_state = use_state(|| None);
    // 初回のみ
    use_effect_with((), {
        let node_ref = node_ref.clone();
        let map_state = map_state.clone();
        let initial_forcus = *initial_forcus;
        let map_ready = map_ready.clone();
        move |()| {
            let div = node_ref.cast::<HtmlDivElement>().unwrap();
            let map = Map::new_with_element(&div, &MapOptions::default());
            map.set_view(
                &LatLng::new(initial_forcus.latitude, initial_forcus.longitude),
                11.0,
            );
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

            // let cb = Closure::<_>::new({
            //     let map = map.clone();
            //     move |_| {
            //         let latlng = map.get_center();
            //         // centor.set(Point {
            //         //     latitude: latlng.lat(),
            //         //     longitude: latlng.lng(),
            //         // });
            //     }
            // });
            // map.add_event_listener("move", &cb);
            // cb.forget();

            map_ready.emit(map.clone());
            map_state.set(Some(map));
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
