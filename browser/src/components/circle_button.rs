use stylist::yew::use_style;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub enum CircleButtonIcon {
    Plus,
    Polyline,
    Flag,
    Delete,
    Settings,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or(CircleButtonIcon::Plus)]
    pub icon: CircleButtonIcon,
    #[prop_or("#2196f3".to_string())]
    pub color: String,
    #[prop_or(16)]
    pub bottom: i32,
}

#[function_component(CircleButton)]
pub fn circle_button(
    Props {
        onclick,
        icon,
        color,
        bottom,
    }: &Props,
) -> Html {
    let circle_style = use_style!(
        r#"
        position: fixed;
        bottom: ${bottom}px;
        right: 16px;
        width: 56px;
        height: 56px;
        border-radius: 50%;
        background-color: ${color};
        color: white;
        border: none;
        cursor: pointer;
        box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2);
        display: flex;
        align-items: center;
        justify-content: center;
        transition: transform 0.2s, background-color 0.2s;
        z-index: 1000;

        &:hover {
            background-color: ${hover_color};
            transform: scale(1.05);
        }

        &:active {
            background-color: ${active_color};
            transform: scale(0.95);
        }
    "#,
        bottom = bottom * (16 + 56),
        color = color,
        hover_color = darken_color(color, 10),
        active_color = darken_color(color, 20)
    );

    html! {
        <button
            class={circle_style}
            onclick={onclick.clone()}
        >
            <span class="material-icons" style="font-size: 24px; line-height: 1;">
                {match icon {
                    CircleButtonIcon::Plus => "add",
                    CircleButtonIcon::Polyline => "timeline",
                    CircleButtonIcon::Flag => "flag",
                    CircleButtonIcon::Delete => "delete",
                    CircleButtonIcon::Settings => "settings",
                }}
            </span>
        </button>
    }
}

fn darken_color(color: &str, percent: i32) -> String {
    let color = color.trim_start_matches('#');
    if let Ok(rgb) = i64::from_str_radix(color, 16) {
        let r = ((rgb >> 16) & 0xFF) as i32;
        let g = ((rgb >> 8) & 0xFF) as i32;
        let b = (rgb & 0xFF) as i32;

        let darken = |v: i32| -> i32 {
            let v = v - (v * percent / 100);
            v.clamp(0, 255)
        };

        let r = darken(r);
        let g = darken(g);
        let b = darken(b);

        format!("#{:02x}{:02x}{:02x}", r, g, b)
    } else {
        color.to_string()
    }
}
