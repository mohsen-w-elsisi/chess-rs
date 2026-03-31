use crate::{
    r#move::{CastleSide, Move},
    piece::{Color, Piece, PieceType},
    piece_matrix::PieceMatrix,
    square::{FILE_LETTERS, Square},
};

pub fn from_standard_notation(
    notation: &str,
    board: &PieceMatrix,
    color: &Color,
) -> Result<Move, StandardNotationParseError> {
    if notation.contains('O') {
        return parse_castling_notation(notation, color);
    }

    let is_capture = notation.contains('x');

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

    let destination: Square = parse_destination_square(notation)?;

    for piece_square in potential_pieces {
        if is_capture {
            if piece.is_valid_capture_move(&piece_square, &destination, &board) {
                return Ok(Move::Capture {
                    from: piece_square,
                    to: destination,
                });
            }
        } else {
            if piece.is_valid_move(&piece_square, &destination, &board) {
                return Ok(Move::Normal {
                    from: piece_square,
                    to: destination,
                });
            }
        }
    }

    return Err(StandardNotationParseError::InvalidDestination);
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

fn parse_destination_square(notation: &str) -> Result<Square, StandardNotationParseError> {
    let indicater_chars = &notation[notation.len() - 2..];
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
}
