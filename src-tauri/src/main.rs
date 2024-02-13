// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::command_center::{create_app, testing_active};
use crate::game::Game;
use std::sync::Mutex;
use crate::helpers::random::generate_magic_number;

mod pieces;
mod moves;
mod helpers;
mod board;
mod game;
mod test;
mod command_center;


// Makes it so tauri can handle the game state
pub struct TauriStateHolder(pub Mutex<Game>);


fn main() {
    if testing_active() {
        println!("you may not have testing env active while running the app!");
        return;
    }

    let app = create_app(tauri::Builder::default());
    app.run(|_handle, _event| {});
}