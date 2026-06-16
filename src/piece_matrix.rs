use crate::{
    piece::{Color, Piece, PieceType},
    square::{Direction, Square},
};

#[derive(Clone)]
pub struct PieceMatrix {
    matrix: [[Option<Piece>; 8]; 8],
}

impl PieceMatrix {
    pub fn new() -> PieceMatrix {
        PieceMatrix {
            matrix: [[None; 8]; 8],
        }
    }

    pub fn initial() -> PieceMatrix {
        let mut piece_matrix = PieceMatrix::new();

        // Place pawns
        for file in 0..8 {
            piece_matrix.matrix[1][file] = Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::White,
            });
            piece_matrix.matrix[6][file] = Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::Black,
            });
        }

        // Place other pieces
        let back_rank = [
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];

        for (file, &piece_type) in back_rank.iter().enumerate() {
            piece_matrix.matrix[0][file] = Some(Piece {
                piece_type,
                color: Color::White,
            });
            piece_matrix.matrix[7][file] = Some(Piece {
                piece_type,
                color: Color::Black,
            });
        }

        piece_matrix
    }

    pub fn get_pieces(&self) -> Vec<(Square, Piece)> {
        let mut pieces_vec: Vec<(Square, Piece)> = Vec::new();
        for rank in 0..8 {
            for file in 0..8 {
                if let Some(piece) = self.matrix[rank][file] {
                    pieces_vec.push((
                        Square {
                            rank: rank as u8,
                            file: file as u8,
                        },
                        piece,
                    ));
                }
            }
        }
        pieces_vec
    }

    pub fn get_piece(&self, square: &Square) -> Option<Piece> {
        self.matrix[square.rank as usize][square.file as usize]
    }

    pub fn place_piece(&mut self, square: &Square, piece: Piece) {
        self.matrix[square.rank as usize][square.file as usize] = Some(piece);
    }

    pub fn remove_piece(&mut self, square: &Square) {
        self.matrix[square.rank as usize][square.file as usize] = None;
    }

    pub fn find_piece(&self, piece: Piece) -> Vec<Square> {
        let mut squares = Vec::new();
        for rank in 0..8 {
            for file in 0..8 {
                if let Some(p) = self.matrix[rank][file] {
                    if p == piece {
                        squares.push(Square {
                            rank: rank as u8,
                            file: file as u8,
                        });
                    }
                }
            }
        }
        squares
    }

    pub fn is_occupied(&self, square: &Square) -> bool {
        self.get_piece(square).is_some()
    }
    
    pub fn is_occupied_by_color(&self, square: &Square, color: Color) -> bool {
        if let Some(piece) = self.get_piece(square) {
            piece.color == color
        } else {
            false
        }
    }

    pub fn squares_until_blocked(&self, start: &Square, direction: &Direction) -> Vec<Square> {
        let mut squares: Vec<Square> = Vec::new();
        let mut current_square = start.move_in_direction(direction);

        while let Ok(s) = current_square {
            if self.get_piece(&s).is_some() {
                break;
            }
            squares.push(s);
            current_square = s.move_in_direction(direction);
        }

        squares
    }

    pub fn first_occupied_square(&self, start: &Square, direction: &Direction) -> Option<Square> {
        let unoccupied_squares = self.squares_until_blocked(start, direction);
        let last_unoccupied_square = unoccupied_squares.last().unwrap_or(start);
        let candidate = last_unoccupied_square.move_in_direction(direction).ok()?;

        Some(candidate)
    }

    pub fn first_enemy_piece_square(
        &self,
        start: &Square,
        direction: &Direction,
        color: Color,
    ) -> Option<Square> {
        let first_occupied_square = self.first_occupied_square(start, direction)?;
        let occupant_piece = self.get_piece(&first_occupied_square).unwrap();

        if occupant_piece.color != color {
            Some(first_occupied_square)
        } else {
            None
        }
    }
}
