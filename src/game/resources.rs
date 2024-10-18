use bevy::prelude::*;

use super::ChessPieceColorEnum;

#[derive(Resource)]
pub(crate) struct PlayerColorResource(pub ChessPieceColorEnum);
