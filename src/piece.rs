use std::f32::consts::E;

use crate::{
    board,
    piece_matrix::PieceMatrix,
    square::{
        self, ALL_DIRECTIONS, DIAGONAL_DIRECTIONS, Direction, LATTERAL_DIRECTIONS, Move, Square,
    },
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
    pub fn is_valid_move(&self, mv: &Move, board: &PieceMatrix) -> bool {
        let valid_destinations: Vec<Square> = self.valid_destinations(mv.from, board);
        valid_destinations.contains(&mv.to)
    }

    pub fn is_valid_capture_move(&self, mv: &Move, board: &PieceMatrix) -> bool {
        let valid_capture_destinations: Vec<Square> =
            self.valid_capture_destinations(&mv.from, board);
        valid_capture_destinations.contains(&mv.to)
    }

    fn valid_destinations(&self, start: Square, board: &PieceMatrix) -> Vec<Square> {
        match self.piece_type {
            PieceType::Pawn => pawn_move_destinations(start, self.color, board),
            PieceType::Knight => start.l_shapes(),

            PieceType::King => ALL_DIRECTIONS
                .iter()
                .filter_map(|direction| start.move_in_direction(&direction).ok())
                .filter(|square| board.get_piece(square).is_none())
                .collect(),

            piece_type => piece_type
                .movement_directions()
                .unwrap()
                .iter()
                .flat_map(|direction| squares_until_blocked(&start, direction, board))
                .collect(),
        }
    }

    fn valid_capture_destinations(&self, start: &Square, board: &PieceMatrix) -> Vec<Square> {
        match self.piece_type {
            PieceType::Pawn => pawn_capture_destinations(*start, self.color),
            PieceType::Knight => start.l_shapes(),

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
                    first_enemy_piece_square(start, direction, board, self.color)
                })
                .collect(),
        }
    }
}

fn squares_until_blocked(
    start: &Square,
    direction: &Direction,
    board: &PieceMatrix,
) -> Vec<Square> {
    let mut squares: Vec<Square> = Vec::new();
    let mut current_square = start.move_in_direction(&direction);
    while let Ok(s) = current_square {
        if board.get_piece(&s).is_some() {
            break;
        } else {
            squares.push(s);
            current_square = s.move_in_direction(&direction);
        }
    }
    squares
}

fn first_enemy_piece_square(
    start: &Square,
    direction: &Direction,
    board: &PieceMatrix,
    color: Color,
) -> Option<Square> {
    let unoccupied_squares = squares_until_blocked(start, direction, board);
    let last_unoccupied_square = unoccupied_squares.last().unwrap_or(start);
    let first_occupied_square = last_unoccupied_square.move_in_direction(direction).ok()?;
    let occupant_piece = board.get_piece(&first_occupied_square).unwrap();
    if occupant_piece.color != color {
        Some(first_occupied_square)
    } else {
        None
    }
}

fn pawn_move_destinations(start: Square, color: Color, board: &PieceMatrix) -> Vec<Square> {
    let direction = match color {
        Color::White => Direction::Up,
        Color::Black => Direction::Down,
    };

    let basic_move_square = start.move_in_direction(&direction).unwrap();

    if board.get_piece(&basic_move_square).is_some() {
        vec![]
    } else {
        let home_rank = match color {
            Color::White => 1,
            Color::Black => 6,
        };

        if start.rank == home_rank {
            vec![
                basic_move_square,
                basic_move_square.move_in_direction(&direction).unwrap(),
            ]
        } else {
            vec![basic_move_square]
        }
    }
}

fn pawn_capture_destinations(start: Square, color: Color) -> Vec<Square> {
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
