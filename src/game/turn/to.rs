use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Row(pub u8);

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Column(pub u8);

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Position {
    pub row: Row,
    pub column: Column,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ChessMove {
    pub position_from: Position,
    pub position_to: Position,
}
