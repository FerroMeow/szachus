use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Position {
    pub column: i8,
    pub row: i8,
}

impl Position {
    pub fn new(column: i32, row: i32) -> Self {
        Position {
            column: column as i8,
            row: row as i8,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ChessMove {
    pub position_from: Position,
    pub position_to: Position,
}
