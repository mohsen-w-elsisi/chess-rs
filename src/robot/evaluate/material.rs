use crate::{piece::Color, piece_matrix::PieceMatrix, robot::evaluate::{EvaluationCriterion, MaterialValues}};

pub struct MaterialEvaluator {
    config: MaterialEvaluationConfig,
}

pub struct MaterialEvaluationConfig {
    pub piece_values: MaterialValues,
}

impl MaterialEvaluator {
    pub fn new(config: MaterialEvaluationConfig) -> Self {
        MaterialEvaluator { config }
    }
}

impl EvaluationCriterion for MaterialEvaluator {
    fn evaluate(&self, board: &PieceMatrix, color: Color) -> f64 {
        let mut my_score = 0.0;
        let mut opponent_score = 0.0;

        for (_, piece) in board.get_pieces() {
            let piece_value = self.config.piece_values.of(&piece.piece_type);
            if piece.color == color {
                my_score += piece_value;
            } else {
                opponent_score += piece_value;
            }
        }

        return (my_score - opponent_score) / (my_score + opponent_score);
    }
}
