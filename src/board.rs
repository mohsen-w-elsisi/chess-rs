use std::collections::HashMap;

use crate::piece::{Color, Piece, PieceType};
use crate::square::{Move, Square};

pub struct Board {
    pieces: [Option<Piece>; 64],
}

impl Board {
    pub fn empty() -> Board {
        Board { pieces: [None; 64] }
    }

    pub fn initial() -> Board {
        Board {
            pieces: INITIAL_BOARD_POSITION.clone(),
        }
    }

    pub fn get_pieces(&self)-> Vec<(Square, Piece)> {
        let mut pieces_vec: Vec<(Square, Piece)> = Vec::new();
        for (i, piece) in self.pieces.iter().enumerate() {
            if let Some(p) = piece {
                pieces_vec.push((Square::from_flat_index(i as u8), *p));
            }
        }        
        pieces_vec
    }
    
    pub fn get_piece(&self, square: &Square) -> Option<Piece> {
        self.pieces[square.to_flat_index() as usize]
    }

    pub fn place_piece(&mut self, square: &Square, piece: Piece) {
        self.pieces[square.to_flat_index() as usize] = Some(piece);
    }

    pub fn remove_piece(&mut self, square: &Square) -> Option<Piece> {
        let piece = self.get_piece(square);
        self.pieces[square.to_flat_index() as usize] = None;
        piece
    }

    pub fn apply_move(&mut self, mv: &Move) {
        let piece = self.get_piece(&mv.from);
        if let Some(p) = piece {
            self.remove_piece(&mv.from);
            self.place_piece(&mv.to, p);
        }
    }

    pub fn find_piece(&self, piece: Piece) -> Vec<Square> {
        let mut squares = Vec::new();
        for (i, p) in self.pieces.iter().enumerate() {
            if let Some(_piece) = p {
                if *_piece == piece {
                    squares.push(Square::from_flat_index(i as u8));
                }
            }
        }
        squares
    }
}

const INITIAL_BOARD_POSITION: [Option<Piece>; 64] = {
    const WHITE_PAWN: Piece = Piece {
        piece_type: PieceType::Pawn,
        color: Color::White,
    };
    const BLACK_PAWN: Piece = Piece {
        piece_type: PieceType::Pawn,
        color: Color::Black,
    };
    const WHITE_ROOK: Piece = Piece {
        piece_type: PieceType::Rook,
        color: Color::White,
    };
    const BLACK_ROOK: Piece = Piece {
        piece_type: PieceType::Rook,
        color: Color::Black,
    };
    const WHITE_KNIGHT: Piece = Piece {
        piece_type: PieceType::Knight,
        color: Color::White,
    };
    const BLACK_KNIGHT: Piece = Piece {
        piece_type: PieceType::Knight,
        color: Color::Black,
    };
    const WHITE_BISHOP: Piece = Piece {
        piece_type: PieceType::Bishop,
        color: Color::White,
    };
    const BLACK_BISHOP: Piece = Piece {
        piece_type: PieceType::Bishop,
        color: Color::Black,
    };
    const WHITE_QUEEN: Piece = Piece {
        piece_type: PieceType::Queen,
        color: Color::White,
    };
    const BLACK_QUEEN: Piece = Piece {
        piece_type: PieceType::Queen,
        color: Color::Black,
    };
    const WHITE_KING: Piece = Piece {
        piece_type: PieceType::King,
        color: Color::White,
    };
    const BLACK_KING: Piece = Piece {
        piece_type: PieceType::King,
        color: Color::Black,
    };

    let mut pieces: [Option<Piece>; 64] = [None; 64];

    pieces[8] = Some(WHITE_PAWN);
    pieces[9] = Some(WHITE_PAWN);
    pieces[10] = Some(WHITE_PAWN);
    pieces[11] = Some(WHITE_PAWN);
    pieces[12] = Some(WHITE_PAWN);
    pieces[13] = Some(WHITE_PAWN);
    pieces[14] = Some(WHITE_PAWN);
    pieces[15] = Some(WHITE_PAWN);

    pieces[48] = Some(BLACK_PAWN);
    pieces[49] = Some(BLACK_PAWN);
    pieces[50] = Some(BLACK_PAWN);
    pieces[51] = Some(BLACK_PAWN);
    pieces[52] = Some(BLACK_PAWN);
    pieces[53] = Some(BLACK_PAWN);
    pieces[54] = Some(BLACK_PAWN);
    pieces[55] = Some(BLACK_PAWN);

    pieces[0] = Some(WHITE_ROOK);
    pieces[1] = Some(WHITE_KNIGHT);
    pieces[2] = Some(WHITE_BISHOP);
    pieces[3] = Some(WHITE_QUEEN);
    pieces[4] = Some(WHITE_KING);
    pieces[5] = Some(WHITE_BISHOP);
    pieces[6] = Some(WHITE_KNIGHT);
    pieces[7] = Some(WHITE_ROOK);

    pieces[56] = Some(BLACK_ROOK);
    pieces[63] = Some(BLACK_ROOK);
    pieces[57] = Some(BLACK_KNIGHT);
    pieces[62] = Some(BLACK_KNIGHT);
    pieces[58] = Some(BLACK_BISHOP);
    pieces[61] = Some(BLACK_BISHOP);
    pieces[59] = Some(BLACK_QUEEN);
    pieces[60] = Some(BLACK_KING);

    pieces
};
