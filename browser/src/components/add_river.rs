use yew::prelude::*;

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum SideMenuState {
    Open,
    Closed,
}

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub side_menu_state: UseStateHandle<SideMenuState>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(SidebarComponent)]
pub fn sidebar_component(props: &SidebarProps) -> Html {
    html! {
    <fieldset>
            <legend>{"addRiver"}</legend>
            <div>
                <label>
                    {"川:"}
                    <input type="text" id={"river_name"} />
                </label>
            </div>
            <div>{{format!("lat: {}", forcus.0)}}</div>
            <div>{{format!("lng: {}", forcus.1)}}</div>
            <div><button onclick={props.onclick_add_river_cb.clone()}>{"add river"}</button></div>
        </fieldset>
    }
}
