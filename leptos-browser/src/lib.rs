use leptos::prelude::*;
use leptos_leaflet::prelude::*;

const FUJI_RIVER: Position = Position {
    lat: 35.362_222,
    lng: 138.731_389,
};

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="app-shell">
            <header class="topbar">
                <div>
                    <h1>"river.duxca.com"</h1>
                    <p>"Leptos + Leaflet preview"</p>
                </div>
            </header>
            <MapContainer
                class="map"
                center=FUJI_RIVER
                zoom=12.0
                scroll_wheel_zoom=true
                set_view=true
            >
                <TileLayer
                    url="https://tile.openstreetmap.org/{z}/{x}/{y}.png"
                    attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"
                />
                <Marker position=FUJI_RIVER>
                    <Popup>
                        <strong>"富士川周辺"</strong>
                        <br/>
                        "最低限の地図表示確認用です。"
                    </Popup>
                </Marker>
            </MapContainer>
        </div>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
