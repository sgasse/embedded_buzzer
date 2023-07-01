import { ID_TO_SOUND, ID_TO_NAME } from "./idMap.js";
import { createTableRow, ALREADY_PRESSED_SET } from "./reactionGame.js";

var backend = new WebSocket("ws://127.0.0.1:3001/ws");

interface ButtonPress {
    button_id: number;
    millis_since_init: number;
    millis_reaction: number;
}

const handleIncomingPress = (msg: MessageEvent<any>) => {
  console.log("Received message:", msg);

  let buttonPress: ButtonPress = JSON.parse(msg.data).ButtonPress;
  let audioName: string = ID_TO_SOUND[buttonPress.button_id] ?? 'icq';
  let buttonName: string = ID_TO_NAME[buttonPress.button_id] ?? 'Unknown';

  if ALREADY_PRESSED_SET.has(buttonPress.button_id) {
    return;
  }
  ALREADY_PRESSED_SET.add(buttonPress.button_id);

  const tooEarly = buttonPress.millis_reaction <= 0;

  const element = createTableRow(audioName, buttonPress.millis_reaction);
  const table = tooEarly ? document.getElementById('too-early-table') : document.getElementById('leader-table');
  table?.appendChild(element);

  let audioElement: HTMLAudioElement = (tooEarly ? document.getElementById('boowomp') : document.getElementById(audioName)) as HTMLAudioElement;

  if (audioElement.paused) {
    audioElement.play();
  } else {
    audioElement.currentTime = 0;
  }
};

backend.addEventListener("message", handleIncomingPress);



export function initReactionGame() {
    let triggerElement = document.getElementById('trigger') as HTMLElement;
    triggerElement.style.visibility = 'hidden';

    ALREADY_PRESSED_SET.clear()
    const leaderTable = document.getElementById('leader-table') as HTMLTableElement;
    clearTable(leaderTable);
    const tooEarlyTable = document.getElementById('too-early-table') as HTMLTableElement;
    clearTable(tooEarlyTable);

    let randomCountdownMs: number = Math.floor(Math.random() * 3000.0);
    backend.send(`{"InitReactionGame":${randomCountdownMs}}`);

    setTimeout((_: any) => {
        let triggerElement = document.getElementById('trigger') as HTMLElement;
        triggerElement.style.visibility = 'visible';

        backend.send("Countdown finished");

    }, randomCountdownMs);
}

function clearTable(table: HTMLTableElement) {
    table.innerHTML = table.rows[0].innerHTML;
}