use super::{GameClientMsg, ServerMsg};
use async_channel::{Receiver, Sender};
use bevy::prelude::*;

#[derive(Resource)]
pub(crate) struct WebsocketChannels {
    pub tx_control: Sender<GameClientMsg>,
    pub rx_updates: Receiver<ServerMsg>,
}

#[derive(Resource, Default)]
pub(crate) struct WsUpdate(pub Option<ServerMsg>);
