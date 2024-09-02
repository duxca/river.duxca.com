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
}

#[function_component(MapComponent)]
pub fn map_component(Props { forcus }: &Props) -> Html {
    let node_ref = NodeRef::default();
    let map_state = use_state(|| None);
    use_effect_with((), {
        let node_ref = node_ref.clone();
        let map_state = map_state.clone();
        let forcus = forcus.clone();
        move |()| {
            let div = node_ref.cast::<HtmlDivElement>().unwrap();
            let map = Map::new_with_element(&div, &MapOptions::default());
            console::log!("fuji");
            map.set_view(&LatLng::new(forcus.latitude, forcus.longitude), 11.0); // Fuji
            let opt = &leaflet::TileLayerOptions::new();
            opt.set_attribution("<a href='https://maps.gsi.go.jp/development/ichiran.html' target='_blank'>地理院タイル</a>".to_string());
            TileLayer::new_options(
                "https://cyberjapandata.gsi.go.jp/xyz/std/{z}/{x}/{y}.png",
                // "https://cyberjapandata.gsi.go.jp/xyz/seamlessphoto/{z}/{x}/{y}.png",
                // "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",
                opt,
            )
            .add_to(&map);
            map_state.set(Some(map));
        }
    });

    use_effect({
        let forcus = forcus.clone();
        let map_state = map_state.clone();
        move || {
            console::log!(forcus.latitude, forcus.longitude);
            if let Some(map) = map_state.as_ref() {
                map.set_view(&LatLng::new(forcus.latitude, forcus.longitude), 11.0);
            }
        }
    });
    console::log!("aaa", forcus.latitude, forcus.longitude);
    html! {
        <div id="map" ref={node_ref}>
        </div>
    }
}
