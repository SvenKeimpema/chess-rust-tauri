use tauri::App;
use tauri::Manager;
use tauri::test::MockRuntime;
use tauri::Window;

use crate::command_center::create_app;
use crate::get_bit;
use crate::test::ipc_helper::{call_get_bitboard, call_get_moves, call_make_move, call_undo_move};

// Note: The move test almost tests everything there is, due to the magic_generator also using a lot of math from:
// random.rs, bitboard.rs, state.rs. so this makes us able to skip over a lot of testing.

#[cfg(test)]
mod move_gen_tests {
    use super::*;

    #[test]
    fn test_moves_for_square() {
        let app: App<MockRuntime> = create_app(tauri::test::mock_builder());
        let window: Window<MockRuntime> = app.get_window("main").unwrap();
        let pawn_squares: Vec<u64> = call_get_moves(&window, r#"{"square": 52}"#);
        // test if the generated squares for pawn are valid
        assert_eq!(pawn_squares.get(0).unwrap(), &44);
        assert_eq!(pawn_squares.get(1).unwrap(), &36);

        let knight_squares: Vec<u64> = call_get_moves(&window, r#"{"square": 57}"#);

        assert_eq!(knight_squares.len(), 2);
    }

    #[test]
    fn test_make_move() {
        let app: App<MockRuntime> = create_app(tauri::test::mock_builder());
        let window: Window<MockRuntime> = app.get_window("main").unwrap();
        call_make_move(&window, r#"{"startSq": 52, "destSq": 36}"#);
        let bb: u64 = call_get_bitboard(&window)[0];

        // check if the piece has moved to the correct spot
        assert_eq!(get_bit!(bb, 36), true);
    }

    #[test]
    fn test_sliding_pieces() {
        let app: App<MockRuntime> = create_app(tauri::test::mock_builder());
        let window: Window<MockRuntime> = app.get_window("main").unwrap();

        // make a move so the bishop and queen can move
        call_make_move(&window, r#"{"startSq": 51, "destSq": 35}"#);
        // make sure it's white's move by making another move
        call_make_move(&window, r#"{"startSq": 10, "destSq": 18}"#);
        // make another move so the rook can move
        call_make_move(&window, r#"{"startSq": 55, "destSq": 47}"#);
        call_make_move(&window, r#"{"startSq": 9, "destSq": 17}"#);
        call_make_move(&window, r#"{"startSq": 62, "destSq": 45}"#);
        call_make_move(&window, r#"{"startSq": 11, "destSq": 19}"#);

        let bishop_squares: Vec<u64> = call_get_moves(&window, r#"{"square": 58}"#);
        let queen_squares: Vec<u64> = call_get_moves(&window, r#"{"square": 59}"#);
        let rook_squares: Vec<u64> = call_get_moves(&window, r#"{"square": 63}"#);

        assert_eq!(bishop_squares.len(), 5);
        assert_eq!(queen_squares.len(), 2);
        assert_eq!(rook_squares.len(), 2);
    }

    #[test]
    fn test_king_moves() {
        let app: App<MockRuntime> = create_app(tauri::test::mock_builder());
        let window: Window<MockRuntime> = app.get_window("main").unwrap();

        call_make_move(&window, r#"{"startSq": 52, "destSq": 36}"#);
        call_make_move(&window, r#"{"startSq": 10, "destSq": 18}"#);
        call_make_move(&window, r#"{"startSq": 60, "destSq": 52}"#);

        let bb: u64 = call_get_bitboard(&window)[5];
        assert_eq!(get_bit!(bb, 52), true);
    }
    #[test]
    fn test_undo_move() {
        let app: App<MockRuntime> = create_app(tauri::test::mock_builder());
        let window: Window<MockRuntime> = app.get_window("main").unwrap();
        call_make_move(&window, r#"{"startSq": 52, "destSq": 36}"#);
        call_undo_move(&window);
        let bb: u64 = call_get_bitboard(&window)[0];
        // check if the piece went back to it's original spot
        assert_ne!(get_bit!(bb, 36), true);
    }
}