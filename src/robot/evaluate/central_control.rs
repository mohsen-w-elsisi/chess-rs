use crate::{
    r#move::Move, piece::Color, piece_matrix::PieceMatrix, robot::evaluate::EvaluationCriterion,
    square::Square,
};

pub struct PieceActivityEvaluator {
    config: PieceActivityEvaluationConfig,
}

pub struct PieceActivityEvaluationConfig {
    pub opponent_piece_threat_weight: f64,
    pub flat_threat_weight: f64,
    pub central_control_weight: f64,
    pub semi_central_control_weight: f64,
    pub semi_edge_control_weight: f64,
    pub edge_control_weight: f64,
}

impl EvaluationCriterion for PieceActivityEvaluator {
    fn evaluate(&self, board: &PieceMatrix, color: Color) -> f64 {
        self.eval_piece_activity(board, color)
    }
}

impl PieceActivityEvaluator {
    pub fn new(config: PieceActivityEvaluationConfig) -> Self {
        PieceActivityEvaluator { config }
    }

    pub fn eval_piece_activity(&self, board: &PieceMatrix, color: Color) -> f64 {
        let my_score = self.eval_piece_activity_for_side(board, color);
        let opponent_score = self.eval_piece_activity_for_side(board, color.opposite());

        return (my_score - opponent_score) / (my_score + opponent_score);
    }
    
    fn eval_piece_activity_for_side(&self, board: &PieceMatrix, color: Color) -> f64 {
        let mut score = 0.0;

        let available_moves = board
            .get_pieces()
            .iter()
            .filter(|(_, piece)| piece.color == color)
            .flat_map(|(square, piece)| piece.get_available_moves(square, board))
            .collect::<Vec<_>>();

        for mv in available_moves {
            match mv {
                Move::Normal { to, from: _ } => score += self.square_centrality_value(&to),

                Move::Capture { to, from } => {
                    let capturing_piece = board.get_piece(&from).unwrap();
                    let captured_piece = board.get_piece(&to).unwrap();

                    let clamped_value_difference: f64 =
                        if captured_piece.piece_type.value() > capturing_piece.piece_type.value() {
                            captured_piece.piece_type.value() - capturing_piece.piece_type.value()
                        } else {
                            self.config.flat_threat_weight
                        };

                    score += clamped_value_difference * self.config.opponent_piece_threat_weight
                        + self.square_centrality_value(&to);
                }

                _ => unimplemented!(),
            }
        }

        score
    }

    fn square_centrality_value(&self, square: &Square) -> f64 {
        match SquareCentrallity::from(square) {
            SquareCentrallity::Center => self.config.central_control_weight,
            SquareCentrallity::SemiCenter => self.config.semi_central_control_weight,
            SquareCentrallity::SemiEdge => self.config.semi_edge_control_weight,
            SquareCentrallity::Edge => self.config.edge_control_weight,
        }
    }
}

enum SquareCentrallity {
    Center,
    SemiCenter,
    SemiEdge,
    Edge,
}

impl SquareCentrallity {
    pub fn from(square: &Square) -> Self {
        match square {
            Square {
                file: 0 | 7,
                rank: _,
            }
            | Square {
                file: _,
                rank: 0 | 7,
            } => SquareCentrallity::Edge,

            Square {
                file: 1 | 6,
                rank: _,
            }
            | Square {
                file: _,
                rank: 1 | 6,
            } => SquareCentrallity::SemiEdge,

            Square {
                file: 2 | 5,
                rank: _,
            }
            | Square {
                file: _,
                rank: 2 | 5,
            } => SquareCentrallity::SemiCenter,

            Square {
                rank: 3 | 4,
                file: 3 | 4,
            } => SquareCentrallity::Center,

            _ => unreachable!(),
        }
    }
}
