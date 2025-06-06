use stylist::yew::use_style;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onclose: Callback<MouseEvent>,
}

#[function_component(Dialog)]
pub fn dialog(
    Props {
        title,
        children,
        onclose,
    }: &Props,
) -> Html {
    let pane_style = use_style!(
        r#"
        position: fixed;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        width: 80%;
        max-width: 400px;
        max-height: 80vh;
        background-color: white;
        z-index: 1500;
        box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
        border-radius: 8px;
        overflow-y: auto;
        transition: opacity 0.3s ease, visibility 0.3s ease;
        & header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 1em;
            border-bottom: 1px solid #e0e0e0;
        }
        & header h3 {
            margin: 0;
            font-size: 1.2em;
            color: #333;
        }
        "#
    );
    html! {
        <div class={pane_style}>
            <header>
                <h3>{title}</h3>
                <button class="close-settings" onclick={onclose.clone()}>
                    <span class="material-icons">{"close"}</span>
                </button>
            </header>
            <div class="map-settings-content">
                { for children.iter() }
            </div>
        </div>
    }
}
