use async_channel::{Receiver, Sender};
use bevy::tasks::TaskPool;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{MessageEvent, WebSocket};

use crate::game::ChessPieceColorEnum;

pub mod resources;

pub(crate) struct GameWsControlMsg {}

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
    _rx_control: Receiver<GameWsControlMsg>,
    tx_updates: Sender<GameWsUpdateMsg>,
) {
    let Ok(ws) = WebSocket::new("ws://localhost:3000/game") else {
        return;
    };
    let ws_closure = ws.clone();
    let open_cb: Closure<dyn FnMut()> = Closure::new(move || {
        ws_closure.send_with_str(&jwt).expect("Failed sending jwt");
    });
    ws.set_onopen(Some(open_cb.as_ref().unchecked_ref()));
    open_cb.forget();
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
}
