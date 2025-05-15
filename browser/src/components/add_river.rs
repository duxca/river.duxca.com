use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    // latlng
    pub focus: (f64, f64),
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
    html! {
        <fieldset class="control-bottom-right-1st">
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
