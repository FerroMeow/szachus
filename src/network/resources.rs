use super::{GameWsControlMsg, GameWsUpdateMsg};
use async_channel::{Receiver, Sender};
use bevy::prelude::*;

#[derive(Resource)]
pub(crate) struct WebsocketChannels {
    pub tx_control: Sender<GameWsControlMsg>,
    pub rx_updates: Receiver<GameWsUpdateMsg>,
}

#[derive(Resource, Default)]
pub(crate) struct WsUpdate(pub Option<GameWsUpdateMsg>);
