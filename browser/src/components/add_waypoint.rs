//! Form component for adding waypoints to rivers at specific coordinates.
//!
//! # Usage
//!
//! ```rust
//! use yew::prelude::*;
//!
//! #[function_component(MyComponent)]
//! pub fn my_component() -> Html {
//!     let rivers = vec![
//!         (1, "River A".to_string()),
//!         (2, "River B".to_string()),
//!     ];
//!     let selected_river = 1;
//!     let focus = (35.3622222, 138.7313889); // Current map center (lat, lng)
//!     let onsave = Callback::from(|(river_id, waypoint_name): (i64, String)| {
//!         log::info!("Adding waypoint '{}' to river {}", waypoint_name, river_id);
//!         // Handle waypoint creation logic
//!     });
//!
//!     html! {
//!         <AddWaypoint
//!             selected_river={selected_river}
//!             rivers={rivers}
//!             focus={focus}
//!             onsave={onsave}
//!         />
//!     }
//! }
//! ```
//!
//! This component provides a form for creating waypoints with:
//! - River selection dropdown
//! - Text input for waypoint name
//! - Display of current latitude/longitude coordinates
//! - Save button to trigger waypoint creation

use stylist::yew::use_style;
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub selected_river: i64,
    pub rivers: Vec<(i64, String)>,
    // latlng
    pub focus: (f64, f64),
    #[prop_or_default]
    pub onsave: Callback<(i64, String)>,
}

#[function_component(AddWaypoint)]
pub fn add_waypoint(
    Props {
        selected_river,
        rivers,
        focus: (lat, lng),
        onsave,
    }: &Props,
) -> Html {
    let onsave = use_callback(onsave.clone(), move |_e: MouseEvent, onsave| {
        let river_id = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("river")
            .unwrap()
            .dyn_into::<web_sys::HtmlSelectElement>()
            .unwrap()
            .value()
            .parse::<i64>()
            .unwrap_or(0); // Default to 0 for river-independent markers
        let waypoint_name = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("waypoint_name")
            .unwrap()
            .dyn_ref::<web_sys::HtmlInputElement>()
            .unwrap()
            .value();
        onsave.emit((river_id, waypoint_name));
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
            <legend>{"マーカーを追加"}</legend>
            <div>
                <label>
                    {"川:"}
                    <select id="river" size="1">
                        <option value="0">{"川を選択してください"}</option>
                        {
                            rivers.iter().map(|(id, name)|{
                                if selected_river == id {
                                    html!{
                                        <option value={id.to_string()} selected=true>{name}</option>
                                    }
                                } else {
                                    html!{
                                        <option value={id.to_string()}>{name}</option>
                                    }
                                }
                            }).collect::<Html>()
                        }
                    </select>
                </label>
            </div>
            <div>
                <label>
                    {"地点名:"}
                    <input type="text" id="waypoint_name" placeholder="名所、景勝地、危険箇所など" />
                </label>
            </div>
            <div>{{format!("緯度: {:.6}", lat)}}</div>
            <div>{{format!("経度: {:.6}", lng)}}</div>
            <div><button onclick={onsave}>{"マーカーを配置"}</button></div>
        </fieldset>
    }
}
