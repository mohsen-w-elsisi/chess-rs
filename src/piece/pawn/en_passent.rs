use crate::{
    board::Board,
    r#move::Move,
    piece::{
        Color, Piece, PieceType,
        pawn::{home_rank, home_rank_plus, movement_direction},
    },
    square::Square,
};

pub fn get_moves(board: &Board, from: &Square, color: Color) -> Option<Move> {
    let (last_move, last_piece) = match destruct_last_histroy_entry(board.history()) {
        Ok((m, p)) => (m, p),
        Err(_) => return None,
    };

    let last_to = match last_move {
        Move::Normal { from: _, to } => to,
        _ => return None,
    };

    if !move_is_double_pawn(last_move, last_piece, color.opposite()) {
        return None;
    }

    if !pawns_are_adjacent(from, &last_to) {
        return None;
    }

    let capturing_pawn_destination = last_to
        .move_in_direction(&movement_direction(color))
        .unwrap();

    Some(Move::EnPassent {
        from: *from,
        to: capturing_pawn_destination,
    })
}

fn move_is_double_pawn(last_move: &Move, piece: &Piece, color: Color) -> bool {
    if piece.piece_type != PieceType::Pawn {
        return false;
    }

    match last_move {
        Move::Normal { from, to } => {
            if from.rank != home_rank(color) {
                return false;
            }
            if to.rank != home_rank_plus(2, color) {
                return false;
            }
            return true;
        }
        _ => return false,
    }
}

fn pawns_are_adjacent(capturing_square: &Square, captured_square: &Square) -> bool {
    if capturing_square.rank != captured_square.rank {
        false
    } else {
        (capturing_square.file as i8 - captured_square.file as i8).abs() == 1
    }
}

fn destruct_last_histroy_entry(history: &Vec<(Move, Piece)>) -> Result<(&Move, &Piece), Error> {
    match history.last() {
        Some((m, p)) => Ok((m, p)),
        None => Err(Error::HistoryEmpty),
    }
}

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
