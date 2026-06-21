use crate::{
    board::Board,
    piece::{Color, Piece, PieceType},
    piece_matrix::PieceMatrix,
    square::Square,
};

impl Board {
    pub fn from_fen(fen: &str) -> Self {
        let mut matrix = PieceMatrix::new();
        let mut flat_index = 0 as u8;

        for c in fen.chars() {
            if c == '/' {
                continue;
            } else if c.is_digit(10) {
                flat_index += c.to_digit(10).unwrap() as u8;
            } else {
                let piece = Piece {
                    piece_type: PieceType::from_char(c).unwrap(),
                    color: if c.is_uppercase() {
                        Color::White
                    } else {
                        Color::Black
                    },
                };
                matrix.place_piece(&Square::from_flat_index(flat_index), piece);
                flat_index += 1;
            }
        }

        Board::custom(matrix, Vec::new())
    }
}
