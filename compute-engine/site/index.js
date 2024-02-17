import { Universe, Cell, DisplayScene } from "compute-engine";
import { memory } from "compute-engine/compute_engine_bg";

import * as THREE from 'three';
import { OrbitControls } from './dependencies/OrbitControls.js';





const renderer = new THREE.WebGLRenderer();
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

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






//controls.update() must be called after any manual changes to the camera's transform
camera.position.set(1.0, -25, 12);
camera.rotation.set(1.14, 0.027, -0.06);
controls.update();





// function updateDiffusive(display_scene_) {
//   console.log("Update Diffusive");
//   // display_scene_.simple_diffusive_update();
//   console.log("Updated next step");

//   const new_zg = display_scene_.height_accessible_js();
//   console.log("Got zg");

//   const new_zg_vec = new Float64Array(memory.buffer, new_zg, 4 * 6);

//   while (scene.children.length > 0) {
//     scene.remove(scene.children[0]);
//   }
//   let row = display_scene_.get_row();
//   console.log("got row", row);
//   let col = display_scene_.get_col();
//   console.log("got col", col);
//   var points = [];
//   for (let ii = 0; ii < row * col; ii++) {
//     points.push(new THREE.Vector3(xg[ii], yg[ii], new_zg_vec[ii]));
//   }

//   var geometry = new THREE.BufferGeometry().setFromPoints(points);
//   var dotMaterial = new THREE.PointsMaterial({ size: 3, sizeAttenuation: false });
//   var dot = new THREE.Points(geometry, dotMaterial);
//   scene.add(dot);
// }

function getxyz() {
  let endpoint = "http://localhost:5057/" + 'xyz';
  return fetch(endpoint, { method: 'GET' })
    .then((response) => response.json())
    .then((data) => {
      console.log("value for", 'xyz');
      console.log(data);
      let xg = data.xg;
      let yg = data.yg;
      let zg = data.zg;
      let row = data.row;
      let col = data.col;
      const display_scene = DisplayScene.new(data.row, data.col, data.zg);
      console.log("Got init val");



      let counter = 0;
      while (counter < 1000) {
        const new_zg = display_scene.height_accessible_js()
        const new_zg_vec = new Float64Array(memory.buffer, new_zg, row * col);

        // console.log(new_zg_vec[10]);

        while (scene.children.length > 0) {
          scene.remove(scene.children[0]);
        }
        var points = [];

        for (let ii = 0; ii < row * col; ii++) {
          // console.log(new_zg_vec[ii]);
          points.push(new THREE.Vector3(xg[ii], yg[ii], new_zg_vec[ii]));
        }

        let drow = display_scene.get_row();
        // console.log("got row", drow);
        let dcol = display_scene.get_col();
        // console.log("got col", dcol);

        var geometry = new THREE.BufferGeometry().setFromPoints(points);
        var dotMaterial = new THREE.PointsMaterial({ size: 3, sizeAttenuation: false });
        var dot = new THREE.Points(geometry, dotMaterial);
        scene.add(dot);
        display_scene.simple_diffusive_update();
        // console.log('update');
        counter = counter + 1;
        // animate();
      }

      // updateDiffusive(display_scene);
      // console.log("Out of scope");
    });
}




getxyz();
animate();



function animate() {
  requestAnimationFrame(animate);

  // cube.rotation.x += 0.01;
  // cube.rotation.y += 0.01;

  renderer.render(scene, camera);
}

animate();
// getInitVal('x');
// getInitVal('y');
// getInitVal('z');


console.log("Hello");

// // Give the canvas room for all of our cells and a 1px border
// // around each of them.
// const canvas = document.getElementById("game-of-life-canvas");
// canvas.height = (CELL_SIZE + 1) * height + 1;
// canvas.width = (CELL_SIZE + 1) * width + 1;


