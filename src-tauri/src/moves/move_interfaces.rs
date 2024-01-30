/*
simple way to store move information
 */

#[derive(Copy)]
pub struct Move {
    pub src: i32,
    pub dest: i32,
    pub piece_type: i32,
    pub capture: bool,
    pub castle: bool,
    pub en_passant: bool,
}

/// give Move the ability to Clone(simply will return Move(..current_set_vars)
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
}

pub trait AddMove {
    fn add_move(&mut self, src: i32, dest: i32, piece_type: i32, capture: bool, castle: bool, en_passant: bool);
    fn add_move_class(&mut self, chess_move: &Move);
}

impl Default for Move {
    fn default() -> Self {
        return Self {
            src: -1,
            dest: -1,
            piece_type: -1,
            capture: false,
            castle: false,
            en_passant: false,
        };
    }
}

impl Default for Moves {
    fn default() -> Self {
        return Self {
            moves: vec![],
        };
    }
}

impl AddMove for Moves {
    /// add a move to the moves(array)
    fn add_move(&mut self, src: i32, dest: i32, piece_type: i32, capture: bool, castle: bool, en_passant: bool) {
        self.moves.push(Move { src, dest, piece_type, capture, castle, en_passant });
    }

    /// adds a move that has already been initialized and set
    fn add_move_class(&mut self, chess_move: &Move) {
        self.moves.push(chess_move.clone());
    }
}