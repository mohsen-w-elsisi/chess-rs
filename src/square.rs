use std::collections::HashMap;

use crate::{
    board::Board,
    piece::{Color, Piece, PieceType},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Square {
    pub file: u8,
    pub rank: u8,
}

impl Square {
    pub fn from_flat_index(index: u8) -> Square {
        Square {
            file: index % 8,
            rank: index / 8,
        }
    }

    pub fn to_flat_index(&self) -> u8 {
        self.rank * 8 + self.file
    }

    pub fn is_valid(&self) -> bool {
        self.rank < 8 && self.file < 8
    }

    pub fn up(&self) -> Result<Square, MoveError> {
        if self.rank == 7 {
            return Err(MoveError);
        }
        Ok(Square {
            file: self.file,
            rank: self.rank + 1,
        })
    }

    pub fn down(&self) -> Result<Square, MoveError> {
        if self.rank == 0 {
            return Err(MoveError);
        }
        Ok(Square {
            file: self.file,
            rank: self.rank - 1,
        })
    }

    pub fn left(&self) -> Result<Square, MoveError> {
        if self.file == 0 {
            return Err(MoveError);
        }
        Ok(Square {
            file: self.file - 1,
            rank: self.rank,
        })
    }

    pub fn right(&self) -> Result<Square, MoveError> {
        if self.file == 7 {
            return Err(MoveError);
        }
        Ok(Square {
            file: self.file + 1,
            rank: self.rank,
        })
    }

    pub fn up_left(&self) -> Result<Square, MoveError> {
        self.up()?.left()
    }

    pub fn up_right(&self) -> Result<Square, MoveError> {
        self.up()?.right()
    }

    pub fn down_left(&self) -> Result<Square, MoveError> {
        self.down()?.left()
    }

    pub fn down_right(&self) -> Result<Square, MoveError> {
        self.down()?.right()
    }

    pub fn laterals(&self) -> Vec<Square> {
        let mut laterals: Vec<Square> = Vec::new();

        let mut current_square = self.up();
        while let Ok(s) = current_square {
            laterals.push(s);
            current_square = s.up();
        }

        current_square = self.down();
        while let Ok(s) = current_square {
            laterals.push(s);
            current_square = s.down();
        }

        current_square = self.left();
        while let Ok(s) = current_square {
            laterals.push(s);
            current_square = s.left();
        }

        current_square = self.right();
        while let Ok(s) = current_square {
            laterals.push(s);
            current_square = s.right();
        }

        laterals
    }

    pub fn diagonals(&self) -> Vec<Square> {
        let mut diagonals: Vec<Square> = Vec::new();

        let mut current_square = self.up_left();
        while let Ok(s) = current_square {
            diagonals.push(s);
            current_square = s.up_left();
        }

        current_square = self.up_right();
        while let Ok(s) = current_square {
            diagonals.push(s);
            current_square = s.up_right();
        }

        current_square = self.down_left();
        while let Ok(s) = current_square {
            diagonals.push(s);
            current_square = s.down_left();
        }

        current_square = self.down_right();
        while let Ok(s) = current_square {
            diagonals.push(s);
            current_square = s.down_right();
        }

        diagonals
    }

    pub fn l_shapes(&self) -> Vec<Square> {
        let potentially_valid_squares = [
            self.up_left().and_then(|s| s.up()),
            self.up_left().and_then(|s| s.left()),
            self.up_right().and_then(|s| s.up()),
            self.up_right().and_then(|s| s.right()),
            self.down_left().and_then(|s| s.down()),
            self.down_left().and_then(|s| s.left()),
            self.down_right().and_then(|s| s.down()),
            self.down_right().and_then(|s| s.right()),
        ];

        let mut valid_squares: Vec<Square> = Vec::new();
        for square in potentially_valid_squares {
            if let Ok(s) = square {
                valid_squares.push(s);
            }
        }

        valid_squares
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
}

impl Move {
    pub fn from_standard_notation(notation: &str, board: &Board, side: Color) -> Option<Move> {
        let piece_standard_notation: HashMap<&str, PieceType> = HashMap::from([
            ("R", PieceType::Rook),
            ("N", PieceType::Knight),
            ("B", PieceType::Bishop),
            ("Q", PieceType::Queen),
            ("K", PieceType::King),
        ]);

        let piece_type: PieceType = {
            let first_char = notation.chars().next().unwrap();
            if first_char.is_ascii_uppercase() {
                let first_char_string = &first_char.to_string()[..];
                piece_standard_notation
                    .get(first_char_string)
                    .unwrap()
                    .to_owned()
            } else {
                PieceType::Pawn
            }
        };

        let piece = Piece {
            piece_type,
            color: side,
        };

        let potential_pieces = board.find_piece(piece);

        let destination: Square = {
            let destination_indicater = notation[notation.len() - 2..].chars();
            let file = destination_indicater.clone().next().unwrap() as u8 - 'a' as u8;
            let rank = destination_indicater.last().unwrap().to_digit(10).unwrap() as u8 - 1;
            Square { file, rank }
        };

        for piece_square in potential_pieces {
            let mv = Move {
                from: piece_square,
                to: destination,
            };
            if piece.is_valid_move(&mv) {
                // todo!("did not handle multiple pieces targetting the same square");
                return Some(mv);
            }
        }

        return None;
    }
}

#[derive(Debug)]
pub struct MoveError;
