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

/// BottomAppBar is a reusable component for a bottom app bar.
#[function_component(BottomAppBarItem)]
pub fn bottom_app_bar_item(props: &BottomAppBarItemProps) -> Html {
    html! {
        <div class="bottom-app-bar-item">
            { for props.children.iter() }
        </div>
    }
}
