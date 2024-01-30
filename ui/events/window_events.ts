import {requests} from "../requests/tauri_requests";

let Requests = new requests();

export class window_events {
    last_click: number = -1;

    /*
    calculates which square the user has clicked based on mouseEvent
    Note this function should only be performed when the user clicked inside the square.
     */
    square_clicked(event: MouseEvent) {
        // the size of the chess_board doesn't matter the only thing that matters is that we do /8 for screen
        // width/height. This is because there are 8 squares on a chess_board(total amount of square may not increase
        // without changing this code)
        let chess_board = document.getElementById("chess_board");

        if(chess_board === null) return;

        let square_width = event.x / (chess_board.offsetWidth / 8);
        let square_height = event.y / (chess_board.offsetHeight / 8);

        return Math.floor(square_height) * 8 + Math.floor(square_width)
    }

    // if the key `z` has been pressed we want to undo a move
    init_keydown_event() {
        window.onkeydown = (event: KeyboardEvent) => {
            if(event.key.toLowerCase() == "z") {
                Requests.undo_move();
            }
        }
    }

    init_square_clicked_event() {
        window.onclick = (event: MouseEvent) => {
            let square_clicked: number | undefined = this.square_clicked(event);
            let chess_squares = document.getElementsByClassName("square")

            if(square_clicked === undefined) return;

            if (chess_squares[square_clicked].classList.contains("movable")) {
                Requests.move_piece_request(this.last_click, square_clicked)
                this.last_click = -1;
                return
            }

            this.last_click = square_clicked;
            Requests.square_clicked_request(square_clicked);
        };
    }

    //sets up all events
    setup_events() {
        this.init_square_clicked_event();
        this.init_keydown_event();
    }
}