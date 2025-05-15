use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub selected_river: i64,
    pub rivers: Vec<(i64, String)>,
    pub onchange: Callback<i64>,
}

#[function_component(SelectRiver)]
pub fn select_river(
    Props {
        selected_river,
        rivers,
        onchange,
    }: &Props,
) -> Html {
    let onchange = use_callback(onchange.clone(), move |_e: Event, onchange| {
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
        onchange.emit(river_id);
    });
    html! {
        <fieldset class="control-bottom-right-1st">
            <legend>{"selectRiver"}</legend>
            <div>
                <label>
                    {"川:"}
                    <select id="river" size="1" onchange={onchange}>
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
        </fieldset>
    }
}
