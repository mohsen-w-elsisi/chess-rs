use crate::{
    piece::{Color, Piece, PieceType},
    square::Square,
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
}
