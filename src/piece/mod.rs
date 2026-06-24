pub mod pawn;

use crate::{
    board::Board,
    r#move::Move,
    piece,
    piece_matrix::{self, PieceMatrix},
    square::{ALL_DIRECTIONS, DIAGONAL_DIRECTIONS, Direction, LATTERAL_DIRECTIONS, Square},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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

    pub fn has_long_range_movement(&self) -> bool {
        match self {
            PieceType::Pawn => false,
            PieceType::Knight => false,
            PieceType::King => false,
            _ => true,
        }
    }

    pub fn value(&self) -> f64 {
        match self {
            PieceType::Pawn => 1.0,
            PieceType::Knight => 3.0,
            PieceType::Bishop => 3.0,
            PieceType::Rook => 5.0,
            PieceType::Queen => 9.0,
            PieceType::King => 0.0, // King is invaluable
        }
    }
}

impl Piece {
    pub fn get_available_moves(&self, from: &Square, board: &Board) -> Vec<Move> {
        let piece_matrix = &board.matrix();
        let mut available_moves: Vec<Move> = Vec::new();

        available_moves.extend(
            self.valid_destinations(from, piece_matrix)
                .iter()
                .map(|square| Move::Normal {
                    from: *from,
                    to: *square,
                }),
        );

        available_moves.extend(
            self.valid_capture_destinations(from, piece_matrix)
                .iter()
                .map(|square| Move::Capture {
                    from: *from,
                    to: *square,
                }),
        );

        if self.piece_type == PieceType::Pawn {
            available_moves.extend(pawn::promotion::available_promotions(
                from,
                self.color,
                piece_matrix,
            ));

            available_moves.extend(pawn::en_passent::get_moves(board, from, self.color));
        }

        return available_moves;
    }

    pub fn is_valid_move(&self, from: &Square, to: &Square, board: &PieceMatrix) -> bool {
        let valid_destinations: Vec<Square> = self.valid_destinations(from, board);
        valid_destinations.contains(&to)
    }

    pub fn is_valid_capture_move(&self, from: &Square, to: &Square, board: &PieceMatrix) -> bool {
        let valid_capture_destinations: Vec<Square> = self.valid_capture_destinations(&from, board);
        valid_capture_destinations.contains(&to)
    }

    pub fn valid_destinations(&self, start: &Square, board: &PieceMatrix) -> Vec<Square> {
        match self.piece_type {
            PieceType::Pawn => pawn::move_destinations(start, self.color, board)
                .into_iter()
                .filter(|square| !board.is_occupied(square))
                .collect(),

            PieceType::Knight => start
                .l_shapes()
                .into_iter()
                .filter(|square| !board.is_occupied(square))
                .collect(),

            PieceType::King => ALL_DIRECTIONS
                .iter()
                .filter_map(|direction| start.move_in_direction(&direction).ok())
                .filter(|square| !board.is_occupied(square))
                .collect(),

            piece_type => piece_type
                .movement_directions()
                .unwrap()
                .iter()
                .flat_map(|direction| board.squares_until_blocked(start, direction))
                .collect(),
        }
    }

    pub fn valid_capture_destinations(&self, start: &Square, board: &PieceMatrix) -> Vec<Square> {
        match self.piece_type {
            PieceType::Pawn => pawn::capture_destinations(start, self.color)
                .into_iter()
                .filter(|square| board.is_occupied_by_color(square, self.color.opposite()))
                .collect(),

            PieceType::Knight => start
                .l_shapes()
                .into_iter()
                .filter(|square| board.is_occupied_by_color(square, self.color.opposite()))
                .collect(),

            PieceType::King => ALL_DIRECTIONS
                .iter()
                .filter_map(|direction| start.move_in_direction(&direction).ok())
                .filter(|square| board.is_occupied_by_color(square, self.color.opposite()))
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
