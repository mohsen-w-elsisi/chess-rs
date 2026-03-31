use crate::{
    board::Board,
    piece::{Color, Piece, PieceType},
    square::{FILE_LETTERS, Move, Square},
};

pub fn from_standard_notation(
    notation: &str,
    board: &Board,
    color: &Color,
) -> Result<Move, StandardNotationParseError> {
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
        let mv = Move {
            from: piece_square,
            to: destination,
        };
        if !is_capture {
            if piece.is_valid_move(&mv, &board.matrix()) {
                // todo!("did not handle multiple pieces targetting the same square");
                return Ok(mv);
            }
        } else {
            if piece.is_valid_capture_move(&mv, &board.matrix()) {
                return Ok(mv);
            }
        }
    }

    return Err(StandardNotationParseError::InvalidDestination);
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
}
