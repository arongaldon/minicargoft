import * as THREE from 'three';
import { PointerLockControls } from 'three/addons/controls/PointerLockControls.js';

export class Player {
  constructor(camera, scene, domElement, world) {
    this.camera = camera;
    this.world = world;
    this.controls = new PointerLockControls(camera, domElement);
    
    // Player physics state
    this.velocity = new THREE.Vector3();
    this.direction = new THREE.Vector3();
    this.onObject = false;
    
    // Config
    this.speed = 10.0;
    this.jumpForce = 8.0;
    this.gravity = 25.0;
    
    // Input state
    this.moveForward = false;
    this.moveBackward = false;
    this.moveLeft = false;
    this.moveRight = false;
    this.sprint = false;

    this.setupInputs();
  }

  setupInputs() {
    const onKeyDown = (event) => {
      switch (event.code) {
        case 'ArrowUp':
        case 'KeyW':
          this.moveForward = true;
          break;
        case 'ArrowLeft':
        case 'KeyA':
          this.moveLeft = true;
          break;
        case 'ArrowDown':
        case 'KeyS':
          this.moveBackward = true;
          break;
        case 'ArrowRight':
        case 'KeyD':
          this.moveRight = true;
          break;
        case 'Space':
          if (this.onObject === true) this.velocity.y += this.jumpForce;
          break;
        case 'ShiftLeft':
        case 'ShiftRight':
          this.sprint = true;
          break;
      }
    };

    const onKeyUp = (event) => {
      switch (event.code) {
        case 'ArrowUp':
        case 'KeyW':
          this.moveForward = false;
          break;
        case 'ArrowLeft':
        case 'KeyA':
          this.moveLeft = false;
          break;
        case 'ArrowDown':
        case 'KeyS':
          this.moveBackward = false;
          break;
        case 'ArrowRight':
        case 'KeyD':
          this.moveRight = false;
          break;
        case 'ShiftLeft':
        case 'ShiftRight':
          this.sprint = false;
          break;
      }
    };

    document.addEventListener('keydown', onKeyDown);
    document.addEventListener('keyup', onKeyUp);
  }

  lock() {
    this.controls.lock();
  }

  spawn() {
    // Find highest block at 0,0
    let highestY = 0;
    for (let y = 32; y >= 0; y--) {
      if (this.world.getBlock(0, y, 0)) {
        highestY = y;
        break;
      }
    }
    this.camera.position.set(0, highestY + 2, 0);
  }

  checkCollisions(newPos) {
    // Very basic collision: check block at foot and eye level
    const playerWidth = 0.6;
    const playerHeight = 1.6;
    
    const footPos = newPos.clone();
    footPos.y -= 1.6; // Assuming camera is at eye level
    
    const x = Math.floor(footPos.x + 0.5);
    const z = Math.floor(footPos.z + 0.5);
    const yFoot = Math.floor(footPos.y + 0.5);
    const yEye = Math.floor(newPos.y + 0.5);

    // If there is a solid block where we want to go, return false
    if (this.world.getBlock(x, yFoot, z) || this.world.getBlock(x, yEye, z)) {
      return true; // Collided
    }
    return false;
  }

  update(delta) {
    this.velocity.x -= this.velocity.x * 10.0 * delta;
    this.velocity.z -= this.velocity.z * 10.0 * delta;
    this.velocity.y -= this.gravity * delta; // 100.0 = mass

    this.direction.z = Number(this.moveForward) - Number(this.moveBackward);
    this.direction.x = Number(this.moveRight) - Number(this.moveLeft);
    this.direction.normalize(); // this ensures consistent movements in all directions

    const currentSpeed = this.sprint ? this.speed * 2.5 : this.speed;

    if (this.moveForward || this.moveBackward) this.velocity.z -= this.direction.z * currentSpeed * delta;
    if (this.moveLeft || this.moveRight) this.velocity.x -= this.direction.x * currentSpeed * delta;

    // Tentative new position
    const oldPos = this.camera.position.clone();
    
    this.controls.moveRight(-this.velocity.x * delta);
    this.controls.moveForward(-this.velocity.z * delta);
    
    // Check horizontal collisions
    if (this.checkCollisions(this.camera.position)) {
      // Revert horizontal movement
      this.camera.position.x = oldPos.x;
      this.camera.position.z = oldPos.z;
    }

    // Apply vertical movement
    this.camera.position.y += this.velocity.y * delta;
    
    // Floor collision
    this.onObject = false;
    const footY = this.camera.position.y - 1.6;
    const blockX = Math.floor(this.camera.position.x + 0.5);
    const blockZ = Math.floor(this.camera.position.z + 0.5);
    const blockY = Math.floor(footY + 0.5);

    if (this.world.getBlock(blockX, blockY, blockZ)) {
      this.velocity.y = Math.max(0, this.velocity.y);
      this.camera.position.y = blockY + 0.5 + 1.6;
      this.onObject = true;
    }

    // Simple falling off world safety net
    if (this.camera.position.y < -10) {
      this.spawn();
      this.velocity.y = 0;
    }
  }
}
