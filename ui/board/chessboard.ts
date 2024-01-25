export class chess_board {
    static html_chess_board = document.getElementById("chess_board");

    static get_color(id: number): "darkgrey" | "white" {
        let r: number = id / 8;
        let c: number = id % 8;

        return Math.floor((c+r) % 2) == 0 ? "darkgrey" : "white"
    }

    static create_empty_square(id: number) {
        let empty_div = document.createElement("div");
        empty_div.classList.add('square');
        empty_div.id = id;
        empty_div.style.backgroundColor = this.get_color(id);
        return empty_div
    }

    static add_empty_squares(x: number, id: number) {
        for(let i = 0; i < x; i++) {
            // we need to create a new element every single time, we can't use the same multiple times!
            let empty_div = this.create_empty_square(id+i)
            // append to new empty div to the chess_board
            chess_board.html_chess_board.appendChild(empty_div);
        }
    }

    static add_piece(piece: string, id: number) {
        // Create a new img element
        let img = document.createElement('img');

        img.src = "assets/" + piece + ".png";
        img.classList.add('square');
        img.id = id
        img.style.backgroundColor = this.get_color(id);

        // Append the img at the correct location on the board

        chess_board.html_chess_board.appendChild(img);
    }

    static unset_movable_squares() {
        let squares = document.getElementsByClassName('movable')

        while(squares.length != 0) {
            squares[0].classList.remove('movable')
        }
    }

    static set_squares_movable(moves: Array<number>) {
        let squares = document.getElementsByClassName('square')

        moves.forEach( (move: number) => {
            squares[move].classList.add('movable')
        })
    }

    static move_piece(start_sq: number, end_sq: number) {
        let squares = document.getElementsByClassName('square');
        let chess_board = document.getElementById('chess_board');
        let empty_div = this.create_empty_square(start_sq);

        let start_piece = chess_board.children[start_sq];
        start_piece.style.backgroundColor = this.get_color(end_sq);


        chess_board.removeChild(squares[start_sq]);
        chess_board.children[start_sq-1].insertAdjacentElement("afterEnd", empty_div);

        chess_board.removeChild(squares[end_sq])
        chess_board.children[end_sq-1].insertAdjacentElement("afterEnd", start_piece);
    }


    /*
    start_sq: square where the piece is currently on.
    <br>end_sq: square where the piece needs to go.
    */
    static undo_move(start_sq: number, end_sq: number): void {
        if(start_sq == end_sq) return;

        this.move_piece(start_sq, end_sq);

    }
}