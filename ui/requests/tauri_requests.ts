import { invoke } from '@tauri-apps/api/tauri'

import {fen_helper} from "../fen_helper";
import {chess_board} from "../board/chessboard";

export class requests {
    get_board_request = async (): Promise<void> => {
        return new Promise<void>((resolve, reject) => {
            invoke('get_board').then((fen: unknown) => {
                fen_helper.chess_fen = fen as string;
                resolve();
            }).catch(reject);
        });
    }

    square_clicked_request = async(square_clicked: number): Promise<void> => {
        new Promise<void>((resolve, reject) => {
            invoke('select_square', {"square": square_clicked}).then((moves: unknown) => {
                chess_board.unset_movable_squares();
                chess_board.set_squares_movable(moves as Array<number>);
                resolve();
            }).catch(reject);
        });
    }

    undo_move = async(): Promise<void> => {
        new Promise<void>((resolve, reject) => {
            invoke("undo_move", {}).then((move: unknown) => {
                let arr_move = move as Array<number>;
                let start = arr_move[0], dest = arr_move[1];

                chess_board.unset_movable_squares()
                chess_board.undo_move(start, dest);
                resolve();
            }).catch(reject)
        })
    }

    move_piece_request = async(start_sq: number, move_sq: number): Promise<void> => {
        new Promise<void>((resolve, reject) => {
            invoke("move_piece", {"startSq": start_sq, "destSq": move_sq}).then(() => {
                chess_board.unset_movable_squares()
                chess_board.move_piece(start_sq, move_sq);

                resolve();
            }).catch(reject)
        })
    }


}