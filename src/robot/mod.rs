mod evaluate;
mod tree;

use rayon::prelude::*;

use crate::{board::Board, game::Player, r#move::Move, piece::Color};

use evaluate::Evaluater;
use rayon::iter::IntoParallelRefIterator;
use tree::{PositionNode, PositionTree};

pub struct RobotPlayer {
    color: Color,
    position_evalutator: Evaluater,
}

impl RobotPlayer {
    pub fn new(color: Color) -> RobotPlayer {
        RobotPlayer {
            color,
            position_evalutator: create_evaluater(),
        }
    }

    fn get_best_move(&self, board: &Board, color: Color) -> Move {
        let position_tree = PositionTree::of_depth(board.clone(), color.opposite(), 3);
        let available_moves = position_tree.children_of(position_tree.root());

        let evaluations: Vec<(&PositionNode, f64)> = available_moves
            .par_iter()
            .map(|node| {
                (
                    *node,
                    position_tree.evaluation_of(node, &self.position_evalutator),
                )
            })
            .collect();

        let (best_node, best_eval) = evaluations
            .iter()
            .reduce(|best, current| if current.1 > best.1 { current } else { best })
            .unwrap();

        println!("Best move evaluation: {}", best_eval);

        return best_node.move_from_parent.unwrap();
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
        (2.0, Box::new(material_evaluator)),
        (1.0, Box::new(central_control_evaluator)),
    ])
}
