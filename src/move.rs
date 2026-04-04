use crate::{piece::{Color, PieceType}, square::Square};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Move {
    Normal { from: Square, to: Square },
    Capture { from: Square, to: Square },
    Castle { side: CastleSide, color: Color },
    Promotion { from: Square, to: Square, capture: bool, promotion_piece_type: PieceType },
    EnPassent { from: Square, to: Square },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CastleSide {
    KingSide,
    QueenSide,
}
