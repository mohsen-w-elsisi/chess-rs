pub(crate) mod central_control;
pub(crate) mod material;

use crate::piece::{Color, PieceType};
use crate::piece_matrix::PieceMatrix;

pub trait EvaluationCriterion {
    fn evaluate(&self, board: &PieceMatrix, color: Color) -> f64;
}

pub struct Evaluater {
    criteria: Vec<(f64, Box<dyn EvaluationCriterion>)>,
}

impl Evaluater {
    pub fn new(criteria: Vec<(f64, Box<dyn EvaluationCriterion>)>) -> Self {
        Evaluater { criteria }
    }

    pub fn evaluate(&self, board: &PieceMatrix, color: Color) -> f64 {
        let evaluation: f64 = self
            .criteria
            .iter()
            .map(|(weight, criterion)| *weight as f64 * criterion.evaluate(board, color))
            .sum();

        let total_weights = self
            .criteria
            .iter()
            .map(|(weight, _)| *weight as f64)
            .sum::<f64>();

        evaluation / total_weights
    }
}

pub struct MaterialValues {
    pub pawn: f64,
    pub knight: f64,
    pub bishop: f64,
    pub rook: f64,
    pub queen: f64,
    pub king: f64,
}

impl MaterialValues {
    pub fn default() -> MaterialValues {
        MaterialValues {
            pawn: 1.0,
            knight: 3.0,
            bishop: 3.0,
            rook: 5.0,
            queen: 9.0,
            king: 0.0, // King is invaluable
        }
    }

    pub fn of(&self, piece_type: &PieceType) -> f64 {
        match piece_type {
            PieceType::Pawn => self.pawn,
            PieceType::Knight => self.knight,
            PieceType::Bishop => self.bishop,
            PieceType::Rook => self.rook,
            PieceType::Queen => self.queen,
            PieceType::King => self.king,
        }
    }
}
