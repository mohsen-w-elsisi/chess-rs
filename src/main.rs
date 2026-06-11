mod piece;
mod square;
mod r#move;
mod piece_matrix;
mod board;
mod standard_notation;
mod game;
mod console;
mod robot;

use crate::{
    console::{ConsolePlayer, ConsoleVisualiser},
    game::Game,
    piece::Color,
    robot::RobotPlayer,
};

fn main() {
    let mut game = Game::new(
        Box::new(ConsolePlayer::new(Color::White)),
        Box::new(RobotPlayer::new(Color::Black)),
        Box::new(ConsoleVisualiser),
    );

    game.play();
}
