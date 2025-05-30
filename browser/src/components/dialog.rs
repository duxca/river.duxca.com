use stylist::yew::use_style;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    // pub name: String,
    // pub email: String,
    // pub profile_image_url: String,
}

#[function_component(Dialog)]
pub fn dialog(Props {}: &Props) -> Html {
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
        "#
    );
    let header_style = use_style!(
        r#"
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 1em;
            border-bottom: 1px solid #e0e0e0;
            & h3 {
                margin: 0;
                font-size: 1.2em;
                color: #333;
            }
        "#
    );
    html! {
        <div class={pane_style}>
            <div class={header_style}>
                <h3>{"地図設定"}</h3>
                <button class="close-settings">
                    <span class="material-icons">{"close"}</span>
                </button>
            </div>
            <div class="map-settings-content">
                <div class="settings-group">
                    <h4>{"表示設定"}</h4>
                    <div class="setting-item">
                        <label>
                            <input type="checkbox" checked={true} />
                            <span>{"ウェイポイントを表示"}</span>
                        </label>
                    </div>
                    <div class="setting-item">
                        <label>
                            <input type="checkbox" checked={true} />
                            <span>{"トラックを表示"}</span>
                        </label>
                    </div>
                </div>
                <div class="settings-group">
                    <h4>{"地図スタイル"}</h4>
                    <div class="setting-item">
                        <select>
                            <option value="gsi" selected={true}>{"地理院タイル"}</option>
                            <option value="osm">{"OpenStreetMap"}</option>
                            <option value="hillshade">{"陰影起伏図"}</option>
                            <option value="seamlessphoto">{"航空写真"}</option>
                        </select>
                    </div>
                </div>
            </div>
        </div>
    }
}
