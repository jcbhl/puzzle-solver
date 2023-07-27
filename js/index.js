import("../pkg/index.js")
  .catch(console.error)
  .finally(() => console.log("Successfully imported wasm package"));
import * as THREE from "three";
import { TrackballControls } from "three/examples/jsm/controls/TrackballControls.js";
import { Orientation, Point, wasm_get_points_for_orientation } from "../pkg/index.js";

const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(
  75,
  window.innerWidth / window.innerHeight,
  0.1,
  1000
);

const renderer = new THREE.WebGLRenderer();
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

const controls = new TrackballControls(camera, renderer.domElement);

function draw_piece(piece_points){
  const color = new THREE.Color(Math.random(), Math.random(), Math.random());
  for (const point of piece_points){
    fill_grid_square(point, color)
  }
}

// Expects the point to be already converted from Rust space to JS space
function fill_grid_square(point, color){
  const geometry = new THREE.BoxGeometry(1,1,1);
  const material = new THREE.MeshBasicMaterial({color: color});
  const cube = new THREE.Mesh(geometry, material);
  cube.position.add(point);

  console.log(point.x, point.y, point.z)

  let shift = new THREE.Vector3(0.5, 0.5, 0.5);
  cube.position.add(shift);

  scene.add(cube);

  const edge_geometry = new THREE.EdgesGeometry(geometry);
  const line = new THREE.Line(edge_geometry, new THREE.LineBasicMaterial({color: 0x000000}));
  line.position.add(shift);
  line.position.add(point);
  scene.add(line);
}

function switch_coords(point) {
  const temp = point.y;
  point.y = point.z;
  point.z = temp;
}

function test_get_points_for_orientation(){
  const p = new Point(2, 2, 1);
  const o = Orientation.UprightUp
  const result = wasm_get_points_for_orientation(p, o);
  for (const point of result){
    switch_coords(point);
  }

  draw_piece(result)
}

function setupGeometry() {
  const xy_plane = new THREE.GridHelper(6, 6);
  xy_plane.rotateX(THREE.MathUtils.DEG2RAD * 90);
  xy_plane.position.add(new THREE.Vector3(3,3,0));
  scene.add(xy_plane);

  const xz_plane = new THREE.GridHelper(6, 6);
  xz_plane.rotateY(THREE.MathUtils.DEG2RAD * 90);
  xz_plane.position.add(new THREE.Vector3(3,0,3));
  scene.add(xz_plane);

  const yz_plane = new THREE.GridHelper(6, 6);
  yz_plane.rotateZ(THREE.MathUtils.DEG2RAD * 90);
  yz_plane.position.add(new THREE.Vector3(0,3,3));
  scene.add(yz_plane);
  

  // "The X axis is red. The Y axis is green. The Z axis is blue."
  const axesHelper = new THREE.AxesHelper(5);
  scene.add(axesHelper);

  camera.position.z = 5;
}

function setupControls() {
  controls.rotateSpeed = 1.0;
  controls.zoomSpeed = 1.2;
  controls.panSpeed = 0.8;
  controls.keys = ["KeyA", "KeyS", "KeyD"];
}

function animate() {
  requestAnimationFrame(animate);

  controls.update();

  renderer.render(scene, camera);
}

test_get_points_for_orientation();
setupControls();
setupGeometry();
animate();
