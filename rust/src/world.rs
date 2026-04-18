use bevy::prelude::*;
use noise::{NoiseFn, Simplex};
use std::collections::HashMap;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct WorldSystemSet;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VoxelWorld::new())
           .add_systems(Startup, generate_world.in_set(WorldSystemSet));
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Dirt,
    Grass,
    Stone,
}

#[derive(Component)]
pub struct Block;

#[derive(Resource)]
pub struct VoxelWorld {
    pub blocks: HashMap<[i32; 3], BlockType>,
    pub chunk_size: i32,
}

impl VoxelWorld {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
            chunk_size: 32,
        }
    }
    
    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Option<BlockType> {
        self.blocks.get(&[x, y, z]).copied()
    }
}

fn generate_world(
    mut commands: Commands,
    mut world: ResMut<VoxelWorld>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let noise = Simplex::new(42);
    
    // Create materials
    let mat_dirt = materials.add(Color::rgb(0.3, 0.2, 0.1));
    let mat_grass = materials.add(Color::rgb(0.2, 0.5, 0.1));
    let mat_stone = materials.add(Color::rgb(0.5, 0.5, 0.5));
    
    let cube_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

    let size = world.chunk_size;
    
    for x in -size..size {
        for z in -size..size {
            let nx = x as f64 / 30.0;
            let nz = z as f64 / 30.0;
            let noise_val = noise.get([nx, nz]);
            
            let height = ((noise_val + 1.0) * 5.0) as i32 + 5;
            
            for y in 0..=height {
                let block_type = if y == height {
                    BlockType::Grass
                } else if y < height - 3 {
                    BlockType::Stone
                } else {
                    BlockType::Dirt
                };
                
                world.blocks.insert([x, y, z], block_type);
            }
        }
    }

    // Render only visible faces (simple culling)
    for (&[x, y, z], &block_type) in world.blocks.iter() {
        let is_surrounded = 
            world.blocks.contains_key(&[x+1, y, z]) &&
            world.blocks.contains_key(&[x-1, y, z]) &&
            world.blocks.contains_key(&[x, y+1, z]) &&
            world.blocks.contains_key(&[x, y-1, z]) &&
            world.blocks.contains_key(&[x, y, z+1]) &&
            world.blocks.contains_key(&[x, y, z-1]);

        if !is_surrounded {
            let material = match block_type {
                BlockType::Dirt => mat_dirt.clone(),
                BlockType::Grass => mat_grass.clone(),
                BlockType::Stone => mat_stone.clone(),
            };

            commands.spawn((
                PbrBundle {
                    mesh: cube_mesh.clone(),
                    material,
                    transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                    ..default()
                },
                Block,
            ));
        }
    }
}
