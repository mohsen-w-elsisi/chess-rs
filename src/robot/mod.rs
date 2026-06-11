use crate::board::Board;
use crate::game::Player;
use crate::r#move::Move;
use crate::piece::Color;

pub struct RobotPlayer {
    color: crate::piece::Color,
}

impl RobotPlayer {
    pub fn new(color: Color) -> RobotPlayer {
        RobotPlayer { color }
    }
}

impl Player for RobotPlayer {
    fn get_move(&self, board: &Board) -> Move {
        return get_available_moves(board, self.color)[0];
    }
}

fn get_available_moves(board: &Board, color: Color) -> Vec<Move> {
    let mut available_moves: Vec<Move> = Vec::new();

    let pieces = board.get_pieces()
        .into_iter()
        .filter(|piece_info| {
            piece_info.1.color == color
        })
        .collect::<Vec<_>>();

    for (square, piece) in pieces {
        let moves = piece.get_available_moves(&square, &board.matrix());
        available_moves.extend(moves);
    }

    return available_moves;
}
