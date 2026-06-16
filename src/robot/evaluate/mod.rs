use crate::piece::{Color, Piece};
use crate::piece_matrix::PieceMatrix;

const MATERIAL_WIEGHT: f64 = 1.0;

pub fn evaluate_position(board: &PieceMatrix, color: Color) -> f64 {
    Evaluater { board, color }.evaluate()
}

pub struct Evaluater<'a> {
    board: &'a PieceMatrix,
    color: Color,
}

impl Evaluater<'_> {
    pub fn evaluate(&self) -> f64 {
        return self.material_score() * MATERIAL_WIEGHT as f64;
    }

    // normalised material score
    fn material_score(&self) -> f64 {
        let mut my_pieces: Vec<Piece> = vec![];
        let mut opponent_pieces: Vec<Piece> = vec![];

        for (_, piece) in self.board.get_pieces() {
            if piece.color == self.color {
                my_pieces.push(piece);
            } else {
                opponent_pieces.push(piece);
            }
        }

        let my_score: i32 = my_pieces.iter().map(|p| p.piece_type.value()).sum();
        let opponent_score: i32 = opponent_pieces.iter().map(|p| p.piece_type.value()).sum();

        return (my_score - opponent_score) as f64 / (my_score + opponent_score) as f64;
    }
}
