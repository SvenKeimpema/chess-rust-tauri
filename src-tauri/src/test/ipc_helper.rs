#[allow(dead_code)]

// we want to allow dead_code here because we are not using it when people play the game, so it gives dead code warnings

use serde_json::Value as JsonValue;
use tauri::test::MockRuntime;
use tauri::Window;

/// do A ipc(Inter-Process Communication) request to tauri, so we can call commands
///
/// ----
///
/// Args:
///
/// * `window` - mocked tauri app with tauri::test::Mock(best to use createApp)
/// * `Command` - Command you want to call on the tauri backend
/// * `data` - any kind of parameters you want to send to tauri with serde_json(serde_json is REQUIRED!)
pub fn ipc_request<T: std::fmt::Debug + for<'de> serde::de::Deserialize<'de>>(window: &Window<MockRuntime>, command: &str, data: &str) -> Result<T, T
> {
    return tauri::test::get_ipc_response::<T>(window, tauri::InvokePayload {
        cmd: command.into(),
        tauri_module: None,
        callback: tauri::api::ipc::CallbackFn(0),
        error: tauri::api::ipc::CallbackFn(1),
        inner: serde_json::from_str(data).unwrap(),
    });
}


pub fn call_get_moves(window: &Window<MockRuntime>, data: &str) -> Vec<u64> {
    ipc_request::<Vec<u64>>(
        window, "select_square", data,
    ).expect("selectserde_json square panicked!")
}

pub fn call_make_move(window: &Window<MockRuntime>, data: &str) {
    ipc_request::<JsonValue>(
        window, "move_piece", data,
    ).expect("move piece panicked!");
}

pub fn call_get_bitboard(window: &Window<MockRuntime>) -> Vec<u64> {
    return ipc_request::<Vec<u64>>(
        window, "get_bitboard", "{}",
    ).expect("move piece panicked!");
}

// pub fn call_get_occ(window: &Window<MockRuntime>) -> Vec<u64> {
//     return ipc_request::<Vec<u64>>(
//         window, "get_occ", "{}",
//     ).expect("move piece panicked!");
// }

pub fn call_undo_move(window: &Window<MockRuntime>) {
    ipc_request::<JsonValue>(window, "undo_move", "{}").expect("undo move ipc call panicked");
}