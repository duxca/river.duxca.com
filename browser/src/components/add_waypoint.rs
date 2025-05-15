use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub selected_river: i64,
    pub rivers: Vec<(i64, String)>,
    // latlng
    pub focus: (f64, f64),
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
            .unwrap();
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
    html! {
        <fieldset class="control-bottom-right-1st">
            <legend>{"addWaypoint"}</legend>
            <div>
                <label>
                    {"川:"}
                    <select id="river" size="1">
                        <option value="0">{"---"}</option>
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
                    {"地点:"}
                    <input type="text" id="waypoint_name" />
                </label>
            </div>
            <div>{{format!("lat: {}", lat)}}</div>
            <div>{{format!("lng: {}", lng)}}</div>
            <div><button onclick={onsave}>{"add point"}</button></div>
        </fieldset>
    }
}
