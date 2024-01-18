pub mod bishop {
    use num::range_step;
    use crate::{get_bit, set_bit};
    use crate::pieces::piece_interfaces::SlidingPiece;

    pub struct Bishop {
        pub mask: Vec<u64>
    }

    impl SlidingPiece for Bishop {
        fn init(&mut self) {
            for sq in 0..64 {
                self.mask[sq] = self.get_mask(sq as i32);
            }
        }

        fn get_mask(&self, sq: i32) -> u64 {
            let mut attacks: u64 = 0u64;
            let r: i32 = sq / 8;
            let f: i32 = sq % 8;

            for (rank, file) in (r+1..7).zip(f+1..7) {
                set_bit!(&mut attacks, rank*8+file);
            }
            for (rank, file) in (1..r).rev().zip(f+1..7) {
                set_bit!(&mut attacks, rank*8+file);
            }
            for (rank, file) in (r+1..7).zip((1..f).rev()) {
                set_bit!(&mut attacks, rank*8+file);
            }
            for (rank, file) in (1..r).rev().zip((1..f).rev()) {
                set_bit!(&mut attacks, rank*8+file);
            }

            return attacks;
        }

        fn get_full_move(&self, sq: i32, block: u64) -> u64 {
            let mut attacks: u64 = 0u64;
            let r: i32 = sq / 8;
            let f: i32 = sq % 8;

            for (rank, file) in (r+1..8).zip(f+1..8) {
                set_bit!(&mut attacks, rank*8+file);
                if get_bit!(block, rank*8+file) {break}
            }
            for (rank, file) in range_step(0, r-1, -1).zip(f+1..8) {
                set_bit!(&mut attacks, rank*8+file);
                if get_bit!(block, rank*8+file) { break }
            }
            for (rank, file) in (r+1..8).zip(range_step(0, f-1, -1)) {
                set_bit!(&mut attacks, rank*8+file);
                if get_bit!(block, rank*8+file) {break}
            }
            for (rank, file) in range_step(0, r-1, -1).zip(range_step(0, f-1, -1)) {
                set_bit!(&mut attacks, rank*8+file);
                if get_bit!(block, rank*8+file) {break}
            }


            return attacks;
        }
    }
}