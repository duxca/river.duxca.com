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
    let side_menu_state = props.side_menu_state.clone();

    html! {
        <div class={classes!(
            "side-menu",
            if *side_menu_state == SideMenuState::Open { "open" } else { "" }
        )}>
            <div class="side-menu-header">
                <h2>{"メニュー"}</h2>
                <button class="close-menu" onclick={Callback::from({
                    let side_menu_state = side_menu_state.clone();
                    move |_| side_menu_state.set(SideMenuState::Closed)
                })}>
                    <span class="material-icons">{"close"}</span>
                </button>
            </div>
            { for props.children.iter() }
        </div>
    }
}