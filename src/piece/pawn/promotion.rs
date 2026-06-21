use crate::{piece::{Color, PieceType}, piece_matrix::PieceMatrix, square::Square};

pub fn is_valid(
    from: &Square,
    to: &Square,
    capture: bool,
    promotion_piece_type: &PieceType,
    board: &PieceMatrix,
) -> Result<(), Error> {
    let pawn = board
        .get_piece(from)
        .ok_or(Error::PawnNotFound)?;

    if pawn.piece_type != PieceType::Pawn {
        return Err(Error::PieceNotPawn);
    }

    if !VALID_PROMOTION_PIECE_TYPES.contains(promotion_piece_type) {
        return Err(Error::InvalidPromotionPieceType);
    }

    if capture {
        if !pawn.is_valid_capture_move(from, to, board) {
            return Err(Error::InvalidPawnMove);
        }
    } else {
        if !pawn.is_valid_move(from, to, board) {
            return Err(Error::InvalidPawnMove);
        }
    }

    Ok(())
}

pub fn available_promotions(
    start: &Square,
    color: Color,
    board: &PieceMatrix,
) -> Vec<PieceType> {
    unimplemented!()
}

#[derive(Debug)]
pub enum Error {
    PawnNotFound,
    PieceNotPawn,
    InvalidPawnMove,
    InvalidPromotionPieceType,
}

const VALID_PROMOTION_PIECE_TYPES: [PieceType; 4] = [
    PieceType::Queen,
    PieceType::Rook,
    PieceType::Bishop,
    PieceType::Knight,
];
