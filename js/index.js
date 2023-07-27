import("../pkg/index.js")
  .catch(console.error)
  .finally(() => console.log("Successfully imported wasm package"));
import * as THREE from "three";
import { TrackballControls } from "three/examples/jsm/controls/TrackballControls.js";

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

function fill_grid_square(x, y, z){

}

function test_get_points_for_orientation(){

}

function setupGeometry() {
  const xy_plane = new THREE.GridHelper(7, 6);
  xy_plane.rotateX(THREE.MathUtils.DEG2RAD * 90);
  xy_plane.position.add(new THREE.Vector3(3.5,3.5,0));
  scene.add(xy_plane);

  const xz_plane = new THREE.GridHelper(7, 6);
  xz_plane.rotateY(THREE.MathUtils.DEG2RAD * 90);
  xz_plane.position.add(new THREE.Vector3(3.5,0,3.5));
  scene.add(xz_plane);

  const yz_plane = new THREE.GridHelper(7, 6);
  yz_plane.rotateZ(THREE.MathUtils.DEG2RAD * 90);
  yz_plane.position.add(new THREE.Vector3(0,3.5,3.5));
  scene.add(yz_plane);
  

  const geometry = new THREE.SphereGeometry(0.5);
  const material = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
  const sphere = new THREE.Mesh(geometry, material);
  scene.add(sphere);

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

setupControls();
setupGeometry();
animate();
