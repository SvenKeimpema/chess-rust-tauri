use std::default::Default;

use crate::{clear_bit, get_bit};
use crate::board::bitboard::math::get_ls1b;
use crate::board::state::{GameState, GameStateParser};
use crate::moves::magic_moves::{MagicMoves, MagicMovesGenerator, MagicMovesInit};
use crate::moves::move_interfaces::{AddMove, Moves};
use crate::pieces::king::King;
use crate::pieces::knight::Knight;
use crate::pieces::pawn::pawn::Pawn;
use crate::pieces::piece_interfaces::{MultiSideMovingPiece, NonSlidingPiece};

pub struct MoveGenerator {
    pub pawn_generator: Pawn,
    pub knight_generator: Knight,
    pub king_generator: King,
    pub magic_generator: MagicMoves,
}

pub trait MoveCalculator {
    fn generate_moves(&mut self, state: &mut GameState) -> Moves;
}

trait AllPiecesCalculator {
    fn generate_white_pawn_moves(&mut self, moves: &mut Moves, state: &mut GameState);
    fn generate_black_pawn_moves(&mut self, moves: &mut Moves, state: &mut GameState);
    fn generate_pawn_capture(&mut self, piece_sq: i32, captures: u64, moves: &mut Moves, state: &mut GameState);
    fn generate_knight_moves(&mut self, moves: &mut Moves, state: &mut GameState);
    fn generate_bishop_moves(&mut self, moves: &mut Moves, state: &mut GameState);
    fn generate_rook_moves(&mut self, moves: &mut Moves, state: &mut GameState);
    fn generate_queen_moves(&mut self, moves: &mut Moves, state: &mut GameState);
    fn generate_king_moves(&mut self, moves: &mut Moves, state: &mut GameState);
    fn generate_attacking_moves(
        &mut self, start_sq: i32, piece_type: i32, attacking_moves: u64, moves: &mut Moves, state: &mut GameState,
    );
}

impl MoveCalculator for MoveGenerator {
    fn generate_moves(&mut self, state: &mut GameState) -> Moves {
        let mut moves = Moves { ..Default::default() };

        if state.white_to_move {
            self.generate_white_pawn_moves(&mut moves, state)
        } else {
            self.generate_black_pawn_moves(&mut moves, state)
        }

        self.generate_knight_moves(&mut moves, state);
        self.generate_bishop_moves(&mut moves, state);
        self.generate_rook_moves(&mut moves, state);
        self.generate_queen_moves(&mut moves, state);
        self.generate_king_moves(&mut moves, state);

        return moves;
    }
}

impl AllPiecesCalculator for MoveGenerator {
    /// generates moves for all (white)pawns currently on the board
    fn generate_white_pawn_moves(&mut self, mut moves: &mut Moves, state: &mut GameState) {
        let piece_type: i32 = 0;
        let mut bb: u64 = state.bb[piece_type as usize];

        while bb != 0u64 {
            let piece_sq = get_ls1b(bb) as i32;

            if !get_bit!(state.occ[2], piece_sq-8) {
                moves.add_move(piece_sq, piece_sq - 8, piece_type, false, false, false);

                if 47 < piece_sq && piece_sq < 56 && !get_bit!(state.occ[2], piece_sq-16) {
                    moves.add_move(piece_sq, piece_sq - 16, piece_type, false, false, false);
                }
            }

            let captures: u64 = self.pawn_generator.mask[piece_sq as usize][0];
            self.generate_pawn_capture(piece_sq, captures, &mut moves, state);

            clear_bit!(&mut bb, piece_sq);
        }
    }

    /// generates moves for all (black)pawns currently on the board
    fn generate_black_pawn_moves(&mut self, mut moves: &mut Moves, state: &mut GameState) {
        // all black pawns
        let piece_type: i32 = 6;
        let mut bb: u64 = state.bb[piece_type as usize];

        // go over all black pawns until none are left
        while bb != 0u64 {
            let piece_sq = get_ls1b(bb) as i32;

            // makes it so that pawn can go forward
            if !get_bit!(state.occ[2], piece_sq+8) {
                moves.add_move(piece_sq, piece_sq + 8, piece_type, false, false, false);

                if 7 < piece_sq && piece_sq < 16 && !get_bit!(state.occ[2], piece_sq+16) {
                    moves.add_move(piece_sq, piece_sq + 16, piece_type, false, false, false);
                }
            }

            let captures: u64 = self.pawn_generator.mask[piece_sq as usize][1];
            // transforms capture moves on the u64 to the moves vec
            self.generate_pawn_capture(piece_sq, captures, &mut moves, state);

            // remove the pawn in bb so we can goto the next one
            clear_bit!(&mut bb, piece_sq);
        }
    }

    /// attacking moves for pawns that are set on the u64 will goto moves based on the game state and the piece square
    fn generate_pawn_capture(&mut self, piece_sq: i32, mut captures: u64, moves: &mut Moves, state: &mut GameState) {
        let piece_type = if state.white_to_move { 0 } else { 6 };
        let occ_idx = if state.white_to_move { 1 } else { 0 };

        while captures != 0u64 {
            let sq: i32 = get_ls1b(captures) as i32;

            if get_bit!(state.occ[occ_idx], sq) {
                moves.add_move(piece_sq, sq, piece_type, true, false, false);
            }

            clear_bit!(&mut captures, sq);
        }
    }

