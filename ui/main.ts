import {requests} from "./requests/tauri_requests";
import {fen_helper} from "./fen_helper";
import {window_events} from "./events/window_events";

let Requests = new requests();
let FenHelper = new fen_helper();
let WindowEvents = new window_events();

async function main() {
    await Requests.get_board_request();
    FenHelper.parse_fen();
    WindowEvents.setup_events();
}

main().then(() => console.log("setup done."));
