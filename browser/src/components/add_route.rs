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
    pub onclick_add_node: Callback<()>,
    #[prop_or_default]
    pub onsave: Callback<i64>,
}

#[function_component(AddRoute)]
pub fn add_route(
    Props {
        selected_river,
        rivers,
        focus: (lat, lng),
        onclick_add_node,
        onsave,
    }: &Props,
) -> Html {
    let onclick_add_node = use_callback(
        onclick_add_node.clone(),
        move |_e: MouseEvent, onclick_add_node| {
            onclick_add_node.emit(());
        },
    );
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
        position: absolute;
        bottom: 5em;
        right: 1em;
        z-index: 1000;
        padding: 10px;
        border-radius: 5px;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
        background-color: white;
        "#,
    );
    html! {
        <fieldset>
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
            <div>{{format!("lat: {}", lat)}}</div>
            <div>{{format!("lng: {}", lng)}}</div>
            <div><button onclick={onclick_add_node}>{"add node"}</button></div>
            <div><button onclick={onsave}>{"save"}</button></div>
        </fieldset>
    }
}
