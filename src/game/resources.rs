use bevy::prelude::*;

use super::ChessPieceColorEnum;

#[derive(Resource, Default)]
pub(crate) struct PlayerColorResource(pub ChessPieceColorEnum);
