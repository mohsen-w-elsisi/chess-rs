pub mod en_passent;
pub mod promotion;

use crate::{
    piece::Color,
    piece_matrix::PieceMatrix,
    square::{Direction, Square},
};

pub fn move_destinations(start: &Square, color: Color, board: &PieceMatrix) -> Vec<Square> {
    match color {
        Color::White if start.rank >= 6 => return vec![],
        Color::Black if start.rank <= 1 => return vec![],
        _ => {}
    }

    let basic_move_square = forward_square(start, color).unwrap();

    if board.get_piece(&basic_move_square).is_some() {
        return vec![];
    }

    let is_on_home_rank = start.rank == home_rank(color);

    if !is_on_home_rank {
        return vec![basic_move_square];
    } else {
        let double_move_square = forward_square(&basic_move_square, color).unwrap();
        return vec![basic_move_square, double_move_square];
    }
}

pub fn capture_destinations(start: &Square, color: Color) -> Vec<Square> {
    match color {
        Color::White if start.rank >= 6 => return vec![],
        Color::Black if start.rank <= 1 => return vec![],
        _ => {}
    }

    diagonal_squares(start, color)
}

fn movement_direction(color: Color) -> Direction {
    match color {
        Color::White => Direction::Up,
        Color::Black => Direction::Down,
    }
}

fn home_rank(color: Color) -> u8 {
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

pub fn forward_square(start: &Square, color: Color) -> Option<Square> {
    let direction = movement_direction(color);
    start.move_in_direction(&direction).ok()
}

pub fn diagonal_squares(start: &Square, color: Color) -> Vec<Square> {
    let mut diagonal_squares = Vec::new();

    let forward_square = forward_square(start, color).unwrap();
    
    if let Ok(right_diagonal_square) = forward_square.right() {
        diagonal_squares.push(right_diagonal_square);
    }

    if let Ok(left_diagonal_square) = forward_square.left() {
        diagonal_squares.push(left_diagonal_square);
    }

    diagonal_squares
}
