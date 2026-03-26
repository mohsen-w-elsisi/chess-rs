mod board;
mod piece;
mod square;
mod visualise;

use board::Board;
use square::Square;
use crate::visualise::visualise_as_ascii;

fn main() {
    let mut board = Board::initial();
    board.apply_move(&square::Move {
        from: Square { file: 4, rank: 1 },
        to: Square { file: 4, rank: 3 },
    });

    let vis = visualise_as_ascii(&board);

    println!("{}", vis);
}
