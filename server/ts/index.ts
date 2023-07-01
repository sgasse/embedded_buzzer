
var backend = new WebSocket("ws://127.0.0.1:3001/ws");

const player = (msg: MessageEvent<any>) => {
  var createSound: HTMLAudioElement = document.getElementById("create") as HTMLAudioElement;
  console.log(msg);

  if (createSound.paused) {
    createSound.play();
  } else {
    createSound.currentTime = 0;
  }
};

const init = () => {
  backend.send("Init message from frontend");
  backend.addEventListener("message", player);
};