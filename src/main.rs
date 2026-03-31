mod board;
mod piece;
mod piece_matrix;
mod square;
mod visualise;
mod standard_notation;

use crate::{piece::Color, standard_notation::from_standard_notation, visualise::visualise_as_ascii};
use board::Board;

fn main() {
    let mut board = Board::initial();
    let mut vis = visualise_as_ascii(&board);
    let mut current_color = Color::White;

    loop {
        // get input string from user as move in standard chess notation
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        if input == "exit" {
            break;
        }

        // parse the input and apply move
        let mv = from_standard_notation(input, &board, &current_color);
        match mv {
            Ok(mv) => {
                board.apply_move(&mv).unwrap();
                vis = visualise_as_ascii(&board);
                println!("{}", vis);
                current_color = match current_color {
                    Color::White => Color::Black,
                    Color::Black => Color::White,
                };
            }
            Err(error) => {
                println!("{:?}", error);
            }
        }
    }
}
