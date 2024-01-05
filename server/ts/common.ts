export interface ButtonPress {
    button_id: number;
    millis_since_init: number;
}

export function clearTable(table: HTMLTableElement) {
    table.innerHTML = table.rows[0].innerHTML;
}

export function createTableRow(name: string, time: number): HTMLTableRowElement {
    const newRow = document.createElement("tr");

    const newName = document.createElement("td");
    newName.appendChild(document.createTextNode(name));

    const newTime = document.createElement("td");
    newTime.appendChild(document.createTextNode(time.toFixed(0)));

    newRow.appendChild(newName);
    newRow.appendChild(newTime);

    return newRow;
}

export function playAudio(audioElement: HTMLAudioElement) {
  if (audioElement.paused) {
    audioElement.play();
  } else {
    audioElement.currentTime = 0;
  }
}

export function setAllButtons(backend: WebSocket, on: boolean) {
  for (let i = 0; i < 6; i++) {
    backend.send(
      `{"LedUpdate": {"button_id": ${i}, "on": ${on}}}`
    )
  }
}