use bevy::{prelude::*, tasks::IoTaskPool};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{MessageEvent, WebSocket};

use crate::{
    game::{resources::PlayerColorResource, ChessPieceColorEnum, GameState},
    main_menu::components::MainMenuStartButton,
    JwtToken,
};

pub fn on_click_game_start(
    jwt_token: Res<JwtToken>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut player_color: ResMut<PlayerColorResource>,
    query: Query<&Interaction, (Changed<Interaction>, With<MainMenuStartButton>)>,
) {
    for interaction in query.iter() {
        if *interaction != Interaction::Pressed {
            return;
        }
        let cloned_token = jwt_token.jwt.clone();
        IoTaskPool::get()
            .spawn(async move {
                let Ok(ws) = WebSocket::new("ws://localhost:3000/game") else {
                    return;
                };
                let ws_closure = ws.clone();
                let open_cb: Closure<dyn FnMut()> = Closure::new(move || {
                    ws_closure
                        .send_with_str(&cloned_token)
                        .expect("Failed sending jwt");
                });
                ws.set_onopen(Some(open_cb.as_ref().unchecked_ref()));
                open_cb.forget();
                let cb = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
                    let json = e.data().as_string();
                    debug!("{:?}", json);
                });
                ws.set_onmessage(Some(cb.as_ref().unchecked_ref()));
                cb.forget();
            })
            .detach();
        player_color.0 = ChessPieceColorEnum::White;
        next_game_state.set(GameState::Playing);
    }
}
