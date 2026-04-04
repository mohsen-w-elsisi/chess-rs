pub mod pawn;

use crate::{
    piece_matrix::PieceMatrix,
    square::{ALL_DIRECTIONS, DIAGONAL_DIRECTIONS, Direction, LATTERAL_DIRECTIONS, Square},
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

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
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

impl PieceType {
    pub fn movement_directions(&self) -> Option<&[Direction]> {
        match self {
            PieceType::Pawn => None,
            PieceType::Knight => None,
            PieceType::Rook => Some(&LATTERAL_DIRECTIONS),
            PieceType::Bishop => Some(&DIAGONAL_DIRECTIONS),
            PieceType::Queen => Some(&ALL_DIRECTIONS),
            PieceType::King => None,
        }
    }
}

impl Piece {
    pub fn is_valid_move(&self, from: &Square, to: &Square, board: &PieceMatrix) -> bool {
        let valid_destinations: Vec<Square> = self.valid_destinations(from, board);
        valid_destinations.contains(&to)
    }

    pub fn is_valid_capture_move(&self, from: &Square, to: &Square, board: &PieceMatrix) -> bool {
        let valid_capture_destinations: Vec<Square> = self.valid_capture_destinations(&from, board);
        valid_capture_destinations.contains(&to)
    }

    fn valid_destinations(&self, start: &Square, board: &PieceMatrix) -> Vec<Square> {
        match self.piece_type {
            PieceType::Pawn => pawn::move_destinations(start, self.color, board),
            PieceType::Knight => start.l_shapes(), // doesn't check if square occupied

            PieceType::King => ALL_DIRECTIONS
                .iter()
                .filter_map(|direction| start.move_in_direction(&direction).ok())
                .filter(|square| board.get_piece(square).is_none())
                .collect(),

            piece_type => piece_type
                .movement_directions()
                .unwrap()
                .iter()
                .flat_map(|direction| board.squares_until_blocked(start, direction))
                .collect(),
        }
    }

    fn valid_capture_destinations(&self, start: &Square, board: &PieceMatrix) -> Vec<Square> {
        match self.piece_type {
            PieceType::Pawn => pawn::capture_destinations(start, self.color),
            PieceType::Knight => start.l_shapes(), // doesn't check if square occupied

            PieceType::King => ALL_DIRECTIONS
                .iter()
                .filter_map(|direction| start.move_in_direction(&direction).ok())
                .filter(|square| {
                    if let Some(occupant_piece) = board.get_piece(square) {
                        occupant_piece.color != self.color
                    } else {
                        false
                    }
                })
                .collect(),

            piece_type => piece_type
                .movement_directions()
                .unwrap()
                .iter()
                .filter_map(|direction| {
                    board.first_enemy_piece_square(start, direction, self.color)
                })
                .collect(),
        }
    }
}
