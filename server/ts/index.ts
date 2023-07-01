import { ID_TO_SOUND } from "./idMap.js";

var backend = new WebSocket("ws://127.0.0.1:3001/ws");

interface ButtonPress {
    button_id: number;
    millis_since_init: number;
}

const playButtonPressSound = (msg: MessageEvent<any>) => {
  let buttonPress: ButtonPress = JSON.parse(msg.data).ButtonPress;
  console.log(buttonPress);

  let elementId: string = ID_TO_SOUND[buttonPress.button_id] ?? 'icq';

  let audioElement: HTMLAudioElement = document.getElementById(elementId) as HTMLAudioElement;

  if (audioElement.paused) {
    audioElement.play();
  } else {
    audioElement.currentTime = 0;
  }
};

export function init() {
  backend.send("Init message from frontend");
  backend.addEventListener("message", playButtonPressSound);
};