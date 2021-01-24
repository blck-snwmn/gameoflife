import { GameBoard, Cell } from "gameoflife";
import { memory } from "gameoflife/gameoflife_bg";

const gridColor = "#CCCCCC";

const cellSize = 20;
const cellNum = 40;
const board = GameBoard.new(cellNum, cellNum);
const [w, h] = [board.width(), board.height()];

/** @type {HTMLCanvasElement} */
const canvas = document.getElementById("canvas");
// grid size 考慮
canvas.height = h * (cellSize + 1) + 2;
canvas.width = w * (cellSize + 1) + 2;

const ctx = canvas.getContext('2d');
const loop = () => {
  ctx.beginPath();
  board.tick();

  const cellsPtr = board.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, w * h);
  // draw grid
  ctx.strokeStyle = 'gray';
  ctx.lineWidth = 1;
  for (let i = 0; i <= w; i++) {
    ctx.moveTo(i * (cellSize + 1), 0);
    ctx.lineTo(i * (cellSize + 1), (cellSize + 1) * h);
  }
  for (let j = 0; j <= h; j++) {
    ctx.moveTo(0, j * (cellSize + 1));
    ctx.lineTo((cellSize + 1) * w, j * (cellSize + 1));
  }
  // draw cell
  cells.forEach((element, index) => {
    const col = index % w;
    const row = Math.floor(index / w);
    ctx.fillStyle = element === Cell.Dead
      ? "#FFFFFF"
      : "#000000";

    ctx.fillRect(
      col * (cellSize + 1) + 1,
      row * (cellSize + 1) + 1,
      cellSize,
      cellSize
    );
  });
  ctx.stroke();

  requestAnimationFrame(loop);
}
requestAnimationFrame(loop);
