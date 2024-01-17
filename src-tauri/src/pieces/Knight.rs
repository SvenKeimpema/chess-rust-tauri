pub mod knight {
    use crate::piece_interfaces::NonSlidingPiece;
    use crate::bitboard::constants::{A_FILE, AB_FILE, GH_FILE,  H_FILE};
    use crate::bitboard::math::set_bit_not_exists;


    pub struct Knight {
        pub mask: Vec<u64>
    }

    impl Default for Knight {
        fn default() -> Self {
            Self {
                mask: vec![0u64; 64]
            }
        }
    }

    impl NonSlidingPiece for Knight {
        fn init(&mut self) {
            for sq in 0..64 {
                self.mask[sq] = self.get_mask(sq as i32);
            }
        }
        fn get_mask(&mut self, sq: i32) -> u64 {
            let mut attacks: u64 = 0u64;

            attacks = set_bit_not_exists(attacks, AB_FILE, sq-6);
            attacks = set_bit_not_exists(attacks, GH_FILE, sq-10);
            attacks = set_bit_not_exists(attacks, A_FILE, sq-15);
            attacks = set_bit_not_exists(attacks, H_FILE, sq-17);

            attacks = set_bit_not_exists(attacks, AB_FILE, sq+10);
            attacks = set_bit_not_exists(attacks, GH_FILE, sq+6);
            attacks = set_bit_not_exists(attacks, A_FILE, sq+17);
            attacks = set_bit_not_exists(attacks, H_FILE, sq+15);

            return attacks;
        }
    }
}