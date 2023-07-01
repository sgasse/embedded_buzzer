import { ID_TO_SOUND } from "./idMap.js";

var backend = new WebSocket("ws://127.0.0.1:3001/ws");

interface ButtonPress {
    button_id: number;
    millis_since_init: number;
    millis_reaction: number;
}

const playButtonPressSound = (msg: MessageEvent<any>) => {
  console.log(msg);
  let buttonPress: ButtonPress = JSON.parse(msg.data).ButtonPress;

  let elementId: string = ID_TO_SOUND[buttonPress.button_id] ?? 'icq';

  let audioElement: HTMLAudioElement = document.getElementById(elementId) as HTMLAudioElement;

  if (audioElement.paused) {
    audioElement.play();
  } else {
    audioElement.currentTime = 0;
  }
};

backend.addEventListener("message", playButtonPressSound);

export function initReactionGame() {
    let triggerElement = document.getElementById('trigger') as HTMLElement;
    triggerElement.style.visibility = 'hidden';

    let randomCountdownMs: number = Math.floor(Math.random() * 3000.0);
    backend.send(`{"InitReactionGame":${randomCountdownMs}}`);

    setTimeout((_: any) => {
        let triggerElement = document.getElementById('trigger') as HTMLElement;
        triggerElement.style.visibility = 'visible';

        backend.send("Countdown finished");

    }, randomCountdownMs);
}