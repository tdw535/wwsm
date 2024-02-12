import { Universe, Cell, DisplayScene } from "compute-engine";
import { memory } from "compute-engine/compute_engine_bg";




const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const epsilon = 1e-3;

// Construct the universe, and get its width and height.
const universe = Universe.new(60, 60);
const width = universe.width();
const height = universe.height();



function getInitVal() {
  return fetch("http://localhost:5057/a", { method: 'GET' })
    .then((response) => response.json())
    .then((data) => {
      console.log("values are");
      console.log(data);
    });
}




getInitVal();

const display_scene = DisplayScene.new(4, 4);
// let success = display_scene.get_init_val();
// let row = display_scene.get_row();
// let col = display_scene.get_col();
console.log("Got init val");
// console.log(success);
// console.log(row);
// console.log(col);


// try {
//   display_scene.readin();
// }
// catch (error) {
//   console.error(error);
// }
// const height_ds = display_scene.height_zero()
const height_vec_ptr = display_scene.height_accessible_js()
const height_vec = new Float64Array(memory.buffer, height_vec_ptr, 4 * 6);
// console.log("Display scene");
// console.log(height_ds);
console.log("Display scene height");
for (let j = 0; j < 24; j++) {
  console.log(height_vec[j]);
}

console.log("Hello");

// // Give the canvas room for all of our cells and a 1px border
// // around each of them.
// const canvas = document.getElementById("game-of-life-canvas");
// canvas.height = (CELL_SIZE + 1) * height + 1;
// canvas.width = (CELL_SIZE + 1) * width + 1;

// const ctx = canvas.getContext('2d');
// let animationId = null;

// const playPauseButton = document.getElementById("play-pause");
// const nextPageButton = document.getElementById("next-page");

// nextPageButton.textContent = "Next Page";
// // nextPageButton.addEventListener("click", async () => {
// //   location.href = 'scorePage.html';
// // });



// const play = () => {
//   playPauseButton.textContent = "⏸";
//   renderLoop();
// };

// const pause = () => {
//   playPauseButton.textContent = "▶";
//   cancelAnimationFrame(animationId);
//   animationId = null;
// };

// const isPaused = () => {
//   return animationId === null;
// };

// playPauseButton.addEventListener("click", async () => {
//   if (isPaused()) {
//     play();
//   } else {
//     pause();
//   }
// });

// canvas.addEventListener("click", event => {
//   const boundingRect = canvas.getBoundingClientRect();

//   const scaleX = canvas.width / boundingRect.width;
//   const scaleY = canvas.height / boundingRect.height;

//   const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
//   const canvasTop = (event.clientY - boundingRect.top) * scaleY;

//   const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
//   const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

//   universe.toggle_cell(row, col);

//   drawGrid();
//   drawCells();
// });


// // This function is the same as before, except the
// // result of `requestAnimationFrame` is assigned to
// // `animationId`.
// const renderLoop = () => {
//   drawGrid();
//   drawCells();

//   universe.evolve();


//   animationId = requestAnimationFrame(renderLoop);
// };

// const drawGrid = () => {
//   ctx.beginPath();
//   ctx.strokeStyle = GRID_COLOR;

//   for (let i = 0; i <= width; i++) {
//     ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
//     ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
//   }

//   for (let j = 0; j <= height; j++) {
//     ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
//     ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
//   }

//   ctx.stroke();
// }

// const getIndex = (row, column) => {
//   return row * width + column;
// };

// const drawCells = () => {

//   const cellsPtr = universe.cells_accessible();
//   const cells = new Float64Array(memory.buffer, cellsPtr, width * height);


//   ctx.beginPath();

//   for (let row = 0; row < height; row++) {
//     for (let col = 0; col < width; col++) {

//       const idx = getIndex(row, col);
//       if (row === 0 && col === 0) {
//         console.log("row, col toggled", row, col, cells[idx], idx);
//       }

//       ctx.fillStyle = cells[idx] > epsilon
//         ? ALIVE_COLOR
//         : DEAD_COLOR;

//       ctx.fillRect(
//         col * (CELL_SIZE + 1) + 1,
//         row * (CELL_SIZE + 1) + 1,
//         CELL_SIZE,
//         CELL_SIZE
//       );
//     }
//   }

//   ctx.stroke();
// };




// play();

