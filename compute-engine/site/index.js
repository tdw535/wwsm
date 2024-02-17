import { Universe, Cell, DisplayScene } from "compute-engine";
import { memory } from "compute-engine/compute_engine_bg";

import * as THREE from 'three';
import { OrbitControls } from './dependencies/OrbitControls.js';





const renderer = new THREE.WebGLRenderer();
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);


var height_js = [];
height_js.push(0.0);
var height_js_ptr;

const display_scene = DisplayScene.new(1, 1, height_js);
display_scene.set_delt(0.0001);
let display_scene_initialized = false;


const scene = new THREE.Scene();

const camera = new THREE.PerspectiveCamera(20, window.innerWidth / window.innerHeight, 1, 1000);

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

let xg = [];
let yg = [];
let zg = [];




//controls.update() must be called after any manual changes to the camera's transform
camera.position.set(1.0, -25, 12);
camera.rotation.set(1.14, 0.027, -0.06);
controls.update();





function updateDiffusive() {

  if (display_scene_initialized) {
    let row = display_scene.get_row();
    let col = display_scene.get_col();
    // display_scene.update();
    // var new_zg = display_scene.height_accessible_js();
    display_scene.update();
    // new_zg = display_scene.height_accessible_js();


    // const new_zg_vec = new Float64Array(memory.buffer, new_zg, row * col);



    while (scene.children.length > 0) {
      scene.remove(scene.children[0]);
    }

    var points = [];
    for (let ii = 0; ii < row * col; ii++) {
      points.push(new THREE.Vector3(xg[ii], yg[ii], height_js[ii]));
    }

    var geometry = new THREE.BufferGeometry().setFromPoints(points);
    var dotMaterial = new THREE.PointsMaterial({ size: 3, sizeAttenuation: false });
    var dot = new THREE.Points(geometry, dotMaterial);
    scene.add(dot);
  }
}

function getxyz() {
  let endpoint = "http://localhost:5057/" + 'xyz';
  return fetch(endpoint, { method: 'GET' })
    .then((response) => response.json())
    .then((data) => {
      console.log("value for", 'xyz');
      console.log(data);
      xg = data.xg;
      yg = data.yg;
      zg = data.zg;
      let row = data.row;
      let col = data.col;
      height_js = data.zg;
      display_scene.init(data.row, data.col, data.zg);
      display_scene_initialized = true;
      console.log("Initialized");



      // let counter = 0;
      // while (counter < 1000) {
      height_js_ptr = display_scene.height_accessible_js()
      height_js = new Float64Array(memory.buffer, height_js_ptr, row * col);

      // console.log(new_zg_vec[10]);

      while (scene.children.length > 0) {
        scene.remove(scene.children[0]);
      }
      var points = [];

      for (let ii = 0; ii < row * col; ii++) {
        // console.log(new_zg_vec[ii]);
        points.push(new THREE.Vector3(xg[ii], yg[ii], height_js[ii]));
      }
      var geometry = new THREE.BufferGeometry().setFromPoints(points);
      var dotMaterial = new THREE.PointsMaterial({ size: 3, sizeAttenuation: false });
      var dot = new THREE.Points(geometry, dotMaterial);
      scene.add(dot);
      //   display_scene.simple_diffusive_update();
      //   // console.log('update');
      //   counter = counter + 1;
      //   // animate();
      // }

      // updateDiffusive(display_scene);
      // console.log("Out of scope");
    });
}

getxyz();
// for (let i = 0; i < 1000; i++) {
//   updateDiffusive(display_scene);
//   animate();
// }


console.log("display_scene_initialized", display_scene_initialized);


animate();


var zero = performance.now();
requestAnimationFrame(animate);
function animate() {
  const value = (performance.now() - zero) / 10;
  if (value > 1 && value < 50) {
    if (display_scene_initialized) {
      updateDiffusive();
      // console.log(display_scene.get_row());
      // console.log(display_scene.get_col());
      // updateDiffusive();
      // console.log("updated");
    }
    zero = document.timeline.currentTime;
  }
  renderer.render(scene, camera);
  requestAnimationFrame((t) => animate());
}


// function animate() {

//   setTimeout(function () {
//     updateDiffusive();

//     // requestAnimationFrame(animate);

//   }, 10);

//   // requestAnimationFrame(animate);

//   // cube.rotation.x += 0.01;
//   // cube.rotation.y += 0.01;

//   renderer.render(scene, camera);
// }
// requestAnimationFrame(animate);


// for (let i = 0; i < 1000; i++) {
//   updateDiffusive(display_scene);
//   animate();
// }

// getInitVal('x');
// getInitVal('y');
// getInitVal('z');


// console.log("Hello");

// // Give the canvas room for all of our cells and a 1px border
// // around each of them.
// const canvas = document.getElementById("game-of-life-canvas");
// canvas.height = (CELL_SIZE + 1) * height + 1;
// canvas.width = (CELL_SIZE + 1) * width + 1;


