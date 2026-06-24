use crate::{
    board::Board,
    r#move::Move,
    piece::{Color, PieceType},
    piece_matrix::PieceMatrix,
    robot::evaluate::EvaluationCriterion,
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
    fn evaluate(&self, board: &Board, color: Color) -> f64 {
        self.eval_piece_activity(board, color)
    }
}

impl PieceActivityEvaluator {
    pub fn new(config: PieceActivityEvaluationConfig) -> Self {
        PieceActivityEvaluator { config }
    }

    pub fn eval_piece_activity(&self, board: &Board, color: Color) -> f64 {
        let my_score = self.eval_piece_activity_for_side(board, color);
        let opponent_score = self.eval_piece_activity_for_side(board, color.opposite());

        return (my_score - opponent_score) / (my_score + opponent_score);
    }

    fn eval_piece_activity_for_side(&self, board: &Board, color: Color) -> f64 {
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
                    let capturing_piece = board.get_piece(&from).unwrap().piece_type;
                    let captured_piece = board.get_piece(&to).unwrap().piece_type;

                    score += self.threat_bonus(&capturing_piece, &captured_piece)
                        + self.square_centrality_value(&to);
                }

                Move::EnPassent { from: _, to } => {
                    score += self.square_centrality_value(&to)
                        + self.config.flat_threat_weight * self.config.opponent_piece_threat_weight
                }

                Move::Promotion {
                    from: _,
                    to,
                    capture,
                    promotion_piece_type: _,
                } => {
                    score += self.square_centrality_value(&to);
                    if capture {
                        let captured_piece = board.get_piece(&to).unwrap().piece_type;
                        score += self.threat_bonus(&PieceType::Pawn, &captured_piece);
                    }
                }

                Move::Castle { side, color } => unimplemented!(),
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

    fn threat_bonus(&self, threatening_piece: &PieceType, threatened_piece: &PieceType) -> f64 {
        let mut absolute_threat_bonus = threatened_piece.value() - threatening_piece.value();
        if absolute_threat_bonus < 0.0 {
            absolute_threat_bonus = self.config.flat_threat_weight;
        }
        absolute_threat_bonus * self.config.opponent_piece_threat_weight
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
