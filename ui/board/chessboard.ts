export class chess_board {
    static html_chess_board = document.getElementById("chess_board");

    static add_empty(x: number) {
        for(let i = 0; i < x; i++) {
            // we need to create a new element every single time, we can't use the same multiple times!
            let empty_div = document.createElement("div");
            empty_div.classList.add('square');

            // append to new empty div to the chess_board
            chess_board.html_chess_board.appendChild(empty_div);
        }
    }

    static add_square(piece: string) {
        // Create a new img element
        let img = document.createElement('img');

        img.src = "assets/" + piece + ".png";
        img.classList.add('square');

        // Append the img at the correct location on the board


        // if(parentElement.children.length === 0) {
        chess_board.html_chess_board.appendChild(img);
        // return;
        // }
        // parentElement.insertBefore(img, parentElement.children[0]);
    }
}