import { Universe, Cell, DisplayScene } from "compute-engine";
import { memory } from "compute-engine/compute_engine_bg";

import * as THREE from 'three';
import { OrbitControls } from './dependencies/OrbitControls.js';





const renderer = new THREE.WebGLRenderer();
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

const scene = new THREE.Scene();

const camera = new THREE.PerspectiveCamera(45, window.innerWidth / window.innerHeight, 1, 10000);

const controls = new OrbitControls(camera, renderer.domElement);

// const geometry = new THREE.BoxGeometry(1, 1, 1);
// const material = new THREE.LineBasicMaterial({ color: 0x00ff00 });
// const cube = new THREE.Mesh(geometry, material);
// scene.add(cube);

var points = [];

points.push(new THREE.Vector3(0, 0, 1));
points.push(new THREE.Vector3(1, 0, 1));
points.push(new THREE.Vector3(1, 1, 1));
points.push(new THREE.Vector3(0, 1, 1));
points.push(new THREE.Vector3(1, 0, 0));
points.push(new THREE.Vector3(1, 1, 0));
points.push(new THREE.Vector3(0, 1, 0));
points.push(new THREE.Vector3(0, 0, 0));
var geometry = new THREE.BufferGeometry().setFromPoints(points);
var dotMaterial = new THREE.PointsMaterial({ size: 1, sizeAttenuation: false });
var dot = new THREE.Points(geometry, dotMaterial);
scene.add(dot);


camera.position.z = 5;



//controls.update() must be called after any manual changes to the camera's transform
camera.position.set(0, 20, 100);
controls.update();

function animate() {
  requestAnimationFrame(animate);

  // cube.rotation.x += 0.01;
  // cube.rotation.y += 0.01;

  renderer.render(scene, camera);
}

animate();




// function getInitVal() {
//   return fetch("http://localhost:5057/a", { method: 'GET' })
//     .then((response) => response.json())
//     .then((data) => {
//       console.log("values are");
//       console.log(data);
//       const display_scene = DisplayScene.new(data.row, data.col, data.vals);
//       console.log("Got init val");
//       let row = display_scene.get_row();
//       let col = display_scene.get_col();
//       console.log(row);
//       console.log(col);
//       const height_vec_ptr = display_scene.height_accessible_js()
//       const height_vec = new Float64Array(memory.buffer, height_vec_ptr, 4 * 6);
//       // console.log("Display scene");
//       // console.log(height_ds);
//       console.log("Display scene height");
//       for (let j = 0; j < 24; j++) {
//         console.log(height_vec[j]);
//       }

//     });
// }



console.log("Hello");

// // Give the canvas room for all of our cells and a 1px border
// // around each of them.
// const canvas = document.getElementById("game-of-life-canvas");
// canvas.height = (CELL_SIZE + 1) * height + 1;
// canvas.width = (CELL_SIZE + 1) * width + 1;


