use yew::prelude::*;

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum SideMenuState {
    Open,
    Closed,
}

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Sidebar)]
pub fn sidebar_component(props: &SidebarProps) -> Html {
    let side_menu_state = use_state_eq(|| SideMenuState::Closed);
    html! {
        <div class={classes!(
            "side-menu",
            if *side_menu_state == SideMenuState::Open { "open" } else { "" }
        )}>
            <button class="hamburger-menu" onclick={Callback::from({
                let side_menu_state = side_menu_state.clone();
                move |_| {
                    if *side_menu_state == crate::components::sidebar::SideMenuState::Closed {
                        side_menu_state.set(crate::components::sidebar::SideMenuState::Open);
                    } else {
                        side_menu_state.set(crate::components::sidebar::SideMenuState::Closed);
                    }
                }
            })}>
                <span class="material-icons">{"menu"}</span>
            </button>
            // <div class="side-menu-header">
            //     // <h2>{"メニュー"}</h2>
            //     // <button class="close-menu" onclick={Callback::from({
            //     //     let side_menu_state = side_menu_state.clone();
            //     //     move |_| side_menu_state.set(SideMenuState::Closed)
            //     // })}>
            //     //     <span class="material-icons">{"close"}</span>
            //     // </button>
            // </div>
            <nav class="side-menu-nav">
                { for props.children.iter() }
            </nav>
        </div>
    }
}
