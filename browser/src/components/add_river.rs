//! Form component for adding new rivers at specific coordinates.
//!
//! # Usage
//!
//! ```rust
//! use yew::prelude::*;
//!
//! #[function_component(MyComponent)]
//! pub fn my_component() -> Html {
//!     let focus = (35.3622222, 138.7313889); // Coordinates (lat, lng)
//!     let onsave = Callback::from(|river_name: String| {
//!         log::info!("Adding river: {}", river_name);
//!         // Handle river creation logic
//!     });
//!
//!     html! {
//!         <AddRiver
//!             focus={focus}
//!             onsave={onsave}
//!         />
//!     }
//! }
//! ```
//!
//! This component provides a form for creating new rivers with:
//! - Text input for river name
//! - Display of current latitude/longitude coordinates
//! - Save button to trigger river creation

use stylist::yew::use_style;
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    // latlng
    pub focus: (f64, f64),
    #[prop_or_default]
    pub onsave: Callback<String>,
}

#[function_component(AddRiver)]
pub fn add_river(
    Props {
        focus: (lat, lng),
        onsave,
    }: &Props,
) -> Html {
    let onsave = use_callback(onsave.clone(), move |_e: MouseEvent, onsave| {
        let river_name = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("river_name")
            .unwrap()
            .dyn_ref::<web_sys::HtmlInputElement>()
            .unwrap()
            .value();
        onsave.emit(river_name);
    });
    let _style = use_style!(
        r#"
        position: absolute;
        bottom: 5em;
        right: 1em;
        z-index: 1000;
        border-radius: 5px;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
        background-color: white;
        "#,
    );
    html! {
        <fieldset>
            <legend>{"addRiver"}</legend>
            <div>
                <label>
                    {"川:"}
                    <input type="text" id={"river_name"} />
                </label>
            </div>
            <div>{{format!("lat: {}", lat)}}</div>
            <div>{{format!("lng: {}", lng)}}</div>
            <div><button onclick={onsave}>{"add river"}</button></div>
        </fieldset>
    }
}
