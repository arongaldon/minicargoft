import * as THREE from 'three';

export class Interaction {
  constructor(camera, scene, world) {
    this.camera = camera;
    this.scene = scene;
    this.world = world;
    this.raycaster = new THREE.Raycaster();
    this.center = new THREE.Vector2(0, 0); // Always casting from center of screen
    
    // Selected block type (1=Dirt, 2=Grass, 3=Stone)
    this.selectedBlockType = 1;

    document.addEventListener('mousedown', this.onMouseDown.bind(this));
    document.addEventListener('keydown', this.onKeyDown.bind(this));
  }
  
  onKeyDown(event) {
    if (event.key === '1') this.selectedBlockType = 1;
    if (event.key === '2') this.selectedBlockType = 2;
    if (event.key === '3') this.selectedBlockType = 3;
  }

  onMouseDown(event) {
    // Only interact if pointer is locked (game is active)
    if (document.pointerLockElement !== document.body) return;

    this.raycaster.setFromCamera(this.center, this.camera);
    
    // Raycast against all meshes in the world group
    const intersects = this.raycaster.intersectObjects(this.world.group.children, false);
    
    if (intersects.length > 0) {
      // Find the closest intersection
      const intersect = intersects[0];
      
      // Calculate block position.
      // We need to use the instanceId to get the matrix and position for InstancedMesh
      if (intersect.object.isInstancedMesh) {
        const instanceId = intersect.instanceId;
        const matrix = new THREE.Matrix4();
        intersect.object.getMatrixAt(instanceId, matrix);
        
        const position = new THREE.Vector3();
        position.setFromMatrixPosition(matrix);
        
        const x = Math.round(position.x);
        const y = Math.round(position.y);
        const z = Math.round(position.z);

        if (event.button === 0) {
          // Left click: Break block
          this.world.setBlock(x, y, z, 0);
        } else if (event.button === 2) {
          // Right click: Place block
          // We need to place it on the adjacent face.
          // The intersection normal tells us which face was hit.
          const normal = intersect.face.normal;
          
          // Since the block itself may have rotation applied (though we didn't), 
          // usually the normal is local. Since we didn't rotate instances, normal is world space.
          const placeX = x + Math.round(normal.x);
          const placeY = y + Math.round(normal.y);
          const placeZ = z + Math.round(normal.z);
          
          this.world.setBlock(placeX, placeY, placeZ, this.selectedBlockType);
        }
      }
    }
  }
}
