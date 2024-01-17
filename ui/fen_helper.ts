import {chess_board} from "./board/chessboard";

export class fen_helper {
    static chess_fen: string;

    parse_fen() {
        // document.getElementById('chess_board').innerHTML = chess_fen.length
        for (let i = 0; i < fen_helper.chess_fen.length; i++) {
            let chr: string = fen_helper.chess_fen[i];

            if (chr === ' ') {
                break;
            }else if(/^\d+$/.test(chr)) {
                chess_board.add_empty(+chr);
                continue;
            }else if(chr === '/') {
                continue;
            }

            // easy way to convert single char to name of a piece for example: 'P' -> 'wP', 'p' -> 'bP'
            chess_board.add_square((chr === chr.toUpperCase() ? "w" : "b") + chr.toUpperCase())
        }
    }
}