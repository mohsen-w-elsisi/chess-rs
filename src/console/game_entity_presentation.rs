use crate::{
    game::GameResult,
    r#move::{CastleSide, Move},
    piece::PieceType,
    square::Square,
};

impl PieceType {
    pub fn from_char(c: char) -> Option<PieceType> {
        match c.to_ascii_uppercase() {
            'P' => Some(PieceType::Pawn),
            'N' => Some(PieceType::Knight),
            'B' => Some(PieceType::Bishop),
            'R' => Some(PieceType::Rook),
            'Q' => Some(PieceType::Queen),
            'K' => Some(PieceType::King),
            _ => None,
        }
    }

    pub fn to_char(&self) -> &str {
        match self {
            PieceType::Pawn => "",
            PieceType::Knight => "N",
            PieceType::Bishop => "B",
            PieceType::Rook => "R",
            PieceType::Queen => "Q",
            PieceType::King => "K",
        }
    }

    pub fn to_char_with_pawn(&self) -> char {
        match self {
            PieceType::Pawn => 'P',
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Rook => 'R',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
        }
    }
}

impl Move {
    pub fn to_standard_notation_suffix(&self) -> String {
        match self {
            Move::Normal { from: _, to } => format!("{}", to.to_string()),
            Move::Capture { from: _, to } => format!("x{}", to.to_string()),

            Move::Castle {
                side: CastleSide::KingSide,
                color: _,
            } => "O-O".to_string(),
            Move::Castle {
                side: CastleSide::QueenSide,
                color: _,
            } => "O-O-O".to_string(),

            Move::EnPassent { from, to } => {
                format!("{}x{} e.p.", from.file_to_char(), to.to_string())
            }

            Move::Promotion {
                from,
                to,
                capture,
                promotion_piece_type,
            } => {
                let promotion_suffix = format!("={}", promotion_piece_type.to_char());
                let movement_indicator = if *capture {
                    format!("{}x{}", from.file_to_char(), to.to_string())
                } else {
                    format!("{}", to.to_string())
                };
                format!("{}{}", movement_indicator, promotion_suffix)
            }
        }
    }
}

impl Square {
    pub fn to_string(&self) -> String {
        let file_char = (b'a' + self.file as u8) as char;
        let rank_char = (b'1' + self.rank as u8) as char;
        format!("{}{}", file_char, rank_char)
    }

    pub fn file_to_char(&self) -> char {
        (b'a' + self.file as u8) as char
    }
}

impl GameResult {
    pub fn to_string(&self) -> &str {
        match self {
            GameResult::WhiteWins => "1-0",
            GameResult::BlackWins => "0-1",
            GameResult::Draw => "1/2-1/2",
        }
    }
}
