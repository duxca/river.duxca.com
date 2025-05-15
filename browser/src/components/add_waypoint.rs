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
            <legend>{"AddWaypoint"}</legend>
            <div>
                <label>
                    {"川:"}
                    <select id="river" size="1">
                        <option value="0">{"---"}</option>
                        {
                            rivers.iter().map(|river|{
                                html!{
                                    <option value={river.river_id.to_string()}>{&river.river_name}</option>
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
            <div>{{format!("lat: {}", forcus.0)}}</div>
            <div>{{format!("lng: {}", forcus.1)}}</div>
            <div><button onclick={props.onclick_add_waypoint_cb.clone()}>{"add point"}</button></div>
        </fieldset>
    }
}