use tauri::Manager;

#[cfg(test)]
mod fen_tests {
    use serde_json::Value::String;
    use crate::command_center::create_app;

    use super::*;

    #[test]
    fn test_starting_fen() {
        let app = create_app(tauri::test::mock_builder());
        let window = app.get_window("main").unwrap();

        // do something with the app and window
        // in this case we'll run the my_cmd command with no arguments
        tauri::test::assert_ipc_response(
            &window,
            tauri::InvokePayload {
                cmd: "get_board".into(),
                tauri_module: None,
                callback: tauri::api::ipc::CallbackFn(0),
                error: tauri::api::ipc::CallbackFn(1),
                inner: serde_json::Value::Null,
            },
            Ok(String("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".parse().unwrap()))
        );
    }
}