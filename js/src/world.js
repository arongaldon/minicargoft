import * as THREE from 'three';
import { createNoise2D } from 'simplex-noise';

export class World {
  constructor() {
    this.group = new THREE.Group();
    this.chunkSize = 32;
    this.worldHeight = 32;
    
    // Store blocks as a 1D array or Map. Let's use a Map with key "x,y,z" for simplicity
    this.blocks = new Map();
    
    // Materials for different block types
    const textureLoader = new THREE.TextureLoader();
    
    // For MVP, just use basic colors
    this.materials = {
      1: new THREE.MeshLambertMaterial({ color: 0x4d3222 }), // Dirt
      2: new THREE.MeshLambertMaterial({ color: 0x3b8521 }), // Grass
      3: new THREE.MeshLambertMaterial({ color: 0x888888 }), // Stone
    };

    this.geometry = new THREE.BoxGeometry(1, 1, 1);
    this.meshes = {}; // Map blockType -> InstancedMesh
  }

  generate() {
    const noise2D = createNoise2D();
    
    for (let x = -this.chunkSize; x < this.chunkSize; x++) {
      for (let z = -this.chunkSize; z < this.chunkSize; z++) {
        // Generate height using noise
        const noiseVal = noise2D(x / 30, z / 30);
        const height = Math.floor((noiseVal + 1) * 5) + 5; // 5 to 15
        
        for (let y = 0; y <= height; y++) {
          let type = 1; // Dirt
          if (y === height) type = 2; // Grass
          if (y < height - 3) type = 3; // Stone
          
          this.setBlock(x, y, z, type, false);
        }
      }
    }
    this.updateMeshes();
  }

  setBlock(x, y, z, type, updateMesh = true) {
    const key = `${x},${y},${z}`;
    if (type === 0) {
      this.blocks.delete(key);
    } else {
      this.blocks.set(key, { x, y, z, type });
    }
    if (updateMesh) {
      this.updateMeshes();
    }
  }

  getBlock(x, y, z) {
    return this.blocks.get(`${x},${y},${z}`)?.type || 0;
  }

  updateMeshes() {
    // Clear old meshes
    this.group.clear();
    
    // Group blocks by type to count them
    const blockCounts = { 1: 0, 2: 0, 3: 0 };
    for (const block of this.blocks.values()) {
      blockCounts[block.type]++;
    }

    // Create new InstancedMeshes
    const dummy = new THREE.Object3D();
    const typeIndices = { 1: 0, 2: 0, 3: 0 };

    for (const [type, count] of Object.entries(blockCounts)) {
      if (count === 0) continue;
      const mesh = new THREE.InstancedMesh(this.geometry, this.materials[type], count);
      this.meshes[type] = mesh;
      this.group.add(mesh);
    }

    // Set positions
    for (const block of this.blocks.values()) {
      const { x, y, z, type } = block;
      
      // Simple face culling: if surrounded, don't render (skip setting matrix or scale to 0)
      // For InstancedMesh we still have the slot, so we can just scale it to 0.
      const isSurrounded = 
        this.getBlock(x+1, y, z) && this.getBlock(x-1, y, z) &&
        this.getBlock(x, y+1, z) && this.getBlock(x, y-1, z) &&
        this.getBlock(x, y, z+1) && this.getBlock(x, y, z-1);

      dummy.position.set(x, y, z);
      
      if (isSurrounded) {
        dummy.scale.set(0, 0, 0); // Hide
      } else {
        dummy.scale.set(1, 1, 1);
      }
      
      dummy.updateMatrix();
      
      const mesh = this.meshes[type];
      const index = typeIndices[type]++;
      mesh.setMatrixAt(index, dummy.matrix);
    }

    for (const mesh of Object.values(this.meshes)) {
      mesh.instanceMatrix.needsUpdate = true;
    }
  }
}
