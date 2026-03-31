use crate::{piece::{Color, PieceType}, square::Square};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Move {
    Normal { from: Square, to: Square },
    Capture { from: Square, to: Square },
    Castle { side: CastleSide, color: Color },
    // Promotion { mv: PromotionMove },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CastleSide {
    KingSide,
    QueenSide,
}
