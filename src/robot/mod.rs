mod evaluate;

use crate::board::Board;
use crate::game::Player;
use crate::r#move::Move;
use crate::piece::Color;
use crate::piece_matrix::PieceMatrix;
use evaluate::evaluate_position;

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
        return get_best_move(&board, self.color);
    }
}

fn get_best_move(board: &Board, color: Color) -> Move {
    let available_moves = get_available_moves(&board.matrix(), color);
    let mut best_move = available_moves[0];
    let mut best_score: f64 = f64::MIN;

    for m in available_moves {
        let mut new_board = board.clone();
        new_board.apply_move(&m).unwrap();
        let score = evaluate_position(&new_board.matrix(), color);
        if score > best_score {
            best_score = score;
            best_move = m;
        }
    }

    return best_move;
}

fn get_available_moves(board: &PieceMatrix, color: Color) -> Vec<Move> {
    let pieces = board.get_pieces()
        .into_iter()
        .filter(|piece_info| {
            piece_info.1.color == color
        })
        .collect::<Vec<_>>();

    let available_moves = pieces.iter()
        .flat_map(|(square, piece)| {
            piece.get_available_moves(square, board)
        })
        .collect();

    return available_moves;
}
