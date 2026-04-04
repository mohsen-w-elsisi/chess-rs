use crate::{
    board::Board,
    r#move::{CastleSide, Move},
    piece::{Color, Piece, PieceType},
    square::Square,
};

pub fn perform_castling(side: &CastleSide, color: &Color, board: &mut Board) -> Result<(), String> {
    let (king_piece, rook_piece) = get_pieces_for_color(&color);
    let (king_home_square, rook_home_square) = get_home_squares(&side, &color);
    let (king_castled_square, rook_castled_square) = get_castled_squares(&side, &color);

    validate_pieces_not_moved(&rook_home_square, &king_piece, &board.histroy)?;

    board.remove_piece(&king_home_square);
    board.remove_piece(&rook_home_square);
    board.place_piece(&king_castled_square, king_piece);
    board.place_piece(&rook_castled_square, rook_piece);

    Ok(())
}

fn validate_pieces_not_moved(
    rook_home_square: &Square,
    king_piece: &Piece,
    history: &Vec<(Move, Piece)>,
) -> Result<(), String> {
    for (mv, piece) in history {
        // checks if the rook has moved or been captured
        if let Move::Normal { from, to } | Move::Capture { from, to } = mv {
            if from == rook_home_square || to == rook_home_square {
                return Err("Cannot castle after rook has moved or been captured".to_string());
            }
        }

        // checks if king has moved. King cannot be captured
        if piece == king_piece {
            return Err("Cannot castle after king has moved".to_string());
        }
    }

    Ok(())
}

fn get_home_squares(castle_side: &CastleSide, color: &Color) -> (Square, Square) {
    let king_home_square = match color {
        Color::White => Square { rank: 0, file: 4 },
        Color::Black => Square { rank: 7, file: 4 },
    };

    let rook_home_square = match (castle_side, color) {
        (CastleSide::KingSide, Color::White) => Square { rank: 0, file: 7 },
        (CastleSide::QueenSide, Color::White) => Square { rank: 0, file: 0 },
        (CastleSide::KingSide, Color::Black) => Square { rank: 7, file: 7 },
        (CastleSide::QueenSide, Color::Black) => Square { rank: 7, file: 0 },
    };

    (king_home_square, rook_home_square)
}

fn get_castled_squares(side: &CastleSide, color: &Color) -> (Square, Square) {
    let king_castled_square = Square {
        rank: rank(color),
        file: match side {
            CastleSide::KingSide => 6,
            CastleSide::QueenSide => 2,
        },
    };

    let rook_castled_square = Square {
        rank: rank(color),
        file: match side {
            CastleSide::KingSide => 5,
            CastleSide::QueenSide => 3,
        },
    };

    (king_castled_square, rook_castled_square)
}

fn rank(color: &Color) -> u8 {
    match color {
        Color::White => 0,
        Color::Black => 7,
    }
}

fn get_pieces_for_color(color: &Color) -> (Piece, Piece) {
    (
        Piece {
            color: *color,
            piece_type: PieceType::King,
        },
        Piece {
            color: *color,
            piece_type: PieceType::Rook,
        },
    )
}
