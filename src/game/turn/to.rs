use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Row(pub u8);

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Column(pub u8);

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Position {
    pub row: Row,
    pub column: Column,
}

#[derive(Serialize)]
pub(crate) struct ChessMove {
    pub position_from: Position,
    pub position_to: Position,
}
