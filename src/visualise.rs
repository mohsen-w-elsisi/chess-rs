use std::iter;

use crate::{
    board::Board,
    piece::{self, Color, PieceType},
};

pub fn visualise_as_ascii(board: &Board) -> String {
    let mut output = vec![' '; 64];

    for (mv, piece) in board.get_pieces() {
        let mut piece_char = match piece.piece_type {
            PieceType::Pawn => 'P',
            PieceType::Rook => 'R',
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
        };

        piece_char = match piece.color {
            Color::White => piece_char,
            Color::Black => piece_char.to_ascii_lowercase(),
        };

        output[mv.to_flat_index() as usize] = piece_char;
    }

    let mut result = String::new();
    for i in 0..8 {
        let rank_chars = &output[((7 - i) * 8)..((7 - i + 1) * 8)];
        result.push_str(&rank_chars.iter().collect::<String>());
        result.push('\n');
    }

    result
}
