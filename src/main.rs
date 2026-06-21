mod board;
mod console;
mod game;
mod r#move;
mod piece;
mod piece_matrix;
mod robot;
mod square;
mod standard_notation;

use crate::{
    console::{player::ConsolePlayer, visualiser::ConsoleVisualiser},
    game::Game,
    piece::Color,
    robot::RobotPlayer,
};

fn main() {
    let mut game = Game::new(
        Box::new(RobotPlayer::new(Color::White)),
        Box::new(RobotPlayer::new(Color::Black)),
        Box::new(ConsoleVisualiser),
    );

    game.play();
}
