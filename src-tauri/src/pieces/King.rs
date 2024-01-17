pub mod king {
    use crate::piece_interfaces::NonSlidingPiece;
    use crate::bitboard::constants::{A_FILE,  H_FILE};
    use crate::bitboard::math::set_bit_not_exists;
    use crate::set_bit;

    pub struct King {
        pub mask: Vec<u64>
    }

    impl Default for King {
        fn default() -> Self {
            Self {
                mask: vec![0u64; 64]
            }
        }
    }

    impl NonSlidingPiece for King{
        fn init(&mut self) {
            for sq in 0..64 {
                self.mask[sq] = self.get_mask(sq as i32);
            }
        }

        fn get_mask(&mut self, sq: i32) -> u64 {
            let mut attacks: u64 = 0u64;

            attacks = set_bit_not_exists(attacks, A_FILE, sq+1);
            attacks = set_bit_not_exists(attacks, A_FILE, sq-7);
            attacks = set_bit_not_exists(attacks, A_FILE, sq+9);

            attacks = set_bit_not_exists(attacks, H_FILE, sq-1);
            attacks = set_bit_not_exists(attacks, H_FILE, sq+7);
            attacks = set_bit_not_exists(attacks, H_FILE, sq-9);

            attacks = set_bit!(attacks, sq-8);
            attacks = set_bit!(attacks, sq+8);

            return attacks;
        }
    }
}