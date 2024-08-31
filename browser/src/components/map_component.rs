use gloo::utils::document;
use leaflet::{LatLng, Map, MapOptions, TileLayer};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, Node};
use yew::{html::ImplicitClone, prelude::*};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point(pub f64, pub f64);

#[derive(PartialEq, Clone, Debug)]
pub struct City {
    pub name: String,
    pub lat: Point,
}

impl ImplicitClone for City {}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub city: City,
}

fn add_tile_layer(map: &Map) {
    TileLayer::new("https://cyberjapandata.gsi.go.jp/xyz/std/{z}/{x}/{y}.png").add_to(map);
    // TileLayer::new("https://cyberjapandata.gsi.go.jp/xyz/seamlessphoto/{z}/{x}/{y}.png").add_to(map);
    // TileLayer::new("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png").add_to(map);
}

#[function_component(MapComponent)]
pub fn map_component(Props { city }: &Props) -> Html {
    let container: Element = document().create_element("div").unwrap();
    let container: HtmlElement = container.dyn_into().unwrap();
    container.set_id("map");
    let map = Map::new_with_element(&container, &MapOptions::default());
    let lat = use_state(|| city.lat);
    use_effect(move || {
        map.set_view(&LatLng::new(lat.0, lat.1), 11.0);
        add_tile_layer(&map);
        || {}
    });
    let node: &Node = &container.clone().into();
    let elm = Html::VRef(node.clone());
    html! {
        {elm}
    }
}
