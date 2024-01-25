use crate::board::bitboard::math::get_ls1b;
use crate::board::state::{GameState, GameStateParser};
use crate::clear_bit;
use crate::moves::move_generator::{MoveCalculator, MoveGenerator};
use crate::moves::move_interfaces::{Move, Moves};
use crate::moves::move_maker::make_move;
use crate::moves::move_validator::validate_moves;

pub struct Game{
    pub game_state: GameState,
    pub move_generator: MoveGenerator,
    pub default_fen: String
}

impl Game {
    pub fn get_game_state_mut(&mut self) -> &mut GameState {
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
        let game_state = GameState { ..Default::default() };
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
    fn init_game(&mut self) {
        self.game_state.parse_fen(&self.default_fen);
    }

    fn get_moves(&mut self) -> Moves {
        return self.move_generator.generate_moves(&mut self.game_state);
    }

    fn move_piece(&mut self, chess_move: Move) {
        make_move(&chess_move, &mut self.game_state)
    }

    fn validate_moves(&mut self, unvalidated_moves: Moves) -> Moves{
        validate_moves(unvalidated_moves, &mut self.move_generator, &mut self.game_state)
    }

    fn move_made_in_diff(&mut self, mut occ_diff: u64, start_square: i32) -> Vec<i32> {
        let mut move_squares: Vec<i32> = vec![];

        while occ_diff != 0u64 {
            let ls1sq: u64 = get_ls1b(occ_diff);
            move_squares.push(ls1sq as i32);
            clear_bit!(&mut occ_diff, ls1sq);
        }

        if *move_squares.get(0).unwrap() == start_square {
            move_squares.reverse();
        }


        return move_squares;
    }
}