    /// generates all moves for the knight on the current board
    fn generate_knight_moves(&mut self, mut moves: &mut Moves, state: &mut GameState) {
        // knights bb
        let piece_type: i32 = if state.white_to_move { 1 } else { 7 };
        let mut bb = state.bb[piece_type as usize];
        while bb != 0u64 {
            let knight_sq: i32 = get_ls1b(bb) as i32;
            let knight_moves = self.knight_generator.mask[knight_sq as usize];
            self.generate_attacking_moves(knight_sq, piece_type, knight_moves, &mut moves, state);
            clear_bit!(&mut bb, knight_sq);
        }
    }

    /// generates all moves for the bishop on the current board
    fn generate_bishop_moves(&mut self, moves: &mut Moves, state: &mut GameState) {
        // bishop bb
        let piece_type: i32 = if state.white_to_move { 2 } else { 8 };
        let mut bb = state.bb[piece_type as usize];

        while bb != 0u64 {
            let bishop_sq: i32 = get_ls1b(bb) as i32;
            let bishop_moves = self.magic_generator.get_bishop_moves(bishop_sq, state.occ[2]);
            self.generate_attacking_moves(bishop_sq, piece_type, bishop_moves, moves, state);
            clear_bit!(&mut bb, bishop_sq);
        }
    }

    /// generates all moves for the rook on the current board
    fn generate_rook_moves(&mut self, mut moves: &mut Moves, state: &mut GameState) {
        // bishop bb
        let piece_type: i32 = if state.white_to_move { 3 } else { 9 };
        let mut bb = state.bb[piece_type as usize];

        while bb != 0u64 {
            let rook_sq: i32 = get_ls1b(bb) as i32;
            let rook_moves = self.magic_generator.get_rook_moves(rook_sq, state.occ[2]);
            self.generate_attacking_moves(rook_sq, piece_type, rook_moves, &mut moves, state);
            clear_bit!(&mut bb, rook_sq);
        }
    }

    /// generates all moves for the queen on the current board
    fn generate_queen_moves(&mut self, mut moves: &mut Moves, state: &mut GameState) {
        let piece_type: i32 = if state.white_to_move { 4 } else { 10 };
        let mut bb: u64 = state.bb[piece_type as usize];
        while bb != 0u64 {
            let piece_sq = get_ls1b(bb) as i32;

            let bishop_moves = self.magic_generator.get_bishop_moves(piece_sq, state.occ[2]);
            self.generate_attacking_moves(piece_sq, piece_type, bishop_moves, &mut moves, state);

            let rook_moves = self.magic_generator.get_rook_moves(piece_sq, state.occ[2]);
            self.generate_attacking_moves(piece_sq, piece_type, rook_moves, &mut moves, state);

            clear_bit!(&mut bb, piece_sq);
        }
    }

    /// generates all moves for the king on the current board
    fn generate_king_moves(&mut self, mut moves: &mut Moves, state: &mut GameState) {
        let piece_type: i32 = if state.white_to_move { 5 } else { 11 };
        let mut bb: u64 = state.bb[piece_type as usize];
        while bb != 0u64 {
            let piece_sq: i32 = get_ls1b(bb) as i32;

            let king_moves: u64 = self.king_generator.mask[piece_sq as usize];
            self.generate_attacking_moves(piece_sq, piece_type, king_moves, &mut moves, state);

            clear_bit!(&mut bb, piece_sq);
        }
    }

    /// generates the moves for pieces, adds a move non-capture move if there isn't a piece on the given square.
    /// if there is a piece on the attacking square we can set capture=true
    fn generate_attacking_moves(
        &mut self, start_sq: i32, piece_type: i32, mut attacking_moves: u64, moves: &mut Moves, state: &mut GameState,
    ) {
        let occ_idx = state.get_capture_occ_idx();
        while attacking_moves != 0u64 {
            let sq: i32 = get_ls1b(attacking_moves) as i32;

            if get_bit!(state.occ[occ_idx as usize], sq) {
                moves.add_move(start_sq, sq, piece_type, true, false, false);
            } else if !get_bit!(state.occ[2], sq) {
                moves.add_move(start_sq, sq, piece_type, false, false, false);
            }

            clear_bit!(&mut attacking_moves, sq);
        }
    }
}

impl Default for MoveGenerator {
    fn default() -> Self {
        let mut pawn_generator = Pawn { ..Default::default() };
        let mut knight_generator = Knight { ..Default::default() };
        let mut king_generator = King { ..Default::default() };
        let mut magic_generator = MagicMoves { ..Default::default() };

        pawn_generator.init();
        knight_generator.init();
        king_generator.init();
        magic_generator.init();

        return Self {
            pawn_generator,
            knight_generator,
            king_generator,
            magic_generator,
        };
    }
}