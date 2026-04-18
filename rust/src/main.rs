use bevy::prelude::*;

mod world;
mod player;
mod interaction;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Minicargoft (Rust)".to_string(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0))) // Black sky
        .add_plugins(world::WorldPlugin)
        .add_plugins(player::PlayerPlugin)
        .configure_sets(Startup, player::PlayerSystemSet.after(world::WorldSystemSet))
        .add_plugins(interaction::InteractionPlugin)
        .add_systems(Startup, setup_scene)
        .run();
}

fn setup_scene(mut commands: Commands) {
    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.6,
    });

    // Directional light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        transform: Transform::from_xyz(100.0, 100.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
