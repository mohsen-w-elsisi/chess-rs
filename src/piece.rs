use crate::{
    piece,
    square::{Move, Square},
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
    pub fn is_valid_move(&self, mv: &Move) -> bool {
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

            PieceType::Pawn => match self.color {
                Color::White => {
                    let basic_move = mv.from.up().unwrap();
                    if mv.from.rank == 1 {
                        vec![basic_move, basic_move.up().unwrap()]
                    } else {
                        vec![basic_move]
                    }
                },
                Color::Black => {
                    let basic_move = mv.from.down().unwrap();
                    if mv.from.rank == 6 {
                        vec![basic_move, basic_move.down().unwrap()]
                    } else {
                        vec![basic_move]
                    }
                }
            },

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
}
