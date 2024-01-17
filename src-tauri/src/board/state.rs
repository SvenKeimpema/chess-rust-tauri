use crate::set_bit;

/// bb: list of 12 vectors of positions of pieces(every piece has its own u64 so we can identify which piece is which)
/// <br><br>occ: list of 3 u64's 1st is whites occ, 2nd is blacks occ, 3rd is whites and blacks occ together
/// <br><br>white_to_move: dictates which side is allowed to move
pub struct GameState {
    pub bb: Vec<u64>,
    pub occ: Vec<u64>,
    pub white_to_move: bool
}

pub trait GameStateParser {
    fn get_capture_occ_idx(&mut self) -> i32;
    fn parse_fen(&mut self, fen: String);
}

impl GameStateParser for GameState {
    // returns which side needs to be check if there is a piece
    fn get_capture_occ_idx(&mut self) -> i32 {
        return if self.white_to_move {1} else {0};
    }

    fn parse_fen(&mut self, fen: String) {
        let mut board_index = 0;

        for char in fen.chars() {
            if char.is_digit(10) { board_index += char.to_digit(10) }

            match char {
                'p' => { set_bit!(&self.bb[0], 52);},
                _ => {}
            };
        }
    }
}