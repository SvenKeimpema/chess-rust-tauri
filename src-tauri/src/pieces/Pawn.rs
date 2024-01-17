pub mod pawn {
    use crate::piece_interfaces::MultiSideMovingPiece;
    use crate::bitboard::constants::{A_FILE,  H_FILE};
    use crate::bitboard::math::set_bit_not_exists;

    // in pawn_mask will be all pre generated moves from get_mask
    pub struct Pawn {
        pub mask: Vec<Vec<u64>>
    }

    impl Default for Pawn {
        fn default() -> Self {
            return Self {
                mask: vec![vec![0u64; 2]; 64]
            }
        }
    }

    impl MultiSideMovingPiece for Pawn {
        fn init(&mut self) {
            for sq in 0..64 {
                self.mask[sq][0] = self.get_mask(sq as i32, true);
                self.mask[sq][1] = self.get_mask(sq as i32, false);
            }
        }

        fn get_mask(&self, sq: i32, white_to_move: bool) -> u64{
            let mut moves: u64 = 0u64;

            if white_to_move {
                moves = set_bit_not_exists(moves, A_FILE, sq-7);
                moves = set_bit_not_exists(moves, H_FILE, sq-9);
            }else {
                moves = set_bit_not_exists(moves, H_FILE, sq+7);
                moves = set_bit_not_exists(moves, A_FILE, sq+9);
            }

            moves
        }
    }
}

