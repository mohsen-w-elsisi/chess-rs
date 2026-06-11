use crate::{
    board::Board,
    r#move::{CastleSide, Move},
    piece::{Color, Piece, PieceType, pawn},
    square::{FILE_LETTERS, Square, file_letter_to_index},
};

pub fn from_standard_notation(
    notation: &str,
    board: &Board,
    color: &Color,
) -> Result<Move, StandardNotationParseError> {
    if notation.contains('O') {
        return parse_castling_notation(notation, color);
    }

    let is_capture = notation.contains('x');
    
    if notation.contains('=') {
        return parse_promotion_notation(notation, color, is_capture);
    }

    let is_en_passent = notation.ends_with("e.p.") && is_capture;


    let piece_type = {
        let first_char = notation.chars().next().unwrap();
        piece_type_from_char(first_char)?
    };

    let piece = Piece {
        piece_type,
        color: *color,
    };

    let potential_pieces = board.find_piece(piece);

    if potential_pieces.is_empty() {
        return Err(StandardNotationParseError::PieceNotFound(piece));
    }

    let destination: Square = parse_destination_square(notation, &piece_type, is_capture)?;


    for piece_square in potential_pieces {
        if is_en_passent {
            if pawn::is_valid_en_passent(&piece_square, &destination, &board.history()).is_ok() {
                return Ok(Move::EnPassent {
                    from: piece_square,
                    to: destination,
                });
            }
        } else if is_capture {
            if piece.is_valid_capture_move(&piece_square, &destination, &board.matrix()) {
                return Ok(Move::Capture {
                    from: piece_square,
                    to: destination,
                });
            }
        } else {
            if piece.is_valid_move(&piece_square, &destination, &board.matrix()) {
                return Ok(Move::Normal {
                    from: piece_square,
                    to: destination,
                });
            }
        }
    }

    return Err(StandardNotationParseError::InvalidDestination);
}

fn parse_promotion_notation(
    notation: &str,
    color: &Color,
    is_capture: bool,
) -> Result<Move, StandardNotationParseError> {
    let start_square = {
        let first_char = notation.chars().next().unwrap();
        let file = file_letter_to_index(first_char)
            .ok_or(StandardNotationParseError::InvalidFileIndicator(first_char))?;
        let rank: u8 = match color {
            Color::White => 6,
            Color::Black => 1,
        };
        Square { file, rank }
    };
    let promotion_square = {
        let promotion_rank = match color {
            Color::White => 7,
            Color::Black => 0,
        };

        let promotion_file = if !is_capture {
            start_square.file
        } else {
            let promotion_file_char = notation.chars().nth(2).unwrap();
            file_letter_to_index(promotion_file_char).ok_or(
                StandardNotationParseError::InvalidFileIndicator(promotion_file_char),
            )?
        };

        Square {
            file: promotion_file,
            rank: promotion_rank,
        }
    };
    let promotion_piece_type = {
        let promotion_char = notation.chars().last().unwrap();
        piece_type_from_char(promotion_char)?
    };
    return Ok(Move::Promotion {
        from: start_square,
        to: promotion_square,
        capture: is_capture,
        promotion_piece_type,
    });
}

fn parse_castling_notation(
    notation: &str,
    color: &Color,
) -> Result<Move, StandardNotationParseError> {
    let castle_side = match notation {
        "O-O" => Ok(CastleSide::KingSide),
        "O-O-O" => Ok(CastleSide::QueenSide),
        _ => Err(StandardNotationParseError::InvalidCastlingNotation),
    }?;
    Ok(Move::Castle {
        color: *color,
        side: castle_side,
    })
}

fn piece_type_from_char(c: char) -> Result<PieceType, StandardNotationParseError> {
    match c {
        'R' => Ok(PieceType::Rook),
        'N' => Ok(PieceType::Knight),
        'B' => Ok(PieceType::Bishop),
        'Q' => Ok(PieceType::Queen),
        'K' => Ok(PieceType::King),
        _ if FILE_LETTERS.contains(&c) => Ok(PieceType::Pawn),
        _ => Err(StandardNotationParseError::InvalidPieceType(c)),
    }
}

fn parse_destination_square(notation: &str, piece_type: &PieceType, is_capture: bool) -> Result<Square, StandardNotationParseError> {
    let indicater_chars = match piece_type {
        _ if is_capture => &notation[2..4],
        PieceType::Pawn => &notation[0..2],
        _ => &notation[1..3],
    };
    let err = StandardNotationParseError::InvalidDestinationIndicator(indicater_chars.to_string());
    if indicater_chars.len() != 2 {
        return Err(err);
    }
    let file_char = indicater_chars.chars().nth(0).unwrap();
    if !FILE_LETTERS.contains(&file_char) {
        return Err(err);
    }
    let rank_char = indicater_chars.chars().nth(1).unwrap();
    let file = file_char as u8 - 'a' as u8;
    let rank = rank_char.to_digit(10).ok_or(err)? as u8 - 1;
    Ok(Square { file, rank })
}

#[derive(Debug)]
pub enum StandardNotationParseError {
    InvalidPieceType(char),
    InvalidDestinationIndicator(String),
    InvalidDestination,
    PieceNotFound(Piece),
    InvalidCastlingNotation,
    InvalidFileIndicator(char),
}
