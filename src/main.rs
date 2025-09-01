use bevy::prelude::*;

mod fps_controller;
mod lighting;
mod physics;
mod multiplayer;
mod maps;
mod ui;
mod weapons;
mod interaction;
mod audio;
mod assets;

use bevy_egui::EguiPlugin;
use fps_controller::FpsControllerPlugin;
use lighting::LightingPlugin;
use physics::PhysicsPlugin;
use multiplayer::MultiplayerPlugin;
use maps::MapLoadingPlugin;
use ui::GameUIPlugin;
use weapons::WeaponPlugin;
use interaction::InteractionPlugin;
use audio::AudioPlugin;
use assets::AssetLoadingPlugin;

use bevy_inspector_egui::quick::WorldInspectorPlugin;

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
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
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
            AudioPlugin,
            AssetLoadingPlugin,
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
    _game_assets: Res<assets::GameAssets>,
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

    // Create the example level geometry
    assets::create_simple_level_geometry(&mut commands, &mut meshes, &mut materials);

    // Spawn some character models around the level
    for i in 0..3 {
        let position = Vec3::new(
            (i as f32 - 1.0) * 8.0, 
            0.1, 
            -3.0 + (i as f32 * 2.0)
        );
        assets::spawn_character_model(&mut commands, &mut meshes, &mut materials, position);
    }

    // Enhanced lighting setup
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 10000.0,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 10.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
    ));

    // Add some point lights for atmosphere
    for i in 0..4 {
        let angle = i as f32 * std::f32::consts::PI * 0.5;
        let x = angle.cos() * 8.0;
        let z = angle.sin() * 8.0;
        
        commands.spawn((
            PointLight {
                intensity: 300.0,
                range: 15.0,
                color: Color::srgb(1.0, 0.9, 0.7),
                shadows_enabled: true,
                ..default()
            },
            Transform::from_xyz(x, 3.0, z),
        ));
    }

    info!("Example level setup complete");
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}