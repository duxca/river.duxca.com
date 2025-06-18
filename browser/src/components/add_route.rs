//! Form component for adding new river routes.
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
//!     let onsave = Callback::from(|river_id: i64| {
//!         log::info!("Saving route for river: {}", river_id);
//!         // Handle route saving logic
//!     });
//!
//!     html! {
//!         <AddRoute
//!             selected_river={selected_river}
//!             rivers={rivers}
//!             onsave={onsave}
//!         />
//!     }
//! }
//! ```
//!
//! This component provides a form with a river selection dropdown and save button.
//! It's typically used within a Dialog component for route creation workflows.

use stylist::yew::use_style;
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub selected_river: i64,
    pub rivers: Vec<(i64, String)>,
    #[prop_or_default]
    pub onsave: Callback<i64>,
}

#[function_component(AddRoute)]
pub fn add_route(
    Props {
        selected_river,
        rivers,
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
            .unwrap();
        onsave.emit(river_id);
    });
    let style = use_style!(
        r#"
        "#,
    );
    html! {
        <fieldset class={style}>
            <legend>{"addRoute"}</legend>
            <div>
                <label>
                    {"川:"}
                    <select id="river" size="1">
                        <option value="0">{"---"}</option>
                        {
                            for rivers.iter().map(|(id, name)|{
                                if selected_river == id {
                                    html!{
                                        <option value={id.to_string()} selected=true>{name}</option>
                                    }
                                } else {
                                    html!{
                                        <option value={id.to_string()}>{name}</option>
                                    }
                                }
                            })
                        }
                    </select>
                </label>
            </div>
            <div><button onclick={onsave}>{"save"}</button></div>
        </fieldset>
    }
}
