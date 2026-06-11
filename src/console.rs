use crate::{
    board::Board,
    game::{Player, Visualiser},
    r#move,
    piece::{Color, PieceType},
    standard_notation::from_standard_notation,
};

pub struct ConsolePlayer {
    color: Color,
}

impl Player for ConsolePlayer {
    fn get_move(&self, board: &Board) -> r#move::Move {
        let mut input = self.read_notation_from_console();
        loop {
            match from_standard_notation(input.as_str(), board, &self.color) {
                Ok(mv) => return mv,
                Err(error) => {
                    println!("{:?}", error);
                    input.clear();
                    input = self.read_notation_from_console();
                }
            }
        }
    }
}

impl ConsolePlayer {
    pub fn new(color: Color) -> ConsolePlayer {
        ConsolePlayer { color }
    }
    
    fn read_notation_from_console(&self) -> String {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().to_string()
    }
}

pub struct ConsoleVisualiser;

impl Visualiser for ConsoleVisualiser {
    fn visualise(&self, board: &Board) {
        let visualisation = visualise_as_ascii(board);
        println!("{}", visualisation);
    }
}

fn visualise_as_ascii(board: &Board) -> String {
    let mut output = vec![' '; 64];

    for (mv, piece) in board.get_pieces() {
        let piece_char = {
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

            piece_char
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
