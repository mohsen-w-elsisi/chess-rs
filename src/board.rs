use crate::piece::{Color, Piece, PieceType};
use crate::piece_matrix::{self, PieceMatrix};
use crate::square::{Move, Square};

pub struct Board {
    matrix: PieceMatrix,
}

impl Board {
    pub fn initial() -> Board {
        Board {
            matrix: PieceMatrix::initial(),
        }
    }

    pub fn matrix(&self) -> PieceMatrix {
        self.matrix.clone()
    }

    pub fn apply_move(&mut self, mv: &Move) -> Result<(), String> {
        let is_capture = self.get_piece(&mv.to).is_some();
        let piece = self.get_piece(&mv.from).expect("No piece at source square");

        if is_capture {
            if !piece.is_valid_capture_move(mv, &self.matrix) {
                return Err("Invalid capture move".to_string());
            }
            let captured_piece = self.get_piece(&mv.to).unwrap();
            if captured_piece.color == piece.color {
                return Err("Cannot capture your own piece".to_string());
            } else {
                self.remove_piece(&mv.to);
                self.remove_piece(&mv.from);
                self.place_piece(&mv.to, piece);
                Ok(())
            }
        } else {
            if !piece.is_valid_move(mv, &self.matrix) {
                return Err("Invalid move".to_string());
            }
            self.remove_piece(&mv.from);
            self.place_piece(&mv.to, piece);
            Ok(())
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
