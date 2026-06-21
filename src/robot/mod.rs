mod deep_search;
mod evaluate;

use crate::board::{Board, MoveApplicationError};
use crate::game::Player;
use crate::r#move::Move;
use crate::piece::Color;
use crate::piece_matrix::PieceMatrix;
use crate::robot::deep_search::DeepEvaluator;
use evaluate::Evaluater;

pub struct RobotPlayer {
    color: crate::piece::Color,
    deep_evaluator: DeepEvaluator,
}

impl RobotPlayer {
    pub fn new(color: Color) -> RobotPlayer {
        RobotPlayer {
            color,
            deep_evaluator: DeepEvaluator::new(create_evaluater(), 2, color),
        }
    }

    fn get_best_move(&self, board: &Board, color: Color) -> Move {
        let available_moves = get_available_moves(&board.matrix(), color);

        let mut best_move = available_moves[0];
        let mut best_score: f64 = f64::MIN;

        for m in available_moves {
            let mut new_board = board.clone();

            match new_board.apply_move(&m, color) {
                Ok(_) => {}
                Err(MoveApplicationError::KingInCheck) => continue,
                Err(e) => unreachable!("Unexpected error while applying move: {:?}", e),
            }

            let score = self.deep_evaluator.evaluate(&new_board);
            if score > best_score {
                best_score = score;
                best_move = m;
            }
        }

        println!("\n current Evaluation: {}", best_score);

        return best_move;
    }
}

impl Player for RobotPlayer {
    fn get_move(&self, board: &Board) -> Move {
        return self.get_best_move(&board, self.color);
    }

    fn name(&self) -> String {
        "Robot".to_string()
    }
}

fn get_available_moves(board: &PieceMatrix, color: Color) -> Vec<Move> {
    return board
        .get_pieces()
        .into_iter()
        .filter(|piece_info| piece_info.1.color == color)
        .flat_map(|(square, piece)| piece.get_available_moves(&square, board))
        .collect();
}

fn create_evaluater() -> Evaluater {
    let material_evaluator =
        evaluate::material::MaterialEvaluator::new(evaluate::material::MaterialEvaluationConfig {
            piece_values: evaluate::MaterialValues::default(),
        });

    let central_control_evaluator = evaluate::central_control::PieceActivityEvaluator::new(
        evaluate::central_control::PieceActivityEvaluationConfig {
            opponent_piece_threat_weight: 0.7,
            flat_threat_weight: 0.5,
            central_control_weight: 1.0,
            semi_central_control_weight: 0.5,
            semi_edge_control_weight: 0.25,
            edge_control_weight: 0.15,
        },
    );

    Evaluater::new(vec![
        (1.5, Box::new(material_evaluator)),
        (1.0, Box::new(central_control_evaluator)),
    ])
}
