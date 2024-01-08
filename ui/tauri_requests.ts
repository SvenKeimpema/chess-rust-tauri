import { invoke } from '@tauri-apps/api/tauri'

import {fen_helper} from "./fen_helper";

export let get_board = async (): Promise<void> => {
    return new Promise<void>((resolve, reject) => {
        invoke('get_board').then((fen: unknown) => {
            fen_helper.chess_fen = fen as string;
            resolve();
        }).catch(reject);
    });
}