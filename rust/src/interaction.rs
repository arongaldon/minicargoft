use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use crate::world::{VoxelWorld, BlockType, Block};

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedBlock(BlockType::Dirt))
           .add_systems(Update, handle_interaction);
    }
}

#[derive(Resource)]
pub struct SelectedBlock(BlockType);

fn handle_interaction(
    mut commands: Commands,
    windows: Query<&Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut world: ResMut<VoxelWorld>,
    mut selected: ResMut<SelectedBlock>,
    camera_query: Query<&Transform, With<Camera>>,
    block_query: Query<(Entity, &Transform), With<Block>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let window = windows.single();
    if window.cursor.grab_mode != CursorGrabMode::Locked {
        return;
    }

    if keys.just_pressed(KeyCode::Digit1) { selected.0 = BlockType::Dirt; }
    if keys.just_pressed(KeyCode::Digit2) { selected.0 = BlockType::Grass; }
    if keys.just_pressed(KeyCode::Digit3) { selected.0 = BlockType::Stone; }

    let left_click = mouse.just_pressed(MouseButton::Left);
    let right_click = mouse.just_pressed(MouseButton::Right);

    if !left_click && !right_click {
        return;
    }

    let transform = camera_query.single();
    let origin = transform.translation;
    let dir = transform.forward();

    // Simple raycast stepping
    let mut current_pos = origin;
    let step = 0.1;
    let max_dist = 6.0;
    let mut dist = 0.0;
    
    let mut prev_block = None;

    while dist < max_dist {
        let block_x = current_pos.x.round() as i32;
        let block_y = current_pos.y.round() as i32;
        let block_z = current_pos.z.round() as i32;

        if world.get_block(block_x, block_y, block_z).is_some() {
            // Hit
            if left_click {
                // Break
                world.blocks.remove(&[block_x, block_y, block_z]);
                
                // Remove entity
                for (entity, t) in block_query.iter() {
                    if t.translation.x.round() as i32 == block_x &&
                       t.translation.y.round() as i32 == block_y &&
                       t.translation.z.round() as i32 == block_z {
                        commands.entity(entity).despawn();
                        break;
                    }
                }
                
                // Render adjacent blocks that might have been hidden
                let neighbors = [
                    [block_x + 1, block_y, block_z],
                    [block_x - 1, block_y, block_z],
                    [block_x, block_y + 1, block_z],
                    [block_x, block_y - 1, block_z],
                    [block_x, block_y, block_z + 1],
                    [block_x, block_y, block_z - 1],
                ];

                for &[nx, ny, nz] in &neighbors {
                    if let Some(b_type) = world.get_block(nx, ny, nz) {
                        let mut exists = false;
                        for (_, t) in block_query.iter() {
                            if t.translation.x.round() as i32 == nx &&
                               t.translation.y.round() as i32 == ny &&
                               t.translation.z.round() as i32 == nz {
                                exists = true;
                                break;
                            }
                        }
                        if !exists {
                            let material = match b_type {
                                BlockType::Dirt => materials.add(Color::rgb(0.3, 0.2, 0.1)),
                                BlockType::Grass => materials.add(Color::rgb(0.2, 0.5, 0.1)),
                                BlockType::Stone => materials.add(Color::rgb(0.5, 0.5, 0.5)),
                            };
                            commands.spawn((
                                PbrBundle {
                                    mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                                    material,
                                    transform: Transform::from_xyz(nx as f32, ny as f32, nz as f32),
                                    ..default()
                                },
                                Block,
                            ));
                        }
                    }
                }
            } else if right_click {
                // Place
                if let Some((px, py, pz)) = prev_block {
                    world.blocks.insert([px, py, pz], selected.0);
                    
                    let material = match selected.0 {
                        BlockType::Dirt => materials.add(Color::rgb(0.3, 0.2, 0.1)),
                        BlockType::Grass => materials.add(Color::rgb(0.2, 0.5, 0.1)),
                        BlockType::Stone => materials.add(Color::rgb(0.5, 0.5, 0.5)),
                    };
                    
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                            material,
                            transform: Transform::from_xyz(px as f32, py as f32, pz as f32),
                            ..default()
                        },
                        Block,
                    ));
                }
            }
            break;
        }

        prev_block = Some((block_x, block_y, block_z));
        current_pos += dir * step;
        dist += step;
    }
}
