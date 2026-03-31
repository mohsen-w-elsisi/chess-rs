use crate::r#move::CastleSide;
use crate::piece::{Color, Piece, PieceType};
use crate::piece_matrix::PieceMatrix;
use crate::square::Direction;
use crate::{r#move::Move, square::Square};

pub struct Board {
    matrix: PieceMatrix,
    histroy: Vec<(Move, Piece)>,
}

impl Board {
    pub fn initial() -> Board {
        Board {
            matrix: PieceMatrix::initial(),
            histroy: Vec::new(),
        }
    }

    pub fn matrix(&self) -> PieceMatrix {
        self.matrix.clone()
    }

    pub fn apply_move(&mut self, mv: &Move) -> Result<(), String> {
        match mv {
            Move::Normal { from, to } => {
                let piece = self
                    .get_piece(from)
                    .ok_or("No piece at source square".to_string())?;
                if !piece.is_valid_move(from, to, &self.matrix) {
                    return Err("Invalid move".to_string());
                }
                self.remove_piece(from);
                self.place_piece(to, piece);
                self.histroy.push((*mv, piece));
                Ok(())
            }

            Move::Capture { from, to } => {
                let piece = self
                    .get_piece(from)
                    .ok_or("No piece at source square".to_string())?;
                if !piece.is_valid_capture_move(from, to, &self.matrix) {
                    return Err("Invalid capture move".to_string());
                }
                let captured_piece = self
                    .get_piece(to)
                    .ok_or("No piece to capture at destination square".to_string())?;
                if captured_piece.color == piece.color {
                    return Err("Cannot capture your own piece".to_string());
                }
                self.remove_piece(to);
                self.remove_piece(from);
                self.place_piece(to, piece);
                self.histroy.push((*mv, piece));
                Ok(())
            }

            Move::Castle { side, color } => {
                let king = Piece {
                    piece_type: PieceType::King,
                    color: *color,
                };

                let rook = Piece {
                    piece_type: PieceType::Rook,
                    color: *color,
                };

                let king_home_square = match color {
                    Color::White => Square { rank: 0, file: 4 },
                    Color::Black => Square { rank: 7, file: 4 },
                };

                let rook_home_square = match (side, color) {
                    (CastleSide::KingSide, Color::White) => Square { rank: 0, file: 7 },
                    (CastleSide::QueenSide, Color::White) => Square { rank: 0, file: 0 },
                    (CastleSide::KingSide, Color::Black) => Square { rank: 7, file: 7 },
                    (CastleSide::QueenSide, Color::Black) => Square { rank: 7, file: 0 },
                };

                let castling_direction = match side {
                    CastleSide::KingSide => Direction::Right,
                    CastleSide::QueenSide => Direction::Left,
                };

                if self.get_piece(&king_home_square) != Some(king) {
                    return Err("King is not on the home square".to_string());
                }

                if self.get_piece(&rook_home_square) != Some(rook) {
                    return Err("Rook is not on the home square".to_string());
                }

                for (mv, piece) in &self.histroy {
                    if let Move::Normal { from, to } | Move::Capture { from, to } = mv {
                        if from == &rook_home_square || to == &rook_home_square {
                            return Err("Cannot castle after rook has moved or been captured".to_string());
                        }
                    }
                    if piece == &king {
                        return Err("Cannot castle after king has moved".to_string());
                    }
                }

                if self
                    .matrix
                    .first_occupied_square(&king_home_square, &castling_direction)
                    != Some(rook_home_square)
                {
                    return Err("Castling path is blocked".to_string());
                }

                let king_castled_square = Square {
                    rank: king_home_square.rank,
                    file: match side {
                        CastleSide::KingSide => 6,
                        CastleSide::QueenSide => 2,
                    },
                };

                let rook_castled_square = Square {
                    rank: rook_home_square.rank,
                    file: match side {
                        CastleSide::KingSide => 5,
                        CastleSide::QueenSide => 3,
                    },
                };

                self.remove_piece(&king_home_square);
                self.remove_piece(&rook_home_square);
                self.place_piece(&king_castled_square, king);
                self.place_piece(&rook_castled_square, rook);
                self.histroy.push((*mv, king));

                Ok(())
            }
        }
    }

    pub fn find_piece(&self, piece: Piece) -> Vec<Square> {
        self.matrix.find_piece(piece)
    }

    pub fn get_pieces(&self) -> Vec<(Square, Piece)> {
        self.matrix.get_pieces()
    }

    pub fn get_piece(&self, square: &Square) -> Option<Piece> {
        self.matrix.get_piece(square)
    }

    fn place_piece(&mut self, square: &Square, piece: Piece) {
        self.matrix.place_piece(square, piece);
    }

    fn remove_piece(&mut self, square: &Square) {
        self.matrix.remove_piece(square)
    }
}
