use stylist::yew::use_style;
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
    let style = use_style!(
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
        <fieldset class={style}>
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
