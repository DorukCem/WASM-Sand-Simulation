import { Universe, CellType } from "wasm-sand-sim";
import fps_logger from "./measure_fps.js"
// We can directly access WebAssembly's linear memory via memory
import { memory } from "../pkg/wasm_sand_sim_bg.wasm";

// ! I want cell size to be determined by how big the user screen is
const CELL_SIZE = 15; // px 
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const SAND_COLOR = "#000000";
const WATER_COLOR = "#0000FF";
const CURSOR_SIZE = CELL_SIZE*4;
const CURSOR_BORDER_WIDTH = 4;
const CURSOR_COLOR = "#000000"

const cellColors = {
  [CellType.Dead]: DEAD_COLOR,
  [CellType.Water]: WATER_COLOR,
  [CellType.Sand]: SAND_COLOR,
  // Add more cell types and colors as needed
};

// Construct the universe, and get its width and height.
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("sand-sim-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');
const playPauseButton = document.getElementById("play-pause");

let mousePos = {x: 0, y: 0}
let mouseGridPos = {row: 0, col: 0}
let being_held = false
let selected_element = CellType.Sand
let animationId = null;

const renderLoop = () => {
  //fps_logger.render();
  setCell();
  drawGrid();
  drawCells();
  drawCursor(mousePos);
  
  universe.tick();

  animationId = requestAnimationFrame(renderLoop);
};

const isPaused = () => {
  return animationId === null;
};

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

const getIndex = (row, column) => {
  return row * width + column;
};

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = cellColors[cells[idx]];

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};

function drawCursor(pos) {
  ctx.strokeStyle = CURSOR_COLOR;
  ctx.lineWidth = CURSOR_BORDER_WIDTH;
  ctx.beginPath();
  ctx.arc(pos.x, pos.y, CURSOR_SIZE/2, 0, 2*Math.PI);
  ctx.stroke();
}

function getMousePos(evt) {
  var rect = canvas.getBoundingClientRect();
  return {
    x: evt.clientX - rect.left,
    y: evt.clientY - rect.top
  };
}

const play = () => {
  playPauseButton.textContent = "⏸";
  renderLoop();
};

const pause = () => {
  playPauseButton.textContent = "▶";
  cancelAnimationFrame(animationId);
  animationId = null;
};

playPauseButton.addEventListener("click", event => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

function setCell() {
  if (being_held){
    const {row, col} = mouseGridPos;
    if (row >= 0 && row < height && col >= 0 && col < height)
    {
      universe.set_cell(row, col, selected_element);
    } 
  }
}

canvas.addEventListener("mousemove", event => {
  mousePos =  getMousePos(event);
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

  mouseGridPos = {row, col}
});

canvas.addEventListener('mousedown', function() {
  being_held = true;
})

canvas.addEventListener('mouseleave', function() {
  being_held = false;
})

canvas.addEventListener('mouseup', function() {
  being_held = false;
})

document.addEventListener("keydown", (event) => {
  if (event.key === "s" || event.key === "S") {
    selected_element = CellType.Sand
  } 
  else if (event.key === "w" || event.key === "W") {
    selected_element = CellType.Water
  }
  console.log(selected_element)
});

// ------------ executes once
drawGrid();
drawCells();
// ------------
play();