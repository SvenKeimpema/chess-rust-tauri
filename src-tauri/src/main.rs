// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


// TODO: move around modules so they make sence!

mod pieces;
mod moves;
mod helpers;
mod board;
mod game;

use crate::game::{Game, GameHandler};

#[tauri::command]
fn get_board() -> &'static str  {
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
}

#[tauri::command]
fn select_square(square: i8) -> Vec<i8> {
    let mut vec = Vec::with_capacity(2);

    vec.push(square);
    vec.push(2);
    vec
}

fn main() {
    let mut game_handler = Game {..Default::default() };
    game_handler.init_game();
    let moves = game_handler.get_moves();

    println!("{}", moves.move_count);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_board, select_square])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
