use std::default::Default;
use crate::bitboard::math::{get_ls1b};
use crate::{clear_bit, get_bit};
use crate::king::king::King;
use crate::knight::knight::Knight;
use crate::magic_moves::{MagicMoves, MagicMovesGenerator};
use crate::pawn::pawn::Pawn;
use crate::state::{GameState, GameStateParser};
use crate::move_i::{AddMove, Moves};

pub struct MoveGenerator {
    pub pawn_generator: Pawn,
    pub knight_generator: Knight,
    king_generator: King,
    magic_generator: MagicMoves,
}

pub trait MoveCalculator {
    fn generator_moves(&mut self, state: &mut GameState);
}

trait AllPiecesCalculator {
    fn generate_white_pawn_moves(&mut self, moves: &mut Moves, state: &mut GameState);
    fn generate_black_pawn_moves(&mut self, moves: &mut Moves, state: &mut GameState);
    fn generate_knight_moves(&mut self, moves: &mut Moves, state: &mut GameState);
    fn generate_bishop_moves(&mut self, moves: &mut Moves, state: &mut GameState);
    fn generate_rook_moves(&mut self, moves: &mut Moves, state: &mut GameState);
    fn generate_queen_moves(&mut self, moves: &mut Moves, state: &mut GameState);
    fn generate_king_moves(&mut self, moves: &mut Moves, state: &mut GameState);
    fn generate_attacking_moves(
        &mut self, start_sq: i32, attacking_moves: u64, moves: &mut Moves, state: &mut GameState
    );
}

impl MoveCalculator for MoveGenerator {
    fn generator_moves(&mut self, state: &mut GameState) {
        let moves = &mut Moves {..Default::default()};
        if state.white_to_move {
            self.generate_white_pawn_moves(moves, state)
        }else {
            self.generate_black_pawn_moves(moves, state)
        }
        self.generate_knight_moves(moves, state);
    }
}

impl AllPiecesCalculator for MoveGenerator {
    /// generates moves for all (white)pawns currently on the board
    fn generate_white_pawn_moves(&mut self, moves: &mut Moves, state: &mut GameState) {
        let mut bb: u64 = state.bb[0];

        while bb != 0u64 {
            let piece_sq = get_ls1b(bb) as i32;

            if !get_bit!(state.occ[2], piece_sq-8) {
                moves.add_move(piece_sq, piece_sq-8, false, false, false);

                if !get_bit!(state.occ[2], piece_sq-16) {
                    moves.add_move(piece_sq, piece_sq-16, false, false, false);
                }
            }

            let captures: u64 = self.pawn_generator.mask[piece_sq as usize][0];
            self.generate_attacking_moves(piece_sq, captures, moves, state);
            bb = clear_bit!(bb, piece_sq);
        }

    }

    /// generates moves for all (black)pawns currently on the board
    fn generate_black_pawn_moves(&mut self, moves: &mut Moves, state: &mut GameState) {
        // all black pawns
        let mut bb: u64 = state.bb[6];

        // go over all black pawns until none are left
        while bb != 0u64 {
            let piece_sq = get_ls1b(bb) as i32;

            // makes it so that pawn can go forward
            if !get_bit!(state.occ[2], piece_sq+8) {
                moves.add_move(piece_sq, piece_sq+8, false, false, false);

                if !get_bit!(state.occ[2], piece_sq+16) {
                    moves.add_move(piece_sq, piece_sq+16, false, false, false);
                }
            }

            let captures: u64 = self.pawn_generator.mask[piece_sq as usize][1];
            self.generate_attacking_moves(piece_sq, captures, moves, state);
            // remove the pawn in bb so we can goto the next one
            bb = clear_bit!(bb, piece_sq);
        }
    }

    fn generate_knight_moves(&mut self, moves: &mut Moves, state: &mut GameState) {
        // knights bb
        let bb = if state.white_to_move {state.bb[1]} else {state.bb[7]};
        while bb != 0u64 {
            let knight_sq: i32 = get_ls1b(bb) as i32;
            let knight_moves = self.knight_generator.mask[knight_sq as usize];
            self.generate_attacking_moves(knight_sq, knight_moves, moves, state);
        }
    }

    fn generate_bishop_moves(&mut self, moves: &mut Moves, state: &mut GameState) {
        // bishop bb
        let bb = if state.white_to_move {state.bb[2]} else {state.bb[8]};

        while bb != 0u64 {
            let bishop_sq: i32 = get_ls1b(bb) as i32;
            let bishop_moves = self.magic_generator.get_bishop_moves(bishop_sq, state.occ[2]);
            self.generate_attacking_moves(bishop_sq, bishop_moves, moves, state);
        }
    }

    fn generate_rook_moves(&mut self, moves: &mut Moves, state: &mut GameState) {
        // bishop bb
        let bb = if state.white_to_move {state.bb[3]} else {state.bb[9]};

        while bb != 0u64 {
            let rook_sq: i32 = get_ls1b(bb) as i32;
            let rook_moves = self.magic_generator.get_rook_moves(rook_sq, state.occ[2]);
            self.generate_attacking_moves(rook_sq, rook_moves, moves, state);
        }
    }

    fn generate_queen_moves(&mut self, moves: &mut Moves, state: &mut GameState) {
        let mut bb: u64 = if state.white_to_move {state.bb[4]} else {state.bb[10]};
        while bb != 0u64 {
            let piece_sq = get_ls1b(bb) as i32;

            let bishop_moves = self.magic_generator.get_bishop_moves(piece_sq, state.occ[2]);
            self.generate_attacking_moves(piece_sq, bishop_moves, moves, state);

            let rook_moves = self.magic_generator.get_rook_moves(piece_sq, state.occ[2]);
            self.generate_attacking_moves(piece_sq, rook_moves, moves, state);

            bb = clear_bit!(bb, piece_sq);
        }
    }

    fn generate_king_moves(&mut self, moves: &mut Moves, state: &mut GameState) {
        let mut bb: u64 = if state.white_to_move {state.bb[5]} else {state.bb[11]};
        while bb != 0u64 {
            let piece_sq: i32 = get_ls1b(bb) as i32;

            let king_moves: u64 = self.king_generator.mask[piece_sq as usize];
            self.generate_attacking_moves(piece_sq, king_moves, moves, state);

            bb = clear_bit!(bb, piece_sq);
        }
    }

    fn generate_attacking_moves(
        &mut self, start_sq: i32, mut attacking_moves: u64, moves: &mut Moves, state: &mut GameState
    ) {
        let occ_idx = state.get_capture_occ_idx();
        while attacking_moves != 0u64 {
            let sq: i32 = get_ls1b(attacking_moves) as i32;
            if get_bit!(state.occ[occ_idx as usize], sq) {
                moves.add_move(start_sq, sq, true, false, false);
            }else if !get_bit!(state.occ[2], sq) {
                moves.add_move(start_sq, sq, false, false, false);
            }
            attacking_moves = clear_bit!(attacking_moves, sq);
        }
    }
}

impl Default for MoveGenerator {
    fn default() -> Self {
        let pawn_generator = Pawn { ..Default::default() };
        let knight_generator = Knight { ..Default::default() };
        let king_generator = King { ..Default::default() };
        let magic_generator = MagicMoves { ..Default::default() };

        return Self {
            pawn_generator,
            knight_generator,
            king_generator,
            magic_generator
        }
    }
}