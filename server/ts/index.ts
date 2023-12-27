import { ID_TO_SOUND, ID_TO_NAME } from "./idMap.js";

var backend = new WebSocket(`ws://${location.host}/ws`);
var already_pressed_set: Set<number> = new Set<number>();
var randomCountdownMs: number = 0;
var first = true;

interface ButtonPress {
    button_id: number;
    millis_since_init: number;
}

const handleIncomingPress = (msg: MessageEvent<any>) => {
  console.log("Received message:", msg);

  let buttonPress: ButtonPress = JSON.parse(msg.data).ButtonPress;
  let audioName: string = ID_TO_SOUND[buttonPress.button_id] ?? 'icq';
  let buttonName: string = ID_TO_NAME[buttonPress.button_id] ?? 'Unknown';

  if (already_pressed_set.has(buttonPress.button_id)) {
    return;
  }
  already_pressed_set.add(buttonPress.button_id);


  // TODO: Change will come here
  const reactionTime = buttonPress.millis_since_init - randomCountdownMs;
  const tooEarly = reactionTime <= 0;

  const element = createTableRow(buttonName, reactionTime);

  if (tooEarly) {
    document.getElementById('too-early-table')?.appendChild(element);
    playAudio(document.getElementById('boowomp') as HTMLAudioElement);
  } else {
    document.getElementById('leader-table')?.appendChild(element);
    if (first) {
      backend.send(
        `{"LedUpdate": {"button_id": ${buttonPress.button_id}, "on": true}}`
      )
      first = false;
    }
    playAudio(document.getElementById(audioName) as HTMLAudioElement);

  }
};

backend.addEventListener("message", handleIncomingPress);

export function initReactionGame() {
    let triggerElement = document.getElementById('trigger') as HTMLElement;
    triggerElement.style.visibility = 'hidden';

    setAllButtons(backend, false);

    first = true;

    already_pressed_set.clear()
    const leaderTable = document.getElementById('leader-table') as HTMLTableElement;
    clearTable(leaderTable);
    const tooEarlyTable = document.getElementById('too-early-table') as HTMLTableElement;
    clearTable(tooEarlyTable);

    // Random delay between 2 and 5 seconds in ms.
    randomCountdownMs = 2000 + Math.floor(Math.random() * 3000);
    console.log("New random delay is ", randomCountdownMs);
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

function createTableRow(name: string, time: number): HTMLTableRowElement {
    const newRow = document.createElement("tr");

    const newName = document.createElement("td");
    newName.appendChild(document.createTextNode(name));

    const newTime = document.createElement("td");
    newTime.appendChild(document.createTextNode(time.toFixed(0)));

    newRow.appendChild(newName);
    newRow.appendChild(newTime);

    return newRow;
}

function playAudio(audioElement: HTMLAudioElement) {
  if (audioElement.paused) {
    audioElement.play();
  } else {
    audioElement.currentTime = 0;
  }
}

function setAllButtons(backend: WebSocket, on: boolean) {
  for (let i = 0; i < 6; i++) {
    backend.send(
      `{"LedUpdate": {"button_id": ${i}, "on": ${on}}}`
    )
  }
}