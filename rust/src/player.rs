use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy::input::mouse::MouseMotion;
use crate::world::VoxelWorld;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
           .add_systems(Update, (player_move, cursor_grab));
    }
}

#[derive(Component)]
pub struct Player {
    pub velocity: Vec3,
    pub speed: f32,
    pub jump_force: f32,
    pub gravity: f32,
    pub on_ground: bool,
}

fn setup_player(mut commands: Commands, world: Res<VoxelWorld>) {
    // Find highest block at 0,0
    let mut highest_y = 0;
    for y in (0..=32).rev() {
        if world.get_block(0, y, 0).is_some() {
            highest_y = y;
            break;
        }
    }

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, highest_y as f32 + 2.0, 0.0),
            ..default()
        },
        Player {
            velocity: Vec3::ZERO,
            speed: 10.0,
            jump_force: 8.0,
            gravity: 25.0,
            on_ground: false,
        },
    ));
}

fn cursor_grab(
    mut windows: Query<&mut Window>,
    btn: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    let mut window = windows.single_mut();

    if btn.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}

fn player_move(
    time: Res<Time>,
    mut mouse_motion: EventReader<MouseMotion>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player)>,
    windows: Query<&Window>,
    world: Res<VoxelWorld>,
) {
    let window = windows.single();
    if window.cursor.grab_mode != CursorGrabMode::Locked {
        return;
    }

    let delta = time.delta_seconds();
    for (mut transform, mut player) in query.iter_mut() {
        // Look
        let mut mouse_delta = Vec2::ZERO;
        for ev in mouse_motion.read() {
            mouse_delta += ev.delta;
        }

        let sensitivity = 0.002;
        let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
        yaw -= mouse_delta.x * sensitivity;
        pitch -= mouse_delta.y * sensitivity;
        pitch = pitch.clamp(-1.5, 1.5);
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);

        // Move
        let mut direction = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) { direction.z -= 1.0; }
        if keys.pressed(KeyCode::KeyS) { direction.z += 1.0; }
        if keys.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
        if keys.pressed(KeyCode::KeyD) { direction.x += 1.0; }

        let current_speed = if keys.pressed(KeyCode::ShiftLeft) {
            player.speed * 2.5
        } else {
            player.speed
        };

        if direction != Vec3::ZERO {
            direction = direction.normalize();
        }

        // Apply rotation to direction based on yaw only
        let (yaw, _, _) = transform.rotation.to_euler(EulerRot::YXZ);
        let forward = Vec3::new(-yaw.sin(), 0.0, -yaw.cos());
        let right = Vec3::new(yaw.cos(), 0.0, -yaw.sin());
        let mut move_dir = forward * -direction.z + right * direction.x;
        
        if move_dir != Vec3::ZERO {
            move_dir = move_dir.normalize();
        }

        player.velocity.x = move_dir.x * current_speed;
        player.velocity.z = move_dir.z * current_speed;

        // Jump
        if keys.pressed(KeyCode::Space) && player.on_ground {
            player.velocity.y = player.jump_force;
            player.on_ground = false;
        }

        // Gravity
        player.velocity.y -= player.gravity * delta;

        let check_collision = |pos: Vec3| -> bool {
            let foot_y = pos.y - 1.6;
            let block_x = pos.x.round() as i32;
            let block_z = pos.z.round() as i32;
            
            world.get_block(block_x, foot_y.round() as i32, block_z).is_some() ||
            world.get_block(block_x, pos.y.round() as i32, block_z).is_some()
        };

        // Apply horizontal movement separately to allow sliding
        let mut new_pos = transform.translation;
        new_pos.x += player.velocity.x * delta;
        if check_collision(new_pos) {
            new_pos.x = transform.translation.x; // Revert X
        }

        new_pos.z += player.velocity.z * delta;
        if check_collision(new_pos) {
            new_pos.z = transform.translation.z; // Revert Z
        }
        
        transform.translation.x = new_pos.x;
        transform.translation.z = new_pos.z;

        // Apply vertical movement
        transform.translation.y += player.velocity.y * delta;
        
        // Floor collision
        player.on_ground = false;
        let foot_y = transform.translation.y - 1.6;
        let block_x = transform.translation.x.round() as i32;
        let block_z = transform.translation.z.round() as i32;
        let block_y = foot_y.round() as i32;

        if world.get_block(block_x, block_y, block_z).is_some() {
            if player.velocity.y < 0.0 {
                player.velocity.y = 0.0;
                transform.translation.y = block_y as f32 + 0.5 + 1.6;
                player.on_ground = true;
            }
        }
        
        // Simple void reset
        if transform.translation.y < -10.0 {
            transform.translation.y = 20.0;
            player.velocity.y = 0.0;
        }
    }
}
