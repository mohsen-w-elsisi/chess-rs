use crate::{
    r#move::Move,
    piece::{
        Piece, PieceType,
        pawn::{home_rank_plus, movement_direction},
    },
    square::Square,
};

pub fn is_valid(from: &Square, to: &Square, history: &Vec<(Move, Piece)>) -> Result<(), Error> {
    let (last_move, last_piece) = history.last().ok_or(Error::HistoryEmpty)?;

    if last_piece.piece_type != PieceType::Pawn {
        return Err(Error::LastMoveNotPawn);
    }

    if from.rank != home_rank_plus(2, last_piece.color) {
        return Err(Error::InvalidFromRank);
    }

    if to.rank != home_rank_plus(1, last_piece.color) {
        return Err(Error::InvalidToRank);
    }

    if let Move::Normal {
        from: last_from,
        to: last_to,
    } = last_move
    {
        let direction = movement_direction(last_piece.color);
        let expected_to = last_from
            .move_in_direction(&direction)
            .map_err(|_| Error::LastMoveNotDoubleStep)?;
        let last_expected_to = expected_to
            .move_in_direction(&direction)
            .map_err(|_| Error::LastMoveNotDoubleStep)?;
        if last_expected_to != *last_to {
            return Err(Error::LastMoveNotDoubleStep);
        }
        if expected_to != *to {
            return Err(Error::InvalidToFile);
        }
    } else {
        return Err(Error::LastMoveNotNormal);
    }

    Ok(())
}

#[derive(Debug)]
pub enum Error {
    HistoryEmpty,
    LastMoveNotPawn,
    InvalidFromRank,
    InvalidToRank,
    LastMoveNotDoubleStep,
    InvalidToFile,
    LastMoveNotNormal,
}
