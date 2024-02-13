use crate::board::state::{ChessGameState, GameStateParser};
use crate::moves::move_generator::{MoveCalculator, MoveGenerator};
use crate::moves::move_interfaces::{AddMove, Moves};
use crate::moves::move_maker::make_move;

/// validates the moves so the user can't make illegal moves.
pub fn validate_moves(
    unvalidated_moves: Moves,
    move_generator: &mut MoveGenerator,
    game_state: &mut ChessGameState
) -> Moves {
    let mut valid_moves = Moves { ..Default::default() };

    for chess_move in &unvalidated_moves.moves {
        make_move(chess_move, game_state);
        let opp_moves: Moves = move_generator.generate_moves(game_state);

        // if the king is still alive after out move and all of the opponents moves, then the move is legal!
        if king_alive_after_moves(game_state, opp_moves) {
            valid_moves.add_move_class(chess_move);
        }

        game_state.undo_state();
    }

    valid_moves
}

/// returns true if the king of the opponent player is alive after every move is made.
pub fn king_alive_after_moves(game_state: &mut ChessGameState, m: Moves) -> bool{
    let king_side: usize = if game_state.white_to_move { 11 } else { 5 };

    // loops over every move and makes it, after it will check if it's still on the bb. if not return false
    for opp_move in &m.moves {
        make_move(opp_move, game_state);
        if game_state.bb[king_side] == 0u64 {
            game_state.undo_state();
            return false;
        }
        game_state.undo_state();
    }

    return true;
}