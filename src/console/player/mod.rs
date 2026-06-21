use crate::{
    board::{Board, MoveApplicationError},
    game::Player,
    r#move::Move,
    piece::Color,
    standard_notation::from_standard_notation,
};

pub struct ConsolePlayer {
    color: Color,
}

impl Player for ConsolePlayer {
    fn get_move(&self, board: &Board) -> Move {
        loop {
            let input = self.read_notation_from_console();
            match from_standard_notation(input.as_str(), board, &self.color) {
                Ok(mv) => match board.clone().apply_move(&mv, self.color) {
                    Ok(_) => return mv,
                    Err(MoveApplicationError::KingInCheck) => {
                        println!("Illegal move: King would be in check")
                    }
                    Err(error) => unreachable!("{:?}", error),
                },
                Err(error) => println!("{:?}", error),
            }
        }
    }

    fn name(&self) -> String {
        "Just a random dude".to_string()
    }
}

impl ConsolePlayer {
    pub fn new(color: Color) -> ConsolePlayer {
        ConsolePlayer { color }
    }

    fn read_notation_from_console(&self) -> String {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().to_string()
    }
}
