// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


// TODO: move around modules so they make sence!

use std::sync::{Mutex, MutexGuard};
use crate::board::bitboard::math::get_ls1b;

use crate::board::state::GameStateParser;
use crate::game::{Game, GameHandler};
use crate::moves::move_interfaces::Moves;
use crate::moves::move_validator::king_alive_after_moves;

mod pieces;
mod moves;
mod helpers;
mod board;
mod game;


pub struct GameState(pub Mutex<Game>);

/// if the front-end calls this command we will want to return the starting fen
/// https://nl.wikipedia.org/wiki/Forsyth-Edwards_Notation
#[tauri::command]
fn get_board(state: tauri::State<GameState>) -> String {
    let state_guard: MutexGuard<Game> = state.0.lock().unwrap();

    return state_guard.default_fen.clone();
}

/// if a square is pressed on the front-end return all moves so we can display them on the front-end
#[tauri::command]
fn select_square(square: i32, state: tauri::State<GameState>) -> Vec<i32> {
    let mut state_guard: MutexGuard<Game> = state.0.lock().unwrap();
    let mut vec: Vec<i32> = Vec::with_capacity(2);
    let unvalidated_moves: Moves = state_guard.get_moves();

    // validate all moves so that the king can easily be captured
    let validated_moves: Moves = state_guard.validate_moves(unvalidated_moves);

    for chess_move in validated_moves.moves {
        if chess_move.src == square {
            vec.push(chess_move.dest)
        }
    }

    vec
}

/// moves a piece on the rust side(will also check if the move is valid).
/// only call this for users not if you are using ai(algorithm) because this is really slow!
#[tauri::command]
fn move_piece(start_sq: i32, dest_sq: i32, state: tauri::State<GameState>) {
    let mut state_guard: MutexGuard<Game> = state.0.lock().unwrap();
    let moves: Moves = state_guard.get_moves();

    // validate moves again, pretty slow but makes it someone can't make invalid moves by sending bad requests
    let validated_moves: Moves = state_guard.validate_moves(moves);

    for chess_move in validated_moves.moves {
        if chess_move.src == start_sq && chess_move.dest == dest_sq {
            state_guard.move_piece(chess_move);
            break;
        }
    }
}

/// undo's the latest made chess move
#[tauri::command]
fn undo_move(state: tauri::State<GameState>) -> Vec<i32> {
    let mut state_guard: MutexGuard<Game> = state.0.lock().unwrap();

    // check if we even can undo the move
    if state_guard.game_state.saved_states.len() == 0 {
        return vec![0i32; 2];
    }

    // get difference in occupancies
    // idea behind this is if we XOR the prev and curr occ[2] we will find a made chess_move
    let occ_copy = state_guard.game_state.occ[2];
    state_guard.game_state.undo_state();
    let occ_diff = occ_copy ^ state_guard.game_state.occ[2];
    let start_square: i32 = get_ls1b(occ_diff & state_guard.game_state.occ[2]) as i32;

    return state_guard.move_made_in_diff(occ_diff, start_square);
}

/// returns a code -1 if the game isn't over, 0 if it is a draw, 1 if the game is won
#[tauri::command]
fn check_game_won(state: tauri::State<GameState>) -> i32{
    let mut state_guard: MutexGuard<Game> = state.0.lock().unwrap();
    let unvalidated_moves: Moves = state_guard.get_moves();

    // validate all moves so that the king can easily be captured
    let validated_moves: Moves = state_guard.validate_moves(unvalidated_moves);

    // validate if the user can make any moves, if not check if the king is in check.
    if validated_moves.moves.len() == 0 {
        // change side because the person can't make any moves anyways.
        state_guard.game_state.white_to_move = !state_guard.game_state.white_to_move;
        let opp_moves = state_guard.get_moves();

        return if king_alive_after_moves(&mut state_guard.game_state, opp_moves) {
            0
        }else {
            1
        }
    }

    return -1;
}

fn main() {
    let app_state = GameState (
        Mutex::new(Game { ..Default::default() }),
    );

    // init the game so we can make moves, see the board, ect.
    app_state.0.lock().unwrap().init_game();

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![get_board, select_square, move_piece, undo_move, check_game_won])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}