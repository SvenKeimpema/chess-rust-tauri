use crate::moves::move_generator::{MoveCalculator, MoveGenerator};
use crate::moves::move_interfaces::Moves;
use crate::board::state::{GameState, GameStateParser};

pub struct Game {
    pub game_state: GameState,
    pub move_generator: MoveGenerator
}

pub trait GameHandler {
    fn init_game(&mut self);
    fn get_moves(&mut self) -> Moves;
}

impl Default for Game {
    fn default() -> Self {
        let game_state = GameState { ..Default::default() };
        let move_generator = MoveGenerator { ..Default::default() };
        return Self {
            game_state,
            move_generator

        }
    }
}

impl GameHandler for Game {
    fn init_game(&mut self) {
        self.game_state.parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    }

    fn get_moves(&mut self) -> Moves {
        return self.move_generator.generate_moves(&mut self.game_state);
    }
}