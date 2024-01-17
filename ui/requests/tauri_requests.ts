import { invoke } from '@tauri-apps/api/tauri'

import {fen_helper} from "../fen_helper";

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
        return new Promise<void>((resolve, reject) => {
            invoke('select_square', {"square": square_clicked}).then((code: unknown) => {
                console.log(code);
                resolve();
            }).catch(reject);
        });
    }
}