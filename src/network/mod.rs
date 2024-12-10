use async_channel::{Receiver, Sender};
use bevy::{
    prelude::*,
    tasks::{IoTaskPool, TaskPool},
};
use resources::WsUpdate;
use serde::{Deserialize, Serialize};
use state::ConnectionState;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Event, MessageEvent, WebSocket};

use crate::game::{turn::to::ChessMove, ChessPieceColorEnum, GameState};

pub mod resources;
pub mod state;
pub mod systems;

#[derive(Serialize)]
pub(crate) enum GameWsControlMsg {
    TurnEnd(ChessMove),
    Ack,
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
    PawnMove(ChessMove),
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
    let ws_on_open = Closure::<dyn FnMut()>::new({
        let ws_closure = ws.clone();
        move || {
            debug!("Handling the web socket! opened!!");
            ws_closure.send_with_str(&jwt).expect("Failed sending jwt");
        }
    });
    ws.set_onopen(Some(ws_on_open.as_ref().unchecked_ref()));
    ws_on_open.forget();
    // ws.onMessage
    let ws_on_message = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        let Some(msg) = e
            .data()
            .as_string()
            .and_then(|str| serde_json::from_str(&str).ok())
        else {
            return;
        };
        let tx_updates_current = tx_updates.clone();
        TaskPool::new()
            .spawn(async move {
                debug!("WebSocket: {:?}", &msg);
                tx_updates_current.send(msg).await.unwrap();
            })
            .detach();
    });
    ws.set_onmessage(Some(ws_on_message.as_ref().unchecked_ref()));
    ws_on_message.forget();
    // ws.onError
    let ws_on_error = Closure::<dyn FnMut(_)>::new(move |e: Event| {
        error!("Error: {:?}", e);
    });
    ws.set_onerror(Some(ws_on_error.as_ref().unchecked_ref()));
    ws_on_error.forget();
    // On rx_control signal
    IoTaskPool::get()
        .spawn_local(send_ws_message(ws, rx_control))
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
        app.init_state::<ConnectionState>()
            .init_resource::<WsUpdate>()
            .add_systems(
                FixedUpdate,
                ws_update.run_if(in_state(ConnectionState::WebSocket)),
            )
            .add_systems(FixedUpdate, ws_get_color.after(ws_update))
            .add_systems(OnEnter(GameState::Playing), on_game_start_confirm);
    }
}
