use crate::{board::Board, r#move::Move, piece::Color};

pub struct Game {
    board: Board,
    white_player: Box<dyn Player>,
    black_player: Box<dyn Player>,
    visuliser: Box<dyn Visualiser>,
    current_turn: Color,
    result: Option<GameResult>,
}

impl Game {
    pub fn new(
        white_player: Box<dyn Player>,
        black_player: Box<dyn Player>,
        visuliser: Box<dyn Visualiser>,
    ) -> Game {
        Game {
            board: Board::initial(),
            white_player,
            black_player,
            visuliser,
            current_turn: Color::White,
            result: None,
        }
    }

    pub fn custom(
        white_player: Box<dyn Player>,
        black_player: Box<dyn Player>,
        visuliser: Box<dyn Visualiser>,
        board: Board,
        starting_color: Color,
    ) -> Game {
        Game {
            board,
            white_player,
            black_player,
            visuliser,
            current_turn: starting_color,
            result: None,
        }
    }

    pub fn play(&mut self) {
        loop {
            if self.board.is_checkmate(self.current_turn) {
                self.result = Some(GameResult::for_winner(self.current_turn.opposite()));
                break;
            }

            if self.board.is_stalemate(self.current_turn) {
                self.result = Some(GameResult::Draw);
                break;
            }
            
            let player_move = self.get_next_move();
            self.board
                .apply_move(&player_move, self.current_turn)
                .unwrap();
            self.visuliser.visualise(&self.board);
            self.flip_turn();
        }

        self.visuliser.on_game_end(
            &self.board,
            &*self.white_player,
            &*self.black_player,
            self.result.clone().unwrap(),
        );
    }

    fn get_next_move(&self) -> Move {
        match self.current_turn {
            Color::White => self.white_player.get_move(&self.board),
            Color::Black => self.black_player.get_move(&self.board),
        }
    }

    fn flip_turn(&mut self) {
        self.current_turn = self.current_turn.opposite();
    }
}

pub trait Player {
    fn get_move(&self, board: &Board) -> Move;
    fn name(&self) -> String;
}

pub trait Visualiser {
    fn visualise(&self, board: &Board);
    fn on_game_end(
        &self,
        board: &Board,
        white_player: &dyn Player,
        black_player: &dyn Player,
        result: GameResult,
    );
}

#[derive(Clone)]
pub enum GameResult {
    WhiteWins,
    BlackWins,
    Draw,
}

impl GameResult {
    pub fn for_winner(color: Color) -> GameResult {
        match color {
            Color::White => GameResult::WhiteWins,
            Color::Black => GameResult::BlackWins,
        }
    }
}
