use crate::{
    r#move::Move,
    piece::{Color, Piece, PieceType},
    piece_matrix::PieceMatrix,
    square::{Direction, Square},
};

pub fn move_destinations(start: &Square, color: Color, board: &PieceMatrix) -> Vec<Square> {
    let direction = match color {
        Color::White => Direction::Up,
        Color::Black => Direction::Down,
    };

    let basic_move_square = start.move_in_direction(&direction).unwrap();

    if board.get_piece(&basic_move_square).is_some() {
        return vec![];
    }

    let is_on_home_rank = match color {
        Color::White => start.rank == 1,
        Color::Black => start.rank == 6,
    };

    if !is_on_home_rank {
        return vec![basic_move_square];
    } else {
        let double_move_square = basic_move_square.move_in_direction(&direction).unwrap();
        if board.get_piece(&double_move_square).is_none() {
            return vec![basic_move_square, double_move_square];
        } else {
            return vec![basic_move_square];
        }
    }
}

pub fn capture_destinations(start: &Square, color: Color) -> Vec<Square> {
    let forward_square = match color {
        Color::White => start.up(),
        Color::Black => start.down(),
    }
    .unwrap();

    let mut valid_capture_destinations: Vec<Square> = Vec::new();

    if let Ok(right_diagonal_square) = forward_square.right() {
        valid_capture_destinations.push(right_diagonal_square);
    }

    if let Ok(left_diagonal_square) = forward_square.left() {
        valid_capture_destinations.push(left_diagonal_square);
    }

    valid_capture_destinations
}

pub fn is_valid_promotion(
    from: &Square,
    to: &Square,
    capture: bool,
    promotion_piece_type: &PieceType,
    board: &PieceMatrix,
) -> Result<(), String> {
    let pawn = board
        .get_piece(from)
        .ok_or("No pawn to promote".to_string())?;

    if pawn.piece_type != PieceType::Pawn {
        return Err("Only pawns can be promoted".to_string());
    }

    if !VALID_PROMOTION_PIECE_TYPES.contains(promotion_piece_type) {
        return Err("Cannot promote to that piece type".to_string());
    }

    if capture {
        if !pawn.is_valid_capture_move(from, to, board) {
            return Err("Invalid capture move for promotion".to_string());
        }
    } else {
        if !pawn.is_valid_move(from, to, board) {
            return Err("Invalid move for promotion".to_string());
        }
    }

    Ok(())
}

const VALID_PROMOTION_PIECE_TYPES: [PieceType; 4] = [
    PieceType::Queen,
    PieceType::Rook,
    PieceType::Bishop,
    PieceType::Knight,
];

pub fn is_valid_en_passent(
    from: &Square,
    to: &Square,
    history: &Vec<(Move, Piece)>,
) -> Result<(), String> {
    let (last_move, last_piece) = history
        .last()
        .ok_or("No previous move to compare with".to_string())?;

    if last_piece.piece_type != PieceType::Pawn {
        return Err("Last move was not a pawn move".to_string());
    }

    if from.rank != home_rank_plus(2, last_piece.color) {
        return Err("En passent can only be performed from 4th or 5th rank".to_string());
    }

    if to.rank != home_rank_plus(1, last_piece.color) {
        return Err("En passent move must be to the 3rd or 6th rank".to_string());
    }

    if let Move::Normal {
        from: last_from,
        to: last_to,
    } = last_move
    {
        let direction = movement_direction(last_piece.color);
        let expected_to = last_from
            .move_in_direction(&direction)
            .map_err(|_| "invalid last move for en passent".to_string())?;
        let last_expected_to = expected_to
            .move_in_direction(&direction)
            .map_err(|_| "invalid last move for en passent".to_string())?;
        if last_expected_to != *last_to {
            return Err("Last move was not a double step pawn move".to_string());
        }
        if expected_to != *to {
            return Err("En passent move must be to the square behind the pawn that moved two steps".to_string());
        }
    } else {
        return Err("Last move was not a normal pawn move".to_string());
    }

    Ok(())
}

fn movement_direction(color: Color) -> Direction {
    match color {
        Color::White => Direction::Up,
        Color::Black => Direction::Down,
    }
}

fn home_rank(color: Color) -> usize {
    match color {
        Color::White => 1,
        Color::Black => 6,
    }
}

pub fn home_rank_plus(offset: u8, color: Color) -> u8 {
    match color {
        Color::White => 1 + offset,
        Color::Black => 6 - offset,
    }
}