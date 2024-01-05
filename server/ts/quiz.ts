import { ButtonPress, clearTable, createTableRow, playAudio, setAllButtons } from "./common.js";
import { ID_TO_SOUND, ID_TO_NAME } from "./idMap.js";

var backend = new WebSocket(`ws://${location.host}/ws`);
var already_pressed_set: Set<number> = new Set<number>();
var disqualified_set: Set<number> = new Set<number>();
var randomCountdownMs: number = 0;
var firstNumber: number | null = null;

var myChart;
var chartData;

const handleIncomingPress = (msg: MessageEvent<any>) => {
  console.log("Received message:", msg);

  let buttonPress: ButtonPress = JSON.parse(msg.data).ButtonPress;
  let audioName: string = ID_TO_SOUND[buttonPress.button_id] ?? 'icq';
  let buttonName: string = ID_TO_NAME[buttonPress.button_id] ?? 'Unknown';

  if (already_pressed_set.has(buttonPress.button_id) || disqualified_set.has(buttonPress.button_id)) {
    return;
  }
  already_pressed_set.add(buttonPress.button_id);

  const reactionTime = buttonPress.millis_since_init - randomCountdownMs;
  const tooEarly = reactionTime <= 0;

  const element = createTableRow(buttonName, reactionTime);

  if (tooEarly) {
    document.getElementById('too-early-table')?.appendChild(element);
    playAudio(document.getElementById('boowomp') as HTMLAudioElement);
  } else {
    document.getElementById('leader-table')?.appendChild(element);
    if (firstNumber == null) {
      backend.send(
        `{"LedUpdate": {"button_id": ${buttonPress.button_id}, "on": true}}`
      )
      firstNumber = buttonPress.button_id;
    }
    playAudio(document.getElementById(audioName) as HTMLAudioElement);

    chartData[buttonPress.button_id] = reactionTime;
    updateChartData(chartData);

  }
};

backend.addEventListener("message", handleIncomingPress);

export function continueRound() {
  let triggerElement = document.getElementById('trigger') as HTMLElement;
  triggerElement.style.visibility = 'hidden';

  setAllButtons(backend, false);

  if (firstNumber != null) {
    disqualified_set.add(firstNumber);

    let buttonName: string = ID_TO_NAME[firstNumber] ?? 'Unknown';
    const element = createTableRow(buttonName, 0);
    document.getElementById('disqualified-table')?.appendChild(element);
  }
  firstNumber = null;

  chartData = [0, 0, 0, 0, 0, 0, 0];
  updateChartData(chartData);

  already_pressed_set.clear()
  const leaderTable = document.getElementById('leader-table') as HTMLTableElement;
  clearTable(leaderTable);
  const tooEarlyTable = document.getElementById('too-early-table') as HTMLTableElement;
  clearTable(tooEarlyTable);

  backend.send(`{"InitReactionGame": 0}`);
}

export function initQuizGame() {
    let triggerElement = document.getElementById('trigger') as HTMLElement;
    triggerElement.style.visibility = 'hidden';

    setAllButtons(backend, false);

    firstNumber = null;

    already_pressed_set.clear()
    disqualified_set.clear()

    chartData = [0, 0, 0, 0, 0, 0, 0];
    updateChartData(chartData);

    const leaderTable = document.getElementById('leader-table') as HTMLTableElement;
    clearTable(leaderTable);
    const disqualifiedTable = document.getElementById('disqualified-table') as HTMLTableElement;
    clearTable(disqualifiedTable);

    backend.send(`{"InitReactionGame": 0}`);
}

export function initChart() {
  var chartDom = document.getElementById("main");
  myChart = echarts.init(chartDom);
  updateChartData([0, 0, 0, 0, 0, 0, 0])

}

function updateChartData(numbers) {
  var option;

  option = {
    xAxis: {
      type: "category",
      data: ID_TO_NAME,
      axisLabel: {
        interval: 0,
        rotate: 30
      }
    },
    yAxis: {
      type: "value",
    },
    series: [
      {
        data: numbers,
        type: "bar",
      },
    ],
  };

  option && myChart.setOption(option);
}




        // data: [
        //   120,
        //   {
        //     value: 200,
        //     itemStyle: {
        //       color: "#a90000",
        //     },
        //   },
        //   150,
        //   80,
        //   70,
        //   110,
        //   130,
        // ],