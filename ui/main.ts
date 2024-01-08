// TODO: make this OOP!
import {get_board} from "./tauri_requests";
import {fen_helper} from "./fen_helper";

async function main() {
    await get_board();
    load_board();
}

main();

function add_empty(x: number) {
    let chess_board = document.getElementById("chess_board");

    for(let i = 0; i < x; i++) {
        // we need to create a new element every single time, we can't use the same multiple times!
        let empty_div = document.createElement("div");
        empty_div.classList.add('square');

        // append to new empty div to the chess_board
        chess_board.appendChild(empty_div);
    }
}

function add_square(piece: string) {
    // Create a new img element
    let img = document.createElement('img');

    img.src = "assets/" + piece + ".png";
    img.classList.add('square');

    // Append the img at the correct location on the board
    let chess_board = document.getElementById("chess_board");

    // if(parentElement.children.length === 0) {
    chess_board.appendChild(img);
    // return;
    // }
    // parentElement.insertBefore(img, parentElement.children[0]);
}

function load_board() {
    // document.getElementById('chess_board').innerHTML = chess_fen.length
    for (let i = 0; i < fen_helper.chess_fen.length; i++) {
        let chr: string = fen_helper.chess_fen[i];

        if (chr === ' ') {
            break;
        }else if(/^\d+$/.test(chr)) {
            // '+' operator converts chr to a num
            console.log(+chr)
            add_empty(+chr);
            continue;
        }

        switch (chr) {
            case 'P':
                add_square("wP");
                break;
            case 'N':
                add_square("wN");
                break;
            case 'B':
                add_square("wB");
                break;
            case 'R':
                add_square("wR");
                break;
            case 'Q':
                add_square("wQ");
                break;
            case 'K':
                add_square("wK");
                break;
            case 'p':
                add_square("bP");
                break;
            case 'n':
                add_square("bN");
                break;
            case 'b':
                add_square("bB");
                break;
            case 'r':
                add_square("bR");
                break;
            case 'q':
                add_square("bQ");
                break;
            case 'k':
                add_square("bK");
                break;
            // TODO: maybe remove ts-ignore here when finishing up the project, currently this gives me a error
            //  in the IDE which is just annoying
            // @ts-ignore
            case /^\d+$/.test(chr):
                add_empty(chr);
                break;
        }
    }
}