mod castling;

use crate::{
    r#move::Move,
    piece::{Piece, PieceType, pawn},
    piece_matrix::PieceMatrix,
    square::Square,
};

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

    pub fn history(&self) -> &Vec<(Move, Piece)> {
        &self.histroy
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
                castling::perform_castling(side, color, self)?;
                self.histroy.push((
                    *mv,
                    Piece {
                        piece_type: PieceType::King,
                        color: *color,
                    },
                ));
                Ok(())
            }

            Move::Promotion {
                from,
                to,
                capture,
                promotion_piece_type,
            } => {
                pawn::is_valid_promotion(from, to, *capture, promotion_piece_type, &self.matrix)?;

                let original_pawn = self
                    .get_piece(from)
                    .ok_or("No pawn to promote".to_string())?;

                let promotion_piece = Piece {
                    piece_type: *promotion_piece_type,
                    color: original_pawn.color,
                };

                self.remove_piece(from);
                self.place_piece(to, promotion_piece);
                self.histroy.push((*mv, original_pawn));

                Ok(())
            }

            Move::EnPassent { from, to } => {
                pawn::is_valid_en_passent(from, to, &self.histroy)?;

                let moving_pawn = self
                    .get_piece(from)
                    .ok_or("No pawn to move for en passent".to_string())?;

                let captured_pawn_square = Square {
                    rank: from.rank,
                    file: to.file,
                };
                    
                self.remove_piece(&captured_pawn_square);
                self.remove_piece(from);
                self.place_piece(to, moving_pawn);

                self.histroy.push((*mv, moving_pawn));

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
