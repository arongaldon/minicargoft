use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy::input::mouse::MouseMotion;
use crate::world::VoxelWorld;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PlayerSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player.in_set(PlayerSystemSet))
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
    pub yaw: f32,
    pub pitch: f32,
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
            yaw: 0.0,
            pitch: 0.0,
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

fn check_collision(pos: Vec3, world: &VoxelWorld) -> bool {
    let radius = 0.28;
    let camera_offset = 1.6;
    let foot_y = pos.y - camera_offset;
    
    // Check points to cover the player's height
    // We start at 0.4 to avoid getting snagged on the block we are standing on
    for y_offset in [0.4, 0.9, 1.4] {
        let check_y = (foot_y + y_offset + 0.5).floor() as i32;
        
        for x_offset in [-radius, radius] {
            for z_offset in [-radius, radius] {
                let check_x = (pos.x + x_offset + 0.5).floor() as i32;
                let check_z = (pos.z + z_offset + 0.5).floor() as i32;
                
                if world.get_block(check_x, check_y, check_z).is_some() {
                    return true;
                }
            }
        }
    }
    false
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

    let delta = time.delta_seconds().min(0.05);
    
    let mut mouse_delta = Vec2::ZERO;
    if time.elapsed_seconds() > 0.5 {
        for ev in mouse_motion.read() {
            mouse_delta += ev.delta;
        }
    } else {
        // Still clear the events during warmup so they don't pile up
        for _ in mouse_motion.read() {}
    }

    for (mut transform, mut player) in query.iter_mut() {
        // --- Rotation (Look) ---
        let sensitivity = 0.002;
        player.yaw -= mouse_delta.x * sensitivity;
        player.pitch -= mouse_delta.y * sensitivity;
        player.pitch = player.pitch.clamp(-1.5, 1.5);
        
        transform.rotation = Quat::from_euler(EulerRot::YXZ, player.yaw, player.pitch, 0.0);

        // --- Movement Direction ---
        let mut direction = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) { direction.z -= 1.0; }
        if keys.pressed(KeyCode::KeyS) { direction.z += 1.0; }
        if keys.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
        if keys.pressed(KeyCode::KeyD) { direction.x += 1.0; }

        let current_speed = if keys.pressed(KeyCode::ShiftLeft) {
            player.speed * 1.8
        } else {
            player.speed
        };

        if direction != Vec3::ZERO {
            direction = direction.normalize();
        }

        let forward = Vec3::new(-player.yaw.sin(), 0.0, -player.yaw.cos());
        let right = Vec3::new(player.yaw.cos(), 0.0, -player.yaw.sin());
        let move_dir = forward * -direction.z + right * direction.x;
        
        player.velocity.x = move_dir.x * current_speed;
        player.velocity.z = move_dir.z * current_speed;

        // --- Jump ---
        if keys.pressed(KeyCode::Space) && player.on_ground {
            player.velocity.y = player.jump_force;
            player.on_ground = false;
        }

        // --- Gravity ---
        player.velocity.y -= player.gravity * delta;

        // --- Collision & Movement Resolution ---
        
        // 1. Horizontal X
        let mut next_pos = transform.translation;
        next_pos.x += player.velocity.x * delta;
        if !check_collision(next_pos, &world) {
            transform.translation.x = next_pos.x;
        } else {
            player.velocity.x = 0.0;
        }

        // 2. Horizontal Z
        next_pos = transform.translation;
        next_pos.z += player.velocity.z * delta;
        if !check_collision(next_pos, &world) {
            transform.translation.z = next_pos.z;
        } else {
            player.velocity.z = 0.0;
        }

        // 3. Vertical Y
        next_pos = transform.translation;
        next_pos.y += player.velocity.y * delta;
        if check_collision(next_pos, &world) {
            if player.velocity.y < 0.0 {
                let foot_y = (next_pos.y - 1.6 + 0.5).floor();
                transform.translation.y = foot_y + 0.5 + 1.6;
                player.velocity.y = 0.0;
                player.on_ground = true;
            } else {
                player.velocity.y = 0.0;
            }
        } else {
            transform.translation.y = next_pos.y;
            let mut check_ground_pos = transform.translation;
            check_ground_pos.y -= 0.1;
            player.on_ground = check_collision(check_ground_pos, &world);
        }

        // --- Void Reset ---
        if transform.translation.y < -30.0 {
            transform.translation.y = 50.0;
            player.velocity.y = 0.0;
        }
    }
}
