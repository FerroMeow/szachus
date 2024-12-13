use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Position {
    pub column: i8,
    pub row: i8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ChessMove {
    pub position_from: Position,
    pub position_to: Position,
}
