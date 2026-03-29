use std::f32::consts::E;

use crate::{
    piece_matrix::PieceMatrix,
    square::{self, Direction, Move, Square},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl Piece {
    pub fn is_valid_move(&self, mv: &Move, board: &PieceMatrix) -> bool {
        let valid_destinations: Vec<Square> = match self.piece_type {
            PieceType::King => {
                let potentially_valid_squares = [
                    mv.from.up(),
                    mv.from.down(),
                    mv.from.left(),
                    mv.from.right(),
                    mv.from.up_left(),
                    mv.from.up_right(),
                    mv.from.down_left(),
                    mv.from.down_right(),
                ];

                let mut valid_squares: Vec<Square> = Vec::new();
                for square in potentially_valid_squares {
                    if let Ok(s) = square {
                        valid_squares.push(s);
                    }
                }

                valid_squares
            }

            PieceType::Pawn => {
                let direction = match self.color {
                    Color::White => Direction::Up,
                    Color::Black => Direction::Down,
                };

                let basic_move_square = mv.from.move_in_direction(&direction).unwrap();

                if board.get_piece(&basic_move_square).is_some() {
                    vec![]
                } else {
                    let home_rank = match self.color {
                        Color::White => 1,
                        Color::Black => 6,
                    };

                    if mv.from.rank == home_rank {
                        vec![
                            basic_move_square,
                            basic_move_square.move_in_direction(&direction).unwrap(),
                        ]
                    } else {
                        vec![basic_move_square]
                    }
                }
            }

            PieceType::Rook => mv.from.laterals(),
            PieceType::Bishop => mv.from.diagonals(),
            PieceType::Knight => mv.from.l_shapes(),
            PieceType::Queen => {
                let mut valid_destinations = mv.from.laterals();
                valid_destinations.extend(mv.from.diagonals());
                valid_destinations
            }
        };

        valid_destinations.contains(&mv.to)
    }

    pub fn is_valid_capture_move(&self, mv: &Move, board: &PieceMatrix) -> bool {
        if self.piece_type != PieceType::Pawn {
            return self.is_valid_move(mv, board);
        }

        let valid_capture_destinations: Vec<Square> = {
            let mut valid_capture_destinations: Vec<Square> = Vec::new();

            let forward_square = match self.color {
                Color::White => mv.from.up().unwrap(),
                Color::Black => mv.from.down().unwrap(),
            };

            if let Ok(right_diagonal_square) = forward_square.right() {
                valid_capture_destinations.push(right_diagonal_square);
            }

            if let Ok(left_diagonal_square) = forward_square.left() {
                valid_capture_destinations.push(left_diagonal_square);
            }

            valid_capture_destinations
        };

        valid_capture_destinations.contains(&mv.to)
    }
}
