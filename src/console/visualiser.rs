use crate::{
    board::Board,
    game::{GameResult, Player, Visualiser},
    r#move::Move,
    piece::{Color, Piece, PieceType},
};

pub struct ConsoleVisualiser;

impl Visualiser for ConsoleVisualiser {
    fn visualise(&self, board: &Board) {
        let visualisation = visualise_as_ascii(board);
        println!("{}", visualisation);
    }

    fn on_game_end(
        &self,
        board: &Board,
        white_player: &dyn Player,
        black_player: &dyn Player,
        result: GameResult,
    ) {
        let pgn = generate_pgn(board.history(), white_player, black_player, result);
        println!("\nPGN:\n{}", pgn);
    }
}

const RANK_SEPARATOR: &str = "+---+---+---+---+---+---+---+---+\n";

fn visualise_as_ascii(board: &Board) -> String {
    let mut unformatted_char_matrix = vec![' '; 64];

    for (square, piece) in board.get_pieces() {
        let piece_char = match piece.color {
            Color::White => piece.piece_type.to_char_with_pawn(),
            Color::Black => piece.piece_type.to_char_with_pawn().to_ascii_lowercase(),
        };
        unformatted_char_matrix[square.to_flat_index() as usize] = piece_char;
    }

    let mut formatted_output = String::new();
    formatted_output.push_str(RANK_SEPARATOR);

    for i in 0..8 {
        let rank_chars = &unformatted_char_matrix[((7 - i) * 8)..((7 - i + 1) * 8)];
        formatted_output.push_str(
            &rank_chars
                .iter()
                .map(|c| format!("| {} ", c))
                .collect::<String>(),
        );
        formatted_output.push_str("|\n");
        formatted_output.push_str(RANK_SEPARATOR);
    }

    formatted_output
}

fn generate_pgn(
    history: &Vec<(Move, Piece)>,
    white_player: &dyn Player,
    black_player: &dyn Player,
    result: GameResult,
) -> String {
    let mut pgn = String::new();

    pgn.push_str(&format!(
        "[white \"{}\"]\n[black \"{}\"]\n[Result \"{}\"]\n",
        white_player.name(),
        black_player.name(),
        result.to_string()
    ));

    for (i, (mv, piece)) in history.iter().enumerate() {
        if i % 2 == 0 {
            pgn.push_str(&format!("{}. ", (i / 2) + 1));
        }

        match (mv, piece.piece_type) {
            (Move::Capture { from, to: _ }, PieceType::Pawn) => {
                pgn.push_str(&format!(
                    "{}{}",
                    from.file_to_char(),
                    mv.to_standard_notation_suffix(),
                ));
            }

            (_, PieceType::Pawn) | (Move::Castle { .. }, _) => {
                pgn.push_str(&format!("{}", mv.to_standard_notation_suffix()));
            }

            (_, _) => {
                pgn.push_str(&format!(
                    "{}{}",
                    piece.piece_type.to_char(),
                    mv.to_standard_notation_suffix()
                ));
            }
        };

        pgn.push(' ');
    }

    pgn.push_str(result.to_string());

    pgn
}
