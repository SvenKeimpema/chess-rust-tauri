/*
references:
https://www.chessprogramming.org/Looking_for_Magics
https://www.chessprogramming.org/Magic_Bitboards#How_it_works (based on plain implementation)
https://www.chessprogramming.net/generating-magic-multipliers/
 */

use std::num::Wrapping;

use num_traits::WrappingShr;

use crate::board::bitboard::constants::{BISHOP_RELEVANT_BITS, ROOK_RELEVANT_BITS};
use crate::board::bitboard::math::set_occ;
use crate::helpers::random::generate_magic_number;
use crate::pieces::bishop::bishop::Bishop;
use crate::pieces::piece_interfaces::SlidingPiece;
use crate::pieces::rook::rook::Rook;

pub struct MagicMoves {
    pub bishop_moves: Vec<Vec<u64>>,
    pub rook_moves: Vec<Vec<u64>>,
    pub bishop_magic: Vec<u64>,
    pub rook_magic: Vec<u64>,
    pub bishop_generator: Bishop,
    pub rook_generator: Rook,
}

pub trait MagicMovesInit {
    fn init(&mut self);
}

pub trait MagicMovesGenerator {
    fn generate_magic_num(&mut self, sq: i32, relevant_bits: i32, gen_bishop: bool) -> u64;
    fn generate_magic_moves(&mut self, gen_bishop: bool);

    fn generate_magic_bishop(&mut self, mask: u64, relevant_total_bits: u32, square: i32, idx: i32);
    fn generate_magic_rook(&mut self, mask: u64, relevant_total_bits: u32, square: i32, idx: i32);

    fn get_bishop_moves(&mut self, sq: i32, occ: u64) -> u64;
    fn get_rook_moves(&mut self, sq: i32, occ: u64) -> u64;

}

impl Default for MagicMoves {
    fn default() -> Self {
        let mut bishop = Bishop { mask: vec![0u64; 64] };
        let mut rook =   Rook   { mask: vec![0u64; 64] };

        bishop.init();
        rook.init();

        // All possible magics per piece per square
        return Self {
            bishop_moves: vec![vec![0u64; 512]; 64],
            rook_moves: vec![vec![0u64; 4096]; 64],
            bishop_magic: vec![0u64; 64],
            rook_magic: vec![0u64; 64],
            bishop_generator: bishop,
            rook_generator: rook,
        };
    }
}

impl MagicMovesGenerator for MagicMoves {

    /// generates the magic number for bishop/rook on any sq
    fn generate_magic_num(&mut self, sq: i32, relevant_bits: i32, gen_bishop: bool) -> u64 {
        let mut attacks: Vec<u64> = vec![0u64; 4096];

        let mut occ: Vec<u64> = vec![0u64; 4096];
        let occ_idx: i32 = 1 << relevant_bits;
        let attack_mask: u64 = if gen_bishop { self.bishop_generator.get_mask(sq) } else { self.rook_generator.get_mask(sq) };

        for idx in 0..occ_idx {
            occ[idx as usize] = set_occ(attack_mask, relevant_bits as u64, idx as u64);
            attacks[idx as usize] = if gen_bishop
            { self.bishop_generator.get_full_move(sq, occ[idx as usize]) } else { self.rook_generator.get_full_move(sq, occ[idx as usize]) }
        }

        // max amount of loops we need to do until we find the magic num
        for _ in 0..1000000 {
            // TODO: speedup used_attacks initialization if possible
            let mut used_attacks: Vec<u64> = vec![0u64; 4096];

            // generate a fully random magic number(it's just a ph
            let magic_number: u64 = generate_magic_number();
            let magic_attack: Wrapping<u64> = Wrapping(attack_mask) * Wrapping(magic_number);

            // if magic_attack.count_ones > 6 we can always continue because it will always fail!
            if (magic_attack.0 & 0xFF00000000000000u64).count_ones() > 6 { continue; }

            let mut fail: bool = false;

            // Loop through indices from 0 to occ_idx
            for index in 0..occ_idx {
                let magic_occ = Wrapping(occ[index as usize]) * Wrapping(magic_number);
                let magic_index: usize = WrappingShr::wrapping_shr(&(magic_occ.0), (64 - relevant_bits) as u32)
                    as usize;

                // Check if there is a move stored at the magic index
                if used_attacks[magic_index] == 0u64 {
                    used_attacks[magic_index] = attacks[index as usize];
                }
                // if there is a move stored at the magic index it may NOT be different to the move we have currently picked
                else if used_attacks[magic_index] != attacks[index as usize] {
                    fail = true;
                    break;
                }
            }


            // if the for loop above was successful we can return magic number
            if !fail {
                return magic_number;
            }
        }

        println!("GENERATING MAGIC NUMBER FAILED!!");
        return 0u64;
    }

