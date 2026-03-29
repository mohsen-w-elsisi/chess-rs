mod board;
mod piece;
mod square;
mod visualise;

use board::Board;
use square::Square;
use crate::{piece::Color, square::Move, visualise::visualise_as_ascii};

fn main() {
    let mut board = Board::initial();
    let mut vis = String::new();
    let mut current_color = Color::White;

    loop {
        // get input string from user as move in standard chess notation
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        if input == "exit" {
            break;
        }

        // parse the input and apply move
        let mv = Move::from_standard_notation(input, &board, current_color);
        if let Some(mv) = mv {
            board.apply_move(&mv);
            vis = visualise_as_ascii(&board);
            println!("{}", vis);
            current_color = match current_color {
                Color::White => Color::Black,
                Color::Black => Color::White,
            };
        } else {
            println!("Invalid move, please try again.");
        }
    }
}
