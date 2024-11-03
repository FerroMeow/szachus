use crate::game::ChessPieceColorEnum;
use bevy::prelude::*;

use super::ChessPieceTypeEnum;

#[derive(Component, Debug)]
pub(crate) struct ChessBoardTile {
    pub x: i32,
    pub y: i32,
}

impl ChessBoardTile {
    fn get_color(&self, pieces: &[ChessPiece]) -> Option<ChessPieceColorEnum> {
        pieces
            .iter()
            .find(|piece| piece.x == self.x && piece.y == self.y)
            .map(|piece| piece.color)
    }

    fn is_path_empty(&self, end: &ChessBoardTile, pieces: &[ChessPiece]) -> bool {
        let x_diff = (self.x - end.x).abs();
        let y_diff = (self.y - end.y).abs();
        if x_diff == 0
            && pieces.iter().any(|piece| {
                piece.x == self.x
                    && ((self.y < piece.y && piece.y < end.y)
                        || (end.y < piece.y && piece.y < self.y))
            })
        {
            return false;
        }
        if y_diff == 0
            && pieces.iter().any(|piece| {
                piece.y == self.y
                    && ((self.x < piece.x && piece.x < end.x)
                        || (end.x < piece.x && piece.x < self.x))
            })
        {
            return false;
        }
        if x_diff == y_diff
            && (1..x_diff).any(|i| {
                let current_tile = if self.x < end.x && self.y < end.y {
                    ChessBoardTile {
                        x: self.x + i,
                        y: self.y + i,
                    }
                } else if self.x < end.x && self.y > end.y {
                    ChessBoardTile {
                        x: self.x + i,
                        y: self.y - i,
                    }
                } else if self.x > end.x && self.y < end.y {
                    ChessBoardTile {
                        x: self.x - i,
                        y: self.y + i,
                    }
                } else {
                    ChessBoardTile {
                        x: self.x - i,
                        y: self.y - i,
                    }
                };
                current_tile.get_color(pieces).is_some()
            })
        {
            return false;
        }
        true
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub(crate) struct ChessPiece {
    pub x: i32,
    pub y: i32,
    pub piece_type: ChessPieceTypeEnum,
    pub color: ChessPieceColorEnum,
    pub alive: bool,
}

impl ChessPiece {
    pub(crate) fn is_move_valid(
        &self,
        new_position: &ChessBoardTile,
        pieces: &[ChessPiece],
    ) -> bool {
        use ChessPieceTypeEnum as PieceType;
        if new_position.get_color(pieces) == Some(self.color) {
            return false;
        }
        let x_diff = (self.x - new_position.x).abs();
        let y_diff = (self.y - new_position.y).abs();
        let start_position = ChessBoardTile {
            x: self.x,
            y: self.y,
        };
        match self.piece_type {
            PieceType::King => {
                // Horizontal
                (x_diff != 0 || y_diff != 0) && x_diff <= 1 && y_diff <= 1
            }
            PieceType::Queen => {
                start_position.is_path_empty(new_position, pieces)
                    && (x_diff == y_diff
                        || ((x_diff == 0 && y_diff != 0) || (y_diff == 0 && x_diff != 0)))
            }
            PieceType::Bishop => {
                start_position.is_path_empty(new_position, pieces) && x_diff == y_diff
            }
            PieceType::Knight => (x_diff == 2 && y_diff == 1) || (x_diff == 1 && y_diff == 2),
            PieceType::Rook => {
                start_position.is_path_empty(new_position, pieces)
                    && ((x_diff == 0 && y_diff != 0) || (y_diff == 0 && x_diff != 0))
            }
            PieceType::Pawn => {
                if y_diff == 1 && (x_diff == 0) && new_position.get_color(pieces).is_none() {
                    return true;
                }

                // Move 2 squares
                if self.y == 1
                    && y_diff == 2
                    && x_diff == 0
                    && start_position.is_path_empty(new_position, pieces)
                    && new_position.get_color(pieces).is_none()
                {
                    return true;
                }

                // Take piece
                if y_diff == 1
                    && x_diff == 1
                    && new_position.get_color(pieces) == Some(self.color.opposite())
                {
                    return true;
                }
                false
            }
        }
    }
}

#[derive(Component)]
pub(crate) struct MainCamera;
