pub struct Move {
    pub src: i32,
    pub dest: i32,
    pub capture: bool,
    pub castle: bool,
    pub en_passant: bool,
}
impl Clone for Move {
    fn clone(&self) -> Self {
        Self {
            ..*self
        }
    }
}

/// moves: Vec list of moves
/// move_count: total amount of non-empty Move in Vec<Move>
pub struct Moves {
    pub moves: Vec<Move>,
    pub move_count: i32,
}

pub trait AddMove {
    fn add_move(&mut self, src: i32, dest: i32, capture: bool, castle: bool, en_passant: bool);
}

impl Default for Move {
    fn default() -> Self {
        return Self {
            src: -1,
            dest: -1,
            capture: false,
            castle: false,
            en_passant: false,
        };
    }
}

impl Default for Moves {
    fn default() -> Self {
        let mov = Move { ..Default::default() };
        return Self {
            moves: vec![mov; 256],
            move_count: 0,
        };
    }
}

impl AddMove for Moves {
    fn add_move(&mut self, src: i32, dest: i32, capture: bool, castle: bool, en_passant: bool) {
        self.moves[self.move_count as usize] = Move { src, dest, capture, castle, en_passant };
        self.move_count += 1;
    }
}