import * as THREE from 'three';
import { World } from './world.js';
import { Player } from './player.js';
import { Interaction } from './interaction.js';

// Setup Scene, Camera, Renderer
const scene = new THREE.Scene();
scene.background = new THREE.Color(0x000000); // Black sky
scene.fog = new THREE.Fog(0x000000, 20, 60);

const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);

const renderer = new THREE.WebGLRenderer({ canvas: document.getElementById('game-canvas') });
renderer.setSize(window.innerWidth, window.innerHeight);
renderer.setPixelRatio(window.devicePixelRatio);

// Lights
const ambientLight = new THREE.AmbientLight(0xffffff, 0.6);
scene.add(ambientLight);

const dirLight = new THREE.DirectionalLight(0xffffff, 0.8);
dirLight.position.set(100, 100, 50);
scene.add(dirLight);

// World
const world = new World();
scene.add(world.group);

// Player (Controls & Physics)
const player = new Player(camera, scene, document.body, world);

// Interaction (Placing/Breaking blocks)
const interaction = new Interaction(camera, scene, world);

// Handle UI & Pointer Lock
const blocker = document.getElementById('blocker');
const instructions = document.getElementById('instructions');

instructions.addEventListener('click', () => {
  player.lock();
});

player.controls.addEventListener('lock', () => {
  instructions.style.display = 'none';
  blocker.style.display = 'none';
});

player.controls.addEventListener('unlock', () => {
  blocker.style.display = 'flex';
  instructions.style.display = '';
});

// Window Resize
window.addEventListener('resize', () => {
  camera.aspect = window.innerWidth / window.innerHeight;
  camera.updateProjectionMatrix();
  renderer.setSize(window.innerWidth, window.innerHeight);
});

// Main Loop
const clock = new THREE.Clock();

function animate() {
  requestAnimationFrame(animate);

  const delta = clock.getDelta();
  
  if (player.controls.isLocked) {
    player.update(delta);
    // Since interaction is simple raycasting on click, we don't need to update it every frame unless we want a highlight box.
  }

  renderer.render(scene, camera);
}

// Generate world and start
world.generate();
player.spawn();
animate();
