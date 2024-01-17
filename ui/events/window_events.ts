import {requests} from "../requests/tauri_requests";

let Requests = new requests();

export class window_events {
    /*
    calculates which square the user has clicked based on mouseEvent
     */
    square_clicked(event: MouseEvent) {
        // 100 is in this case the size of the screen / 8
        // TODO make 100 not hardcoded and based on the screen_size
        return Math.floor(event.y/100)*8+Math.floor(event.x/100)
    }

    setup_events() {
        window.onclick = (event: MouseEvent) => {
            Requests.square_clicked_request(this.square_clicked(event));
        };
    }
}