mod castling;

use crate::{
    board::castling::CastlingError,
    r#move::Move,
    piece::{Color, Piece, PieceType, pawn},
    piece_matrix::PieceMatrix,
    square::Square,
};

#[derive(Clone)]
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

    pub fn apply_move(&mut self, mv: &Move, color: Color) -> Result<(), MoveApplicationError> {
        let mut temp_board = self.clone();
        temp_board.apply_move_without_legallity_validation(mv)?;

        if temp_board.is_check(color) {
            return Err(MoveApplicationError::KingInCheck);
        }

        self.matrix = temp_board.matrix;
        self.histroy = temp_board.histroy;
        Ok(())
    }

    fn apply_move_without_legallity_validation(
        &mut self,
        mv: &Move,
    ) -> Result<(), MoveApplicationError> {
        match mv {
            Move::Normal { from, to } => {
                let piece = self
                    .get_piece(from)
                    .ok_or(MoveApplicationError::PieceNotFound)?;

                if !piece.is_valid_move(from, to, &self.matrix) {
                    return Err(MoveApplicationError::InvalidMoveForPieceType);
                }
                self.remove_piece(from);
                self.place_piece(to, piece);
                self.histroy.push((*mv, piece));
                Ok(())
            }

            Move::Capture { from, to } => {
                let piece = self
                    .get_piece(from)
                    .ok_or(MoveApplicationError::PieceNotFound)?;

                if !piece.is_valid_capture_move(from, to, &self.matrix) {
                    return Err(MoveApplicationError::InvalidMoveForPieceType);
                }

                let captured_piece = self
                    .get_piece(to)
                    .ok_or(MoveApplicationError::PieceNotFound)?;

                if captured_piece.color == piece.color {
                    return Err(MoveApplicationError::AttemptCaptureOwnPiece);
                }

                self.remove_piece(to);
                self.remove_piece(from);
                self.place_piece(to, piece);
                self.histroy.push((*mv, piece));
                Ok(())
            }

            Move::Castle { side, color } => {
                castling::perform_castling(side, color, self)
                    .or_else(|e| Err(MoveApplicationError::Castling { error: e }))?;

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
                pawn::promotion::is_valid(from, to, *capture, promotion_piece_type, &self.matrix)
                    .or_else(|e| Err(MoveApplicationError::Promotion { error: e }))?;

                let original_pawn = self
                    .get_piece(from)
                    .ok_or(MoveApplicationError::PieceNotFound)?;

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
                pawn::en_passent::is_valid(from, to, &self.histroy)
                    .or_else(|e| Err(MoveApplicationError::EnPassent { error: e }))?;

                let moving_pawn = self
                    .get_piece(from)
                    .ok_or(MoveApplicationError::PieceNotFound)?;

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

    pub fn is_check(&self, color: Color) -> bool {
        let king_square = self.find_piece(Piece {
            piece_type: PieceType::King,
            color,
        })[0];

        let enemy_pieces = self
            .get_pieces()
            .into_iter()
            .filter(|(_, piece)| piece.color != color)
            .collect::<Vec<_>>();

        for (square, piece) in enemy_pieces {
            let capture_moves = piece.valid_capture_destinations(&square, &self.matrix);
            if capture_moves.contains(&king_square) {
                return true;
            }
        }

        false
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

#[derive(Debug)]
pub enum MoveApplicationError {
    KingInCheck,
    PieceNotFound,
    InvalidMoveForPieceType,
    AttemptCaptureOwnPiece,
    Castling { error: CastlingError },
    Promotion { error: pawn::promotion::Error },
    EnPassent { error: pawn::en_passent::Error },
}
