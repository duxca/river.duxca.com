use stylist::yew::use_style;
use yew::prelude::*;

#[derive(Debug, PartialEq, Clone, Eq)]
enum SideMenuState {
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
    let onclick = use_callback(
        side_menu_state.clone(),
        |_ev: MouseEvent, side_menu_state| {
            if **side_menu_state == SideMenuState::Closed {
                side_menu_state.set(SideMenuState::Open);
            } else {
                side_menu_state.set(SideMenuState::Closed);
            }
        },
    );
    let sidebar_style = use_style!(
        r#"
        position: fixed;
        top: 0;
        left: -300px;
        width: 280px;
        height: 100%;
        background-color: white;
        z-index: 1500;
        box-shadow: 2px 0 10px rgba(0, 0, 0, 0.2);
        transition: left 0.3s ease;
        /* overflow-y: auto; */
        &.open {
            left: 0;
        }
    "#
    );
    let hamburger_style = use_style!(
        r#"
        position: absolute;
        top: 1em;
        right: calc(-20px - 3em - 1em);
        z-index: 2000;
        background-color: white;
        border: none;
        border-radius: 50%;
        width: 3em;
        height: 3em;
        display: flex;
        align-items: center;
        justify-content: center;
        box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2);
        cursor: pointer;
        transition: all 0.3s ease;
        &:hover {
            background-color: #f5f5f5;
            transform: scale(1.05);
        }
        & .material-icons {
            font-size: 1.5em;
            color: #333;
        }
    "#
    );
    html! {
        <div class={classes!(
            sidebar_style,
            if *side_menu_state == SideMenuState::Open { "open" } else { "" }
        )}>
            <button class={hamburger_style} onclick={onclick}>
                <span class="material-icons">{"menu"}</span>
            </button>
            <nav class="side-menu-nav">
                { for props.children.iter() }
            </nav>
        </div>
    }
}
