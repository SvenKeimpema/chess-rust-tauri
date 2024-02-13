export class Game {
    /*
    If code=-1 nothing happens, code=0 the game is drawn and needs to be reset, code=1 the game is won and needs to be
    reset. (only reset after uses chooses to replay the game!)
     */
    static parse_win_code(code: number) {
        let game_ended_text_label: HTMLElement | null = document.getElementById("game-ended-text");

        if (game_ended_text_label === null) {
            return;
        }

        switch (code) {
            case 0:
                game_ended_text_label.innerHTML = "Game has been drawn!";
                game_ended_text_label.style.display = "block";
                break;
            case 1:
                game_ended_text_label.innerHTML = "White Won!";
                game_ended_text_label.style.display = "block";
                break;
            case 2:
                game_ended_text_label.innerHTML = "Black Won!";
                game_ended_text_label.style.display = "block";
                break;
            default:
                break;
        }
    }
}