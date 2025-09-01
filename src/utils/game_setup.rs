//! Game setup and initialization
//! 
//! Contains game initialization logic separated from main.rs

use bevy::prelude::*;
use crate::content::assets;

pub struct GameSetupPlugin;

impl Plugin for GameSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_game);
    }
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