use crate::board::state::{ChessGameState, GameStateParser};
use crate::{clear_bit, get_bit, set_bit};
use crate::moves::move_interfaces::Move;

/// make a move on the chess board
pub fn make_move(chess_move: &Move, game_state: &mut ChessGameState) {
    game_state.save_state();
    let start_opponent_pieces: usize = if game_state.white_to_move { 6 } else { 0 };
    let end_oppenent_pieces: usize = if game_state.white_to_move { 12 } else { 6 };

    set_bit!(&mut game_state.bb[chess_move.piece_type as usize], chess_move.dest);
    clear_bit!(&mut game_state.bb[chess_move.piece_type as usize], chess_move.src);

//  clear captured piece
    if chess_move.capture {
        for piece in start_opponent_pieces..end_oppenent_pieces {
            if get_bit!(game_state.bb[piece], chess_move.dest) {
                clear_bit!(&mut game_state.bb[piece], chess_move.dest);
                break;
            }
        }
    }

    game_state.white_to_move = !game_state.white_to_move;
    game_state.update_occ();
}