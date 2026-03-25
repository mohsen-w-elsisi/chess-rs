mod board;
mod piece;
mod square;

use board::Board;
use square::Square;

fn main() {
    let mut board = Board::initial();
    board.apply_move(&square::Move {
        from: Square { file: 0, rank: 1 },
        to: Square { file: 0, rank: 3 },
    });

    println!("{:?}", board.get_piece(&Square { file: 0, rank: 3 }));
}
