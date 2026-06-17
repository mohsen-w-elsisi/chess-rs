pub mod promotion;
pub mod en_passent;

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