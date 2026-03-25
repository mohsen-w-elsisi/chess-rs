pub struct Square {
    pub file: u8,
    pub rank: u8,
}

impl Square {
    pub fn to_flat_index(&self) -> u8 {
        self.rank * 8 + self.file
    }

    pub fn up(&self) -> Square {
        Square {
            file: self.file,
            rank: self.rank + 1,
        }
    }

    pub fn down(&self) -> Square {
        Square {
            file: self.file,
            rank: self.rank - 1,
        }
    }

    pub fn left(&self) -> Square {
        Square {
            file: self.file - 1,
            rank: self.rank,
        }
    }

    pub fn right(&self) -> Square {
        Square {
            file: self.file + 1,
            rank: self.rank,
        }
    }

    pub fn up_left(&self) -> Square {
        self.up().left()
    }

    pub fn up_right(&self) -> Square {
        self.up().right()
    }

    pub fn down_left(&self) -> Square {
        self.down().left()
    }

    pub fn down_right(&self) -> Square {
        self.down().right()
    }
}

pub struct Move {
    pub from: Square,
    pub to: Square,
}
