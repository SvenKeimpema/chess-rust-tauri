pub mod math {
    use crate::board::bitboard::constants::{DEBRUIJ_T, DEBRUIJ_M};

    /// sets the bit on a bitboard for example:
    /// bb=0000, sq=1 => 0100(sq starts by 0)
    #[macro_export]
    macro_rules! set_bit {
        ($bb:expr, $sq:expr) => {
            {
                let bb: &mut u64 = $bb;
                let sq: u64 = $sq as u64;

                if(sq < 64) {
                    *bb |= (1u64 << sq);
                }

                bb
            }
        };
    }

    /// returns true if a bit is one on the given square. for example:
    /// bb=0100, sq=1 => true because the first bit equals one(sq starts by 0)
    #[macro_export]
    macro_rules! get_bit {
        ($bb:expr, $sq:expr) => {
            {
                let mut bb: u64 = $bb;
                let sq: u64 = $sq as u64;

                if(sq < 64) {
                    bb &= (1u64 << sq);
                }else {
                    bb = 0;
                }

                bb != 0
            }
        };
    }

    /// clears or set's a bit on any given square. for example:\
    /// <br>bb=0100, sq=1 => 0000 because the first bit equals one(sq starts by 0)\
    /// <br>NOTE: only use this function whenever you are sure there is a bit set to one on said square.\
    /// <br>if you don't do this it might do undefined behaviour.
    #[macro_export]
    macro_rules! clear_bit {
       ($bb:expr, $sq:expr) => {
           {
               let bb: &mut u64 = &mut $bb;
               let sq: u64 = $sq as u64;

               if(sq < 64) {
                   *bb ^= (1u64 << sq);
               }

               bb
           }
       };
    }

    ///sets the bit on a square if it doesn't exists on the mask
    pub(crate) fn set_bit_not_exists(mut bb: u64, mask: u64, sq: i32) -> u64 {
        if sq < 64 && !get_bit!(mask, sq) {
            set_bit!(&mut bb, sq);

            return bb;
        }

        return bb;
    }

    // dead_code is allowed here, only used for debugging!
    #[allow(dead_code)]
    /// Testing tool for printing out bitboards to console
    pub fn print_bitboard(bb: u64) {
        for row in 0..8 {
            for col in 0..8 {
                let sq: i32 = row * 8 + col;

                // if there is a bit found on the square we will display it with an X
                if get_bit!(bb, sq) {
                    print!("X ");
                } else {
                    print!("- ");
                }
            }
            // make sure there is spacing between rows
            println!();
        }
        println!();
    }

    /// function is used to set every single bit until the index
    /// so if index is 4(00100) the result will be (11100)
    pub fn set_occ(mut mask: u64, bits: u64, index: u64) -> u64 {
        let mut result: u64 = 0;
        for bit in 0..bits {
            // least significant first square
            let ls1sq = get_ls1b(mask);

            clear_bit!(&mut mask, ls1sq as i32);

            if (index & (1u64 << bit)) != 0 {
                result |= 1u64 << ls1sq;
            }
        }
        return result;
    }

    /// gets the lowest first significant bit
    /// this means that 6(011) will output 2 since the second bit is a 1
    /// https://www.chessprogramming.org/BitScan De Bruijn Multiplication: With separated LS1B
    #[inline(always)]
    pub fn get_ls1b(bits: u64) -> u64 {
        DEBRUIJ_T[(((bits ^ bits.wrapping_sub(1)).wrapping_mul(DEBRUIJ_M)).wrapping_shr(58)) as usize]
    }
}

/// contains all u64 constants used in the program
pub mod constants {
    //  bitboard info for moving pieces
    pub const A_FILE: u64 = 72340172838076673u64;
    pub const AB_FILE: u64 = 217020518514230019u64;
    pub const GH_FILE: u64 = 13889313184910721216u64;
    pub const H_FILE: u64 = 9259542123273814144u64;

    pub const BISHOP_RELEVANT_BITS: [u64; 64] = [
        6, 5, 5, 5, 5, 5, 5, 6,
        5, 5, 5, 5, 5, 5, 5, 5,
        5, 5, 7, 7, 7, 7, 5, 5,
        5, 5, 7, 9, 9, 7, 5, 5,
        5, 5, 7, 9, 9, 7, 5, 5,
        5, 5, 7, 7, 7, 7, 5, 5,
        5, 5, 5, 5, 5, 5, 5, 5,
        6, 5, 5, 5, 5, 5, 5, 6
    ];

    // rook relevant occupancy bit count for every square on board
    pub const ROOK_RELEVANT_BITS: [u64; 64] = [
        12, 11, 11, 11, 11, 11, 11, 12,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        12, 11, 11, 11, 11, 11, 11, 12
    ];

    pub(crate) static DEBRUIJ_T: &'static [u64] = &[
        0, 47,  1, 56, 48, 27,  2, 60,
        57, 49, 41, 37, 28, 16,  3, 61,
        54, 58, 35, 52, 50, 42, 21, 44,
        38, 32, 29, 23, 17, 11,  4, 62,
        46, 55, 26, 59, 40, 36, 15, 53,
        34, 51, 20, 43, 31, 22, 10, 45,
        25, 39, 14, 33, 19, 30,  9, 24,
        13, 18,  8, 12,  7,  6,  5, 63
    ];

    pub(crate) const DEBRUIJ_M: u64 = 0x03f7_9d71_b4cb_0a89;
}

