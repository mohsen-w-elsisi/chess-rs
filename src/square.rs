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

    pub fn search_in_direction(&self, direction: Direction) -> Vec<Square> {
        let mut squares: Vec<Square> = Vec::new();
        let mut current_square = self.move_in_direction(&direction);
        while let Ok(s) = current_square {
            squares.push(s);
            current_square = s.move_in_direction(&direction);
        }
        squares
    }

    pub fn move_in_direction(&self, direction: &Direction) -> Result<Square, MoveError> {
        if direction.boundry_check(self) {
            Ok(direction.move_in_direction(self))
        } else {
            Err(MoveError)
        }
    }

    pub fn up(&self) -> Result<Square, MoveError> {
        self.move_in_direction(&Direction::Up)
    }

    pub fn down(&self) -> Result<Square, MoveError> {
        self.move_in_direction(&Direction::Down)
    }

    pub fn left(&self) -> Result<Square, MoveError> {
        self.move_in_direction(&Direction::Left)
    }

    pub fn right(&self) -> Result<Square, MoveError> {
        self.move_in_direction(&Direction::Right)
    }

    pub fn up_left(&self) -> Result<Square, MoveError> {
        self.move_in_direction(&Direction::UpLeft)
    }

    pub fn up_right(&self) -> Result<Square, MoveError> {
        self.move_in_direction(&Direction::UpRight)
    }

    pub fn down_left(&self) -> Result<Square, MoveError> {
        self.move_in_direction(&Direction::DownLeft)
    }

    pub fn down_right(&self) -> Result<Square, MoveError> {
        self.move_in_direction(&Direction::DownRight)
    }

    pub fn ups(&self) -> Vec<Square> {
        self.search_in_direction(Direction::Up)
    }

    pub fn downs(&self) -> Vec<Square> {
        self.search_in_direction(Direction::Down)
    }

    pub fn lefts(&self) -> Vec<Square> {
        self.search_in_direction(Direction::Left)
    }

    pub fn rights(&self) -> Vec<Square> {
        self.search_in_direction(Direction::Right)
    }

    pub fn up_lefts(&self) -> Vec<Square> {
        self.search_in_direction(Direction::UpLeft)
    }

    pub fn up_rights(&self) -> Vec<Square> {
        self.search_in_direction(Direction::UpRight)
    }

    pub fn down_lefts(&self) -> Vec<Square> {
        self.search_in_direction(Direction::DownLeft)
    }

    pub fn down_rights(&self) -> Vec<Square> {
        self.search_in_direction(Direction::DownRight)
    }

    pub fn laterals(&self) -> Vec<Square> {
        let mut laterals: Vec<Square> = Vec::new();
        laterals.extend(self.ups());
        laterals.extend(self.downs());
        laterals.extend(self.lefts());
        laterals.extend(self.rights());
        laterals
    }

    pub fn diagonals(&self) -> Vec<Square> {
        let mut diagonals: Vec<Square> = Vec::new();
        diagonals.extend(self.up_lefts());
        diagonals.extend(self.up_rights());
        diagonals.extend(self.down_lefts());
        diagonals.extend(self.down_rights());
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

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub fn boundry_check(&self, square: &Square) -> bool {
        match self {
            Direction::Up => square.rank < 7,
            Direction::Down => square.rank > 0,
            Direction::Left => square.file > 0,
            Direction::Right => square.file < 7,
            Direction::UpLeft => square.rank < 7 && square.file > 0,
            Direction::UpRight => square.rank < 7 && square.file < 7,
            Direction::DownLeft => square.rank > 0 && square.file > 0,
            Direction::DownRight => square.rank > 0 && square.file < 7,
        }
    }

    pub fn move_in_direction(&self, square: &Square) -> Square {
        match self {
            Direction::Up => Square {
                file: square.file,
                rank: square.rank + 1,
            },
            Direction::Down => Square {
                file: square.file,
                rank: square.rank - 1,
            },
            Direction::Left => Square {
                file: square.file - 1,
                rank: square.rank,
            },
            Direction::Right => Square {
                file: square.file + 1,
                rank: square.rank,
            },
            Direction::UpLeft => Square {
                file: square.file - 1,
                rank: square.rank + 1,
            },
            Direction::UpRight => Square {
                file: square.file + 1,
                rank: square.rank + 1,
            },
            Direction::DownLeft => Square {
                file: square.file - 1,
                rank: square.rank - 1,
            },
            Direction::DownRight => Square {
                file: square.file + 1,
                rank: square.rank - 1,
            },
        }
    }
}

const FILE_LETTERS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
}

impl Move {
    pub fn from_standard_notation(
        notation: &str,
        board: &Board,
        side: Color,
    ) -> Result<Move, StandardNotationParseError> {
        let piece_standard_notation: HashMap<char, PieceType> = HashMap::from([
            ('R', PieceType::Rook),
            ('N', PieceType::Knight),
            ('B', PieceType::Bishop),
            ('Q', PieceType::Queen),
            ('K', PieceType::King),
        ]);

        let is_capture = notation.contains('x');

        let piece_type: PieceType = {
            let first_char = notation.chars().next().unwrap();
            if first_char.is_ascii_uppercase() {
                match piece_standard_notation.get(&first_char) {
                    Some(&piece_type) => Ok(piece_type),
                    None => Err(StandardNotationParseError::InvalidPieceType(first_char)),
                }
            } else if FILE_LETTERS.contains(&first_char) {
                Ok(PieceType::Pawn)
            } else {
                Err(StandardNotationParseError::InvalidPieceType(first_char))
            }
        }?;

        let piece = Piece {
            piece_type,
            color: side,
        };

        let potential_pieces = board.find_piece(piece);

        if potential_pieces.is_empty() {
            return Err(StandardNotationParseError::PieceNotFound(piece));
        }

        let destination: Square = {
            let destination_indicater = notation[(notation.len() - 2)..].chars();
            let file = destination_indicater.clone().next().unwrap() as u8 - 'a' as u8;
            let rank = destination_indicater.last().unwrap().to_digit(10).unwrap() as u8 - 1;
            Square { file, rank }
        };

        for piece_square in potential_pieces {
            let mv = Move {
                from: piece_square,
                to: destination,
            };
            if !is_capture {
                if piece.is_valid_move(&mv, &board.matrix()) {
                    // todo!("did not handle multiple pieces targetting the same square");
                    return Ok(mv);
                }
            } else {
                if piece.is_valid_capture_move(&mv, &board.matrix()) {
                    return Ok(mv);
                }
            }
        }

        return Err(StandardNotationParseError::InvalidDestination);
    }
}

#[derive(Debug)]
pub struct MoveError;

#[derive(Debug)]
pub enum StandardNotationParseError {
    InvalidPieceType(char),
    InvalidDestination,
    PieceNotFound(Piece),
}
