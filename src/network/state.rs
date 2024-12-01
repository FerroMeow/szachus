use bevy::prelude::States;

#[derive(States, Debug, PartialEq, Eq, Hash, Clone, Default)]
pub enum ConnectionState {
    #[default]
    NotConnected,
    WebSocket,
}
