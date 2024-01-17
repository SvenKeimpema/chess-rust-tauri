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
    fn parse_fen(&mut self, fen: &str);
}

impl Default for GameState {
    fn default() -> Self {
        return Self{
            bb: vec![0u64; 12],
            occ: vec![0u64; 3],
            white_to_move: true
        }
    }
}

impl GameStateParser for GameState {
    // returns which side needs to be check if there is a piece
    fn get_capture_occ_idx(&mut self) -> i32 {
        return if self.white_to_move {1} else {0};
    }

    fn parse_fen(&mut self, fen: &str) {
        let mut board_index: u32 = 0;

        for char in fen.chars() {
            if char.is_digit(10) { board_index += char.to_digit(10).unwrap() }

            match char {
                'P' => { set_bit!(&mut self.bb[0],  board_index); board_index += 1;},
                'N' => { set_bit!(&mut self.bb[1],  board_index); board_index += 1;},
                'B' => { set_bit!(&mut self.bb[2],  board_index); board_index += 1;},
                'R' => { set_bit!(&mut self.bb[3],  board_index); board_index += 1;},
                'Q' => { set_bit!(&mut self.bb[4],  board_index); board_index += 1;},
                'K' => { set_bit!(&mut self.bb[5],  board_index); board_index += 1;},
                'p' => { set_bit!(&mut self.bb[6],  board_index); board_index += 1;},
                'n' => { set_bit!(&mut self.bb[7],  board_index); board_index += 1;},
                'b' => { set_bit!(&mut self.bb[8],  board_index); board_index += 1;},
                'r' => { set_bit!(&mut self.bb[9],  board_index); board_index += 1;},
                'q' => { set_bit!(&mut self.bb[10], board_index); board_index += 1;},
                'k' => { set_bit!(&mut self.bb[11], board_index); board_index += 1;},
                _ => {continue;}
            };
        }

        for i in 0..6 {
            self.occ[0] |= self.bb[i];
        }

        for i in 6..12 {
            self.occ[1] = self.bb[i];
        }

        self.occ[2] = self.occ[0] | self.occ[1];
    }
}