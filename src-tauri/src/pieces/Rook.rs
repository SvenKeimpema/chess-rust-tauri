pub mod rook {
    use crate::piece_interfaces::SlidingPiece;
    use crate::{get_bit, set_bit};

    pub struct Rook {
        pub mask: Vec<u64>
    }

    impl SlidingPiece for Rook {
        fn init(&mut self) {
            for sq in 0..64 {
                self.mask[sq] = self.get_mask(sq as i32);
            }
        }

        fn get_mask(&self, sq: i32) -> u64 {
            let mut moves: u64 = 0u64;
            let r: i32 = sq / 8;
            let f: i32 = sq % 8;

            for rank in r..7 {
                moves = set_bit!(moves, rank*8+f);
            }
            for rank in (1..r).rev() {
                moves = set_bit!(moves, rank*8+f);
            }
            for file in f..7 {
                moves = set_bit!(moves, r*8+file);
            }
            for file in (1..f).rev() {
                moves = set_bit!(moves, r*8+file);
            }

            return moves;
        }

        fn get_full_move(&self, sq: i32, block: u64) -> u64 {
            /*
            we subtract/add 1 from rank/file due to us not wanting to select the
            square the piece is standing on.
             */
            let mut moves: u64 = 0u64;
            let r: i32 = sq / 8;
            let f: i32 = sq % 8;

            for rank in r+1..8 {
                moves = set_bit!(moves, rank*8+f);
                if get_bit!(block, rank*8+f) {break}
            }
            for rank in (0..r-1).rev() {
                moves = set_bit!(moves, rank*8+f);
                if get_bit!(block, rank*8+f) {break}
            }
            for file in f+1..8 {
                moves = set_bit!(moves, r*8+file);
                if get_bit!(block, r*8+file) {break}
            }
            for file in (0..f-1).rev() {
                moves = set_bit!(moves, r*8+file);
                if get_bit!(block, r*8+file) {break}
            }

            return moves;
        }
    }
}