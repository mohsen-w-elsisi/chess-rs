use std::ptr::NonNull;

use crate::{
    r#move::Move,
    piece::{
        Color, PieceType,
        pawn::{diagonal_squares, forward_square, home_rank},
    },
    piece_matrix::PieceMatrix,
    square::Square,
};

pub fn is_valid(
    from: &Square,
    to: &Square,
    capture: bool,
    promotion_piece_type: &PieceType,
    board: &PieceMatrix,
) -> Result<(), Error> {
    let pawn = board.get_piece(from).ok_or(Error::PawnNotFound)?;

    if pawn.piece_type != PieceType::Pawn {
        return Err(Error::PieceNotPawn);
    }

    if !VALID_PROMOTION_PIECE_TYPES.contains(promotion_piece_type) {
        return Err(Error::InvalidPromotionPieceType);
    }

    if from.rank != home_rank(pawn.color.opposite()) {
        return Err(Error::InvalidPawnMove);
    }

    if capture {
        if !diagonal_squares(from, pawn.color).contains(to)
            || !board.is_occupied_by_color(to, pawn.color.opposite())
        {
            return Err(Error::InvalidPawnMove);
        }
    } else {
        if *to != forward_square(from, pawn.color).unwrap() || board.is_occupied(to) {
            return Err(Error::InvalidPawnMove);
        }
    }

    Ok(())
}

pub fn available_promotions(start: &Square, color: Color, board: &PieceMatrix) -> Vec<Move> {
    if start.rank != home_rank(color.opposite()) {
        return vec![];
    }

    let mut available_promotion_squares: Vec<Square> = Vec::new();

    let forward_promotion = forward_square(start, color).unwrap();
    if board.get_piece(&forward_promotion).is_none() {
        available_promotion_squares.push(forward_promotion);
    }

    diagonal_squares(start, color)
        .into_iter()
        .filter(|square| board.is_occupied_by_color(square, color.opposite()))
        .for_each(|square| available_promotion_squares.push(square));

    return available_promotion_squares
        .iter()
        .flat_map(|square| {
            VALID_PROMOTION_PIECE_TYPES
                .iter()
                .map(|piece_type| (*square, piece_type))
        })
        .map(|(square, piece_type)| Move::Promotion {
            from: *start,
            to: square,
            promotion_piece_type: *piece_type,
            capture: square != forward_promotion,
        })
        .collect();
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
