import { invoke } from '@tauri-apps/api/tauri'

import {fen_helper} from "../fen_helper";
import {chess_board} from "../board/chessboard";
import {Game} from "../game";

export class requests {
    // gets the fen set in rust
    get_board_request = async (): Promise<void> => {
        return new Promise<void>((resolve, reject) => {
            invoke('get_board').then((fen: unknown) => {
                fen_helper.chess_fen = fen as string;
                resolve();
            }).catch(reject);
        });
    }

    // gets all squares the user can move to if a square is clocked
    square_clicked_request = async(square_clicked: number): Promise<void> => {
        new Promise<void>((resolve, reject) => {
            invoke('select_square', {"square": square_clicked}).then((moves: unknown) => {
                chess_board.unset_movable_squares();
                chess_board.set_squares_movable(moves as Array<number>);
                resolve();
            }).catch(reject);
        });
    }

    // undo a chess move
    undo_move = async(): Promise<void> => {
        new Promise<void>((resolve, reject) => {
            invoke("undo_move", {}).then((move: unknown) => {
                let arr_move = move as Array<number>;
                let start = arr_move[0], dest = arr_move[1];

                chess_board.unset_movable_squares();
                chess_board.undo_move(start, dest);
                resolve();
            }).catch(reject);
        })
    }

    // check if the user has won the game
    check_game_won_request = async(): Promise<void> => {
        new Promise<void>((resolve, reject) => {
            invoke("check_game_won", {}).then((code: any) => {
                Game.parse_win_code(code as number);
                resolve();
            }).catch(reject);
        })
    }

    // send to tauri that a piece has moved
    move_piece_request = async(start_sq: number, move_sq: number): Promise<void> => {
        new Promise<void>((resolve, reject) => {
            invoke("move_piece", {"startSq": start_sq, "destSq": move_sq}).then(() => {
                chess_board.unset_movable_squares();
                chess_board.move_piece(start_sq, move_sq);

                // after moving every single piece we will need to check if we won the game!
                this.check_game_won_request();
                resolve();
            }).catch(reject);
        })
    }
}