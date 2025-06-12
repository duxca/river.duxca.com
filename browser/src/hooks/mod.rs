// #![allow(unused_imports)]

// 基本的な構造体の定義に必要なuseステートメント
// use gloo::utils::document;
// use gloo::utils::format::JsValueSerdeExt;
// use leaflet::{LatLng, MapOptions, TileLayer};
// use std::fmt::Debug;
// use wasm_bindgen::closure::Closure;
// use wasm_bindgen::JsCast;
// use wasm_bindgen::JsValue;
// use web_sys::{Element, HtmlDivElement, HtmlElement, Node};

//         wasm_bindgen_futures::spawn_local(async move {
//             let model::api::create_river_track::Response { river_track_id } =
//                 crate::api::call::<model::api::create_river_track::Response>(
//                     model::api::create_river_track::Request {
//                         river_id,
//                         track_name: "test".to_string(),
//                         description: "test".to_string(),
//                         track: track.clone(),
//                     },
//                 )
//                 .await
//                 .unwrap();
//             let mut trks = (*river_tracks).clone();
//             trks.push(model::river::RiverTrack {
//                 user_id: 0,
//                 river_track_id,
//                 river_id,
//                 track_name: "test".to_string(),
//                 description: "test".to_string(),
//                 track: serde_json::to_value(track).unwrap(),
//                 created_at: 0,
//                 updated_at: 0,
//             });
//             river_tracks.set(trks);
//             edit_mode.set(EditMode::Home {});
//         });

//     <fieldset>
//         <legend>{"Home"}</legend>
//         <div>
//             <label>
//                 {"川:"}
//                 <select id="river" size="1">
//                     <option value="0">{"---"}</option>
//                     {
//                         rivers.iter().map(|river|{
//                             html!{
//                                 <option value={river.river_id.to_string()}>{&river.river_name}</option>
//                             }
//                         }).collect::<Html>()
//                     }
//                 </select>
//             </label>
//         </div>
//         <div><button onclick={props.onclick_go_to_river.clone()}>{"go"}</button></div>
//     </fieldset>

// use gloo::utils::format::JsValueSerdeExt;
// use crate::components::sidebar_component::Sidebar;
// use gloo::console;

//         wasm_bindgen_futures::spawn_local(async move {
//             let model::api::create_river::Response { river_id } =
//                 crate::api::call::<model::api::create_river::Response>(
//                     model::api::create_river::Request {
//                         name: title.clone(),
//                         latitude: lat,
//                         longitude: lng,
//                     },
//                 )
//                 .await
//                 .unwrap();
//             let mut rvs = (*rivers).clone();
//             rvs.push(model::river::River {
//                 user_id: 0,
//                 river_id,
//                 river_name: title,
//                 waypoint: serde_json::to_value((lat, lng)).unwrap(),
//                 description: "".to_string(),
//                 created_at: 0,
//             });
//             rivers.set(rvs);
//             edit_mode.set(EditMode::Home {});
//         });

// use_future_with(page_state,  //{
//     // let rivers = rivers.clone();
//     // let focus = focus.clone();
//      |page_state| async {
//         let page_state = page_state.clone();
//         wasm_bindgen_futures::spawn_local(async move {
//             if **page_state == PageState::Loading {
//                 return;
//             }
//             if **page_state == PageState::LoggedOut {
//                 return;
//             }
//             let url_hash = web_sys::window().unwrap().location().hash().unwrap();
//             if let Some(hash) = url_hash.split('#').nth(1) {
//                 let mut latlng = hash.split(',');
//                 let latitude = latlng.next().unwrap().parse::<f64>().unwrap();
//                 let longitude = latlng.next().unwrap().parse::<f64>().unwrap();
//                 focus.set((latitude, longitude));
//             }
//             // ログインしたら川の一覧を取得
//             let res = crate::api::call::<model::api::list_rivers::Response>(
//                 model::api::list_rivers::Request {},
//             )
//             .await
//             .unwrap();
//             // 川の一覧を描画
//             rivers.set(res.rivers);
//         });
//     }
// //}
// );


// let onclick_go_to_river = Callback::from({
//     let edit_mode = edit_mode.clone();
//     let rivers = rivers.clone();
//     let focus = focus.clone();
//     move |_ev: MouseEvent| {
//         let EditMode::Home = &*edit_mode else {
//             return;
//         };
//         let val = gloo::utils::document()
//             .get_element_by_id("river")
//             .unwrap()
//             .dyn_into::<web_sys::HtmlSelectElement>()
//             .unwrap()
//             .value();
//         let river_id = val.parse::<i64>().unwrap();
//         for river in &*rivers {
//             if river.river_id == river_id {
//                 let (lat, lng) =
//                     serde_json::from_value::<(f64, f64)>(river.waypoint.clone()).unwrap();
//                 web_sys::window()
//                     .unwrap()
//                     .location()
//                     .set_hash(&format!("{},{}", lat, lng))
//                     .unwrap();
//                 focus.set((lat, lng));
//                 return;
//             }
//         }
//     }
// });

//         wasm_bindgen_futures::spawn_local(async move {
//             let model::api::create_river_waypoint::Response { river_waypoint_id } =
//                 crate::api::call::<model::api::create_river_waypoint::Response>(
//                     model::api::create_river_waypoint::Request {
//                         river_id,
//                         name: title.clone(),
//                         latitude: lat,
//                         longitude: lng,
//                     },
//                 )
//                 .await
//                 .unwrap();
//             let mut wpts = (*river_waypoints).clone();
//             wpts.push(model::river::RiverWaypoint {
//                 user_id: 0,
//                 river_waypoint_id,
//                 river_id,
//                 waypoint_name: title,
//                 waypoint: serde_json::to_value((lat, lng)).unwrap(),
//                 description: "".to_string(),
//                 created_at: 0,
//                 updated_at: 0,
//             });
//             river_waypoints.set(wpts);
//         });



//         //             if let EditMode::Home{} = *edit_mode {
//         //     <fieldset>
//         //         <legend>{"Home"}</legend>
//         //         <div>
//         //             <label>
//         //                 {"川:"}
//         //                 <select id="river" size="1">
//         //                     <option value="0">{"---"}</option>
//         //                     {
//         //                         rivers.iter().map(|river|{
//         //                             html!{
//         //                                 <option value={river.river_id.to_string()}>{&river.river_name}</option>
//         //                             }
//         //                         }).collect::<Html>()
//         //                     }
//         //                 </select>
//         //             </label>
//         //         </div>
//         //         <div><button onclick={props.onclick_go_to_river.clone()}>{"go"}</button></div>
//         //     </fieldset>
//         // } else if let EditMode::AddRoute(ref o) = *edit_mode {

//         // } else if let EditMode::AddWaypoint{} = *edit_mode {

//         // } else if let EditMode::AddRiver = *edit_mode {
//         //     <AddRiver />
//         // }

//             <nav id="bottom-nav-bar">
//                 <button onclick={Callback::from({
//                     let edit_mode = edit_mode.clone();
//                     move |_| edit_mode.set(EditMode::Home)
//                 })} class={if matches!(*edit_mode, EditMode::Home{}) { "active" } else { "" }}>
//                     <span class="material-icons">{"home"}</span>
//                     <span class="label">{"ホーム"}</span>
//                 </button>
//                 <button onclick={Callback::from({
//                     let edit_mode = edit_mode.clone();
//                     move |_| edit_mode.set(EditMode::AddRoute(AddRouteMode::default()))
//                 })} class={if matches!(*edit_mode, EditMode::AddRoute(_)) { "active" } else { "" }}>
//                     <span class="material-icons">{"route"}</span>
//                     <span class="label">{"ルート"}</span>
//                 </button>
//                 <button onclick={Callback::from({
//                     let edit_mode = edit_mode.clone();
//                     move |_| edit_mode.set(EditMode::AddWaypoint)
//                 })} class={if matches!(*edit_mode, EditMode::AddWaypoint) { "active" } else { "" }}>
//                     <span class="material-icons">{"place"}</span>
//                     <span class="label">{"ポイント"}</span>
//                 </button>
//                 <button onclick={Callback::from({
//                     let edit_mode = edit_mode.clone();
//                     move |_| edit_mode.set(EditMode::AddRiver)
//                 })} class={if matches!(*edit_mode, EditMode::AddRiver) { "active" } else { "" }}>
//                     <span class="material-icons">{"water"}</span>
//                     <span class="label">{"川"}</span>
//                 </button>
//             </nav>

/*


// 設定画面の表示状態
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum SettingsVisibility {
    Visible,
    Hidden,
}

    // 設定画面の表示状態
    let settings_visibility = use_state(|| SettingsVisibility::Hidden);


    // 設定ボタンのクリックハンドラ
    let toggle_settings = {
        let settings_visibility = settings_visibility.clone();
        Callback::from(move |_: MouseEvent| {
            if *settings_visibility == SettingsVisibility::Hidden {
                settings_visibility.set(SettingsVisibility::Visible);
            } else {
                settings_visibility.set(SettingsVisibility::Hidden);
            }
        })
    };

            // 設定ボタン
            <button class="map-settings-button" onclick={toggle_settings.clone()}>
                <span class="material-icons">{"settings"}</span>
            </button>

            // 設定画面

             */

// #map {
//   height: 100%;
//   // height: calc(100% - 5rem);
//   width: 100%;
// }

// @media (max-width: 480px) {
//   // #map {
//     // height: calc(100% - 4rem);
//   // }
// }

// #bottom-nav-bar {
//   position: fixed;
//   bottom: 0;
//   left: 0;
//   width: 100%;
//   height: 5rem;
//   background-color: rgba(255, 255, 255, 0.95);
//   z-index: 1000;
//   display: flex;
//   justify-content: space-around;
//   box-shadow: 0 -2px 10px rgba(0, 0, 0, 0.2);
//   border-top: 1px solid #e0e0e0;
// }

// #bottom-nav-bar button {
//   flex: 1;
//   display: flex;
//   flex-direction: column;
//   align-items: center;
//   justify-content: center;
//   padding: 0;
//   background: none;
//   border: none;
//   color: #666;
//   font-size: 0.8em;
//   transition: all 0.2s ease;
//   cursor: pointer;
//   outline: none;
// }

// #bottom-nav-bar button:hover {
//   color: #2196F3;
//   background-color: rgba(33, 150, 243, 0.05);
// }

// #bottom-nav-bar button.active {
//   color: #2196F3;
//   background-color: rgba(33, 150, 243, 0.1);
// }

// #bottom-nav-bar .material-icons {
//   font-size: 1.8em;
//   margin-bottom: 0.2em;
// }

// #bottom-nav-bar .label {
//   font-size: 0.9em;
//   font-weight: 500;
// }

// @media (max-width: 480px) {
//   #bottom-nav-bar {
//     height: 4em;
//   }

//   #bottom-nav-bar .material-icons {
//     font-size: 1.5em;
//   }

//   #bottom-nav-bar .label {
//     font-size: 0.8em;
//   }
// }

// .control-top-left-1st {
//   position: absolute;
//   top: 5em;
//   left: 1em;
//   z-index: 1000;
// }
// .control-top-left-2nd {
//   position: absolute;
//   top: 7em;
//   left: 1em;
//   z-index: 1000;
// }
// .control-top-left-3rd {
//   position: absolute;
//   top: 9em;
//   left: 1em;
//   z-index: 1000;
// }
// .control-top-left-4th {
//   position: absolute;
//   top: 11em;
//   left: 1em;
//   z-index: 1000;
// }
// .control-top-left-5th {
//   position: absolute;
//   top: 13em;
//   left: 1em;
//   z-index: 1000;
// }

// .control-bottom-left-1st {
//   position: absolute;
//   bottom: 5em;
//   left: 1em;
//   z-index: 1000;
//   // padding: 10px;
//   // border-radius: 5px;
//   // box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
// }

// .control-bottom-right-1st {
//   position: absolute;
//   bottom: 5em;
//   right: 1em;
//   z-index: 1000;
//   // padding: 10px;
//   // border-radius: 5px;
//   // box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
// }

// // 設定ボタン
// .map-settings-button {
//   position: fixed;
//   top: 1em;
//   right: 1em;
//   z-index: 1000;
//   background-color: white;
//   border: none;
//   border-radius: 50%;
//   width: 3em;
//   height: 3em;
//   display: flex;
//   align-items: center;
//   justify-content: center;
//   box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2);
//   cursor: pointer;
//   transition: all 0.3s ease;
// }

// .map-settings-button:hover {
//   background-color: #f5f5f5;
//   transform: scale(1.05);
// }

// .map-settings-button .material-icons {
//   font-size: 1.5em;
//   color: #333;
// }

// // 設定パネル

// .map-settings-panel.hidden {
//   opacity: 0;
//   visibility: hidden;
// }

// .map-settings-panel.visible {
//   opacity: 1;
//   visibility: visible;
// }

// .close-settings {
//   background: none;
//   border: none;
//   cursor: pointer;
//   color: #666;
//   padding: 0.5em;
//   display: flex;
//   align-items: center;
//   justify-content: center;
// }

// .close-settings:hover {
//   color: #333;
// }

// .map-settings-content {
//   padding: 1em;
// }

// .settings-group {
//   margin-bottom: 1.5em;
// }

// .settings-group h4 {
//   margin: 0 0 0.5em 0;
//   font-size: 1em;
//   color: #555;
// }

// .setting-item {
//   margin-bottom: 0.8em;
// }

// .setting-item label {
//   display: flex;
//   align-items: center;
//   cursor: pointer;
// }

// .setting-item input[type="checkbox"] {
//   margin-right: 0.5em;
// }

// .setting-item select {
//   width: 100%;
//   padding: 0.5em;
//   border: 1px solid #ddd;
//   border-radius: 4px;
//   background-color: #f9f9f9;
// }

// @media (max-width: 480px) {
//   .map-settings-panel {
//     width: 90%;
//   }
// }
