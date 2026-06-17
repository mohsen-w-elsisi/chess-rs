use crate::{board::Board, r#move::Move, piece::Color};

pub struct Game {
    board: Board,
    white_player: Box<dyn Player>,
    black_player: Box<dyn Player>,
    visuliser: Box<dyn Visualiser>,
    current_turn: Color,
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
        }
    }

    pub fn play(&mut self) {
        loop {
            let player_move = self.get_next_move();
            self.board.apply_move(&player_move, self.current_turn).unwrap();
            self.visuliser.visualise(&self.board);
            self.flip_turn();
        }
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
}

pub trait Visualiser {
    fn visualise(&self, board: &Board);
}
