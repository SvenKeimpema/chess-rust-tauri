// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// TODO: move around modules so they make sence!
#[path = "board/bitboard.rs"] mod bitboard;
#[path = "pieces/IPiece.rs"] mod piece_interfaces;
#[path = "pieces/Pawn.rs"] mod pawn;
#[path = "pieces/Knight.rs"] mod knight;
#[path = "pieces/King.rs"] mod king;
#[path = "pieces/Rook.rs"] mod rook;
#[path = "pieces/Bishop.rs"] mod bishop;
#[path = "helpers/Random.rs"] mod random;
#[path = "moves/MoveGenerator.rs"] mod move_gen;
#[path = "moves/MagicMoves.rs"] mod magic_moves;
#[path = "board/state.rs"] mod state;
#[path = "moves/IMove.rs"] mod move_i;

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
    let bb: u64 = 0u64;

    set_bit!(&bb, 52);

    println!("{}", bb);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_board, select_square])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
