use async_channel::{Receiver, Sender};
use bevy::{
    app::{FixedUpdate, Plugin},
    prelude::{in_state, AppExtStates, IntoSystemConfigs},
    tasks::{IoTaskPool, TaskPool},
};
use serde::{Deserialize, Serialize};
use state::ConnectionState;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{MessageEvent, WebSocket};

use crate::game::{turn::to::ChessMove, ChessPieceColorEnum};

pub mod resources;
pub mod state;
pub mod systems;

#[derive(Serialize)]
pub(crate) enum GameWsControlMsg {
    TurnEnd(ChessMove),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum MatchmakingResponse {
    Searching,
    Success { color: ChessPieceColorEnum },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum GameMessage {
    NewTurn(bool),
    Error(String),
    Notification(String),
    GameEnd(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum GameWsUpdateMsg {
    Matchmaking(MatchmakingResponse),
    Game(GameMessage),
}

pub(crate) async fn server_ws_handler(
    jwt: String,
    rx_control: Receiver<GameWsControlMsg>,
    tx_updates: Sender<GameWsUpdateMsg>,
) {
    let Ok(ws) = WebSocket::new("ws://localhost:3000/game") else {
        return;
    };
    // ws.onOpen
    let ws_closure = ws.clone();
    let open_cb: Closure<dyn FnMut()> = Closure::new(move || {
        ws_closure.send_with_str(&jwt).expect("Failed sending jwt");
    });
    ws.set_onopen(Some(open_cb.as_ref().unchecked_ref()));
    open_cb.forget();
    // ws.onMessage
    let cb = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        let Some(json) = e.data().as_string() else {
            return;
        };
        let Ok(msg): Result<GameWsUpdateMsg, _> = serde_json::from_str(&json) else {
            return;
        };
        let tx_updates_current = tx_updates.clone();
        TaskPool::new()
            .spawn(async move { tx_updates_current.send(msg).await.unwrap() })
            .detach();
    });
    ws.set_onmessage(Some(cb.as_ref().unchecked_ref()));
    cb.forget();
    // On rx_control signal
    IoTaskPool::get()
        .spawn_local(send_ws_message(ws.clone(), rx_control))
        .detach();
}

pub(crate) async fn send_ws_message(ws: WebSocket, rx: Receiver<GameWsControlMsg>) {
    while let Ok(message) = rx.recv().await {
        ws.send_with_str(&serde_json::to_string(&message).unwrap())
            .unwrap();
    }
}

pub(crate) struct Network;

impl Plugin for Network {
    fn build(&self, app: &mut bevy::prelude::App) {
        use systems::*;
        app.init_state::<ConnectionState>().add_systems(
            FixedUpdate,
            ws_get_color.run_if(in_state(ConnectionState::WebSocket)),
        );
    }
}
