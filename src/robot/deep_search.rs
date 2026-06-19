use std::collections::HashMap;

use crate::{
    board::{Board, MoveApplicationError},
    r#move::Move,
    piece::Color,
    piece_matrix::PieceMatrix,
    robot::evaluate::Evaluater,
};

pub struct DeepEvaluator {
    evaluater: Evaluater,
    depth: i32,
    color: Color,
    cache: HashMap<PieceMatrix, f64>,
}

impl DeepEvaluator {
    pub fn new(evaluater: Evaluater, depth: i32, color: Color) -> Self {
        DeepEvaluator {
            evaluater: evaluater,
            depth: depth,
            color: color,
            cache: HashMap::new(),
        }
    }

    pub fn evaluate(&self, position: &Board) -> f64 {
        self.recursive_evaluate_position(self.depth, position, self.color)
    }

    fn recursive_evaluate_position(&self, depth: i32, position: &Board, color: Color) -> f64 {
        if depth == 0 {
            let eval = self.evaluater.evaluate(&position.matrix(), color);
            return eval;
        }

        let mut child_evaluations = self.get_child_evaluations(depth, position, color.opposite());
        child_evaluations.sort_unstable_by(f64::total_cmp);

        return -child_evaluations
            .last()
            .cloned()
            .unwrap_or(if color == self.color { -1.0 } else { 1.0 }); // handles checkmate
    }

    fn get_child_evaluations(&self, depth: i32, position: &Board, color: Color) -> Vec<f64> {
        position
            .get_pieces()
            .into_iter()
            .filter(|piece_info| piece_info.1.color == color)
            .flat_map(|(square, piece)| piece.get_available_moves(&square, &position.matrix()))
            .filter_map(|m| new_position_if_legal_move(position, m, color))
            .map(|new_pos| self.recursive_evaluate_position(depth - 1, &new_pos, color))
            .collect::<Vec<_>>()
    }
}

fn new_position_if_legal_move(position: &Board, m: Move, color: Color) -> Option<Board> {
    let mut new_board = position.clone();
    match new_board.apply_move(&m, color) {
        Ok(_) => Some(new_board),
        Err(MoveApplicationError::KingInCheck) => None,
        Err(e) => unreachable!("Unexpected error while applying move: {:?}", e),
    }
}
