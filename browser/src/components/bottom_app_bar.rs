//! Bottom app bar components for the river application.
//!
//! # Usage
//!
//! ```rust
//! use yew::prelude::*;
//!
//! #[function_component(MyComponent)]
//! pub fn my_component() -> Html {
//!     html! {
//!         <BottomAppBar>
//!             <BottomAppBarItem>
//!                 <button>{"Home"}</button>
//!             </BottomAppBarItem>
//!             <BottomAppBarItem>
//!                 <button>{"Settings"}</button>
//!             </BottomAppBarItem>
//!         </BottomAppBar>
//!     }
//! }
//! ```

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BottomAppBarProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<BottomAppBarItem>,
}

#[function_component(BottomAppBar)]
pub fn bottom_app_bar(props: &BottomAppBarProps) -> Html {
    html! {
        <div class="bottom-app-bar">
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct BottomAppBarItemProps {
    #[prop_or_default]
    pub children: Children,
}

/// Individual item component for the bottom app bar.
///
/// # Usage
///
/// ```rust
/// html! {
///     <BottomAppBarItem>
///         <button onclick={my_callback}>{"Action"}</button>
///     </BottomAppBarItem>
/// }
/// ```
#[function_component(BottomAppBarItem)]
pub fn bottom_app_bar_item(props: &BottomAppBarItemProps) -> Html {
    html! {
        <div class="bottom-app-bar-item">
            { for props.children.iter() }
        </div>
    }
}
