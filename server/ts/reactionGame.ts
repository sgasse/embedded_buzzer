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

export var ALREADY_PRESSED_SET = new Set<number>();
