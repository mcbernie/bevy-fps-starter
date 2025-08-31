use bevy::prelude::*;

mod fps_controller;
mod lighting;
mod physics;
mod multiplayer;
mod maps;
mod ui;
mod weapons;
mod interaction;

use fps_controller::FpsControllerPlugin;
use lighting::LightingPlugin;
use physics::PhysicsPlugin;
use multiplayer::MultiplayerPlugin;
use maps::MapLoadingPlugin;
use ui::GameUIPlugin;
use weapons::WeaponPlugin;
use interaction::InteractionPlugin;

fn main() {
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy FPS Starter - Half-Life Style".into(),
                resolution: (1280.0, 720.0).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        // Diagnostics are included in DefaultPlugins in Bevy 0.16
        // Core game systems
        .add_plugins((
            FpsControllerPlugin,
            PhysicsPlugin,
            LightingPlugin,
            MapLoadingPlugin,
            MultiplayerPlugin,
            GameUIPlugin,
            WeaponPlugin,
            InteractionPlugin,
        ))
        .add_systems(Startup, setup_game)
        .add_systems(Update, handle_input);

    // Development tools (commented out for now due to compatibility)
    // #[cfg(feature = "dev")]
    // app.add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin);
    
    app.run();
}

fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            ..default()
        })),
        Transform::default(),
    ));

    // Some basic level geometry for testing
    for i in 0..5 {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.8, 0.7, 0.6),
                ..default()
            })),
            Transform::from_xyz(i as f32 * 4.0 - 8.0, 1.0, -10.0),
        ));
    }

    // Add some lighting
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
    ));
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}