    /// generates moves with all occupancies so we can just lookup the move with the occ without having to generate it
    /// at the start of this file are most links on how the math behind this works!
    fn generate_magic_moves(&mut self, gen_bishop: bool) {
        for sq in 0..64 {
            let mask: u64 = if gen_bishop {self.bishop_generator.get_mask(sq)} else
                                            {self.rook_generator.get_mask(sq)};

            let relevant_total_bits: u32 = mask.count_ones();
            let occ_bits = 1u32 << relevant_total_bits;

            // loop over all possible occ
            for idx in 0..occ_bits {
                if gen_bishop {self.generate_magic_bishop(mask, relevant_total_bits, sq, idx as i32)} else
                              {self.generate_magic_rook(mask, relevant_total_bits, sq, idx as i32)}
            }
        }
    }

    /// generates the bishop move for the magic index
    fn generate_magic_bishop(&mut self, mask: u64, relevant_total_bits: u32, square: i32, idx: i32) {
        let occ: u64 = set_occ(mask, relevant_total_bits as u64, idx as u64);
        let magic_occ = Wrapping(occ) * Wrapping(self.bishop_magic[square as usize]);

        let magic_index: u64 = WrappingShr::wrapping_shr(
            &(magic_occ.0), (64 - BISHOP_RELEVANT_BITS[square as usize]) as u32
        );

        self.bishop_moves[square as usize][magic_index as usize] = self.bishop_generator.get_full_move(square, occ);
    }

    /// generates the rook move for the magic index
    fn generate_magic_rook(&mut self, mask: u64, relevant_total_bits: u32, square: i32, idx: i32) {
        let occ: u64 = set_occ(mask, relevant_total_bits as u64, idx as u64);
        let magic_occ = Wrapping(occ) * Wrapping(self.rook_magic[square as usize]);

        let magic_index: u64 = WrappingShr::wrapping_shr(
            &(magic_occ.0), (64 - ROOK_RELEVANT_BITS[square as usize]) as u32
        );

        self.rook_moves[square as usize][magic_index as usize] = self.rook_generator.get_full_move(square, occ);
    }

    /// gets the bishop move out of the table(O(1) lookup time!)
    fn get_bishop_moves(&mut self, sq: i32, mut occ: u64) -> u64 {
        occ &= self.bishop_generator.mask[sq as usize];
        occ = (Wrapping(occ) * Wrapping(self.bishop_magic[sq as usize])).0;
        occ = WrappingShr::wrapping_shr(&occ, (64 - BISHOP_RELEVANT_BITS[sq as usize]) as u32);

        return self.bishop_moves[sq as usize][occ as usize];
    }

    /// gets the rook move out of the table(O(1) lookup time!)
    fn get_rook_moves(&mut self, sq: i32, mut occ: u64) -> u64 {
        occ &= self.rook_generator.mask[sq as usize];
        occ = (Wrapping(occ) * Wrapping(self.rook_magic[sq as usize])).0;
        occ = WrappingShr::wrapping_shr(&occ, (64 - ROOK_RELEVANT_BITS[sq as usize]) as u32);

        return self.rook_moves[sq as usize][occ as usize];
    }
}

impl MagicMovesInit for MagicMoves {
    /// initializes magic nums for bishop and rook
    fn init(&mut self) {
        for sq in 0..64 {
            self.bishop_magic[sq as usize] =
                self.generate_magic_num(sq, BISHOP_RELEVANT_BITS[sq as usize] as i32, true);

            self.rook_magic[sq as usize] =
                self.generate_magic_num(sq, ROOK_RELEVANT_BITS[sq as usize] as i32, false);
        }

        self.generate_magic_moves(true);
        self.generate_magic_moves(false);
    }
}