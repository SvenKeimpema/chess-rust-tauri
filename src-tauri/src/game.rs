use crate::{clear_bit, get_bit};
use crate::board::bitboard::math::get_ls1b;
use crate::board::state::{ChessGameState, GameStateParser};
use crate::moves::move_generator::{MoveCalculator, MoveGenerator};
use crate::moves::move_interfaces::{Move, Moves};
use crate::moves::move_maker::make_move;
use crate::moves::move_validator::validate_moves;

pub struct Game{
    pub game_state: ChessGameState,
    pub move_generator: MoveGenerator,
    pub default_fen: String
}

impl Game {
    pub fn get_game_state_mut(&mut self) -> &mut ChessGameState {
        &mut self.game_state
    }
    pub fn get_move_gen_mut(&mut self) -> &mut MoveGenerator {
        &mut self.move_generator
    }
}

pub trait GameHandler {
    fn init_game(&mut self);
    fn get_moves(&mut self) -> Moves;
    fn move_piece(&mut self, chess_move: Move);
    fn validate_moves(&mut self, unvalidated_moves: Moves) -> Moves;
    fn move_made_in_diff(&mut self, occ_diff: u64, start_square: i32) -> Vec<i32>;
}

impl Default for Game {
    fn default() -> Self {
        let game_state = ChessGameState { ..Default::default() };
        let move_generator = MoveGenerator { ..Default::default() };
        let default_fen : String = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string();
        return Self {
            game_state,
            move_generator,
            default_fen
        }
    }
}

impl GameHandler for Game {
    /// initializes game with default_fen
    fn init_game(&mut self) {
        self.game_state.parse_fen(&self.default_fen);
    }

    /// generates and returns all moves(maybe not legal move!)
    fn get_moves(&mut self) -> Moves {
        return self.move_generator.generate_moves(&mut self.game_state);
    }

    /// move a piece on the chess_board
    fn move_piece(&mut self, chess_move: Move) {
        make_move(&chess_move, &mut self.game_state)
    }

    /// returns all legal moves allowed to be made by a user
    fn validate_moves(&mut self, unvalidated_moves: Moves) -> Moves{
        validate_moves(unvalidated_moves, &mut self.move_generator, &mut self.game_state)
    }

    /// translates the bb to 2 square indexes so we now the start_square, and end_square
    fn move_made_in_diff(&mut self, mut occ_diff: u64, start_square: i32) -> Vec<i32> {
        let mut move_squares: Vec<i32> = vec![];

        // loop over occ_diff and push all bits set
        while occ_diff != 0u64 {
            let ls1sq: u64 = get_ls1b(occ_diff);
            move_squares.push(ls1sq as i32);
            clear_bit!(&mut occ_diff, ls1sq);
        }

        // we want to return end_sqaure -> start_square to undo a move
        if *move_squares.get(0).unwrap() == start_square {
            move_squares.reverse();
        }
        let start_sq = *move_squares.get(0).unwrap();
        for p in 0..12 {
            if get_bit!(self.game_state.bb[p], start_sq) {
                move_squares.push(p as i32);
                break;
            }
        }
        if move_squares.len() == 2 {move_squares.push(-1);}

        return move_squares;
    }
}