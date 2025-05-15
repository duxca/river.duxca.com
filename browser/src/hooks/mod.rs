
    // use_future_with(page_state,  //{
    //     // let rivers = rivers.clone();
    //     // let forcus = forcus.clone();
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
    //                 forcus.set((latitude, longitude));
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

    // use_effect_with(rivers.clone(), {
    //     let river_waypoints = river_waypoints.clone();
    //     let river_tracks = river_tracks.clone();
    //     move |rivers| {
    //         let rivers = rivers.clone();
    //         // wasm_bindgen_futures::spawn_local(async move {
    //         //     let mut wpts = vec![];
    //         //     let mut tracks = vec![];
    //         //     for river in &*rivers {
    //         //         let mut res = crate::api::call::<model::api::get_river::Response>(
    //         //             model::api::get_river::Request {
    //         //                 river_id: river.river_id,
    //         //             },
    //         //         )
    //         //         .await
    //         //         .unwrap();
    //         //         wpts.append(&mut res.waypoints);
    //         //         tracks.append(&mut res.tracks);
    //         //     }
    //         //     // 都度再描画
    //         //     river_waypoints.set(wpts.clone());
    //         //     river_tracks.set(tracks);
    //         // });
    //     }
    // });

    

    // let onclick_go_to_river = Callback::from({
    //     let edit_mode = edit_mode.clone();
    //     let rivers = rivers.clone();
    //     let forcus = forcus.clone();
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
    //                 forcus.set((lat, lng));
    //                 return;
    //             }
    //         }
    //     }
    // });
    // let onclick_add_route_cb = Callback::from({
    //     let edit_mode = edit_mode.clone();
    //     let forcus = forcus.clone();
    //     move |_ev: MouseEvent| {
    //         let EditMode::AddRoute(st) = &*edit_mode else {
    //             return;
    //         };
    //         let (lat, lng) = *forcus;
    //         let mut track = st.track.clone();
    //         track.push((lat, lng));
    //         edit_mode.set(EditMode::AddRoute(AddRouteMode {
    //             track,
    //             // TODO
    //             distance: st.distance,
    //         }));
    //     }
    // });
    // let onclick_save_route_cb = Callback::from({
    //     let edit_mode = edit_mode.clone();
    //     let forcus = forcus.clone();
    //     let river_tracks = river_tracks.clone();
    //     move |_ev: MouseEvent| {
    //         let EditMode::AddRoute(st) = &*edit_mode else {
    //             return;
    //         };
    //         let (lat, lng) = *forcus;
    //         let mut track = st.track.clone();
    //         track.push((lat, lng));
    //         let river_id = gloo::utils::document()
    //             .get_element_by_id("river")
    //             .unwrap()
    //             .dyn_into::<web_sys::HtmlSelectElement>()
    //             .unwrap()
    //             .value()
    //             .parse::<i64>()
    //             .unwrap();
    //         let edit_mode = edit_mode.clone();
    //         let river_tracks = river_tracks.clone();
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
    //     }
    // });
    // let onclick_add_waypoint_cb = Callback::from({
    //     let edit_mode = edit_mode.clone();
    //     let forcus = forcus.clone();
    //     let river_waypoints = river_waypoints.clone();
    //     move |_ev: MouseEvent| {
    //         let EditMode::AddWaypoint = &*edit_mode else {
    //             return;
    //         };
    //         let val = gloo::utils::document()
    //             .get_element_by_id("river")
    //             .unwrap()
    //             .dyn_into::<web_sys::HtmlSelectElement>()
    //             .unwrap()
    //             .value();
    //         let river_id = val.parse::<i64>().unwrap();
    //         let title = web_sys::window()
    //             .unwrap()
    //             .document()
    //             .unwrap()
    //             .get_element_by_id("waypoint_name")
    //             .unwrap()
    //             .dyn_ref::<web_sys::HtmlInputElement>()
    //             .unwrap()
    //             .value();
    //         let (lat, lng) = *forcus;
    //         let river_waypoints = river_waypoints.clone();
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
    //     }
    // });
    // let onclick_add_river_cb = Callback::from({
    //     let edit_mode = edit_mode.clone();
    //     let forcus = forcus.clone();
    //     let rivers = rivers.clone();
    //     move |_ev: MouseEvent| {
    //         let EditMode::AddRiver = &*edit_mode else {
    //             return;
    //         };
    //         let title = gloo::utils::document()
    //             .get_element_by_id("river_name")
    //             .unwrap()
    //             .dyn_ref::<web_sys::HtmlInputElement>()
    //             .unwrap()
    //             .value();
    //         let (lat, lng) = *forcus;
    //         let edit_mode = edit_mode.clone();
    //         let rivers = rivers.clone();
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
    //     }
    // });

    // let mut waypoints = vec![];
    // let mut tracks = vec![];
    // for river in &*rivers {
    //     let (lat, long) = serde_json::from_value::<(f64, f64)>(river.waypoint.clone()).unwrap();
    //     waypoints.push((river.river_id, river.river_name.clone(), (lat, long)));
    // }
    // for wpt in &*river_waypoints {
    //     let (lat, long) = serde_json::from_value::<(f64, f64)>(wpt.waypoint.clone()).unwrap();
    //     waypoints.push((
    //         wpt.river_waypoint_id,
    //         wpt.waypoint_name.clone(),
    //         (lat, long),
    //     ));
    // }
    // for trk in &*river_tracks {
    //     let track = serde_json::from_value::<Vec<(f64, f64)>>(trk.track.clone()).unwrap();
    //     tracks.push((trk.river_track_id, track));
    // }
    // if let EditMode::AddRoute(ref o) = *edit_mode {
    //     tracks.push((0, o.track.clone()));
    // }

    
    //             // ハンバーガーメニューアイコン
    //             <button class="hamburger-menu" onclick={Callback::from({
    //                 let side_menu_state = side_menu_state.clone();
    //                 move |_| {
    //                     if *side_menu_state == components::sidebar_component::SideMenuState::Closed {
    //                         side_menu_state.set(components::sidebar_component::SideMenuState::Open);
    //                     } else {
    //                         side_menu_state.set(components::sidebar_component::SideMenuState::Closed);
    //                     }
    //                 }
    //             })}>
    //                 <span class="material-icons">{"menu"}</span>
    //             </button>

    //             <SidebarComponent
    //                 side_menu_state={side_menu_state.clone()}
    //                 // edit_mode={edit_mode.clone()}
    //                 // rivers={rivers.clone()}
    //                 // onclick_go_to_river={onclick_go_to_river.clone()}
    //                 // onclick_add_route_cb={onclick_add_route_cb.clone()}
    //                 // onclick_save_route_cb={onclick_save_route_cb.clone()}
    //                 // onclick_add_waypoint_cb={onclick_add_waypoint_cb.clone()}
    //                 // onclick_add_river_cb={onclick_add_river_cb.clone()}
    //                 // forcus={*forcus} 
    //                 >
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
    //             </SidebarComponent>

    
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
    //             <form method="post" action="/logout">
    //                 <input class="control-top-left-2th" type="submit" value="Logout" />
    //             </form>