use bevy::prelude::*;

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameAssets>()
            .add_event::<AssetLoadingComplete>()
            .add_systems(Startup, load_game_assets)
            .add_systems(Update, check_asset_loading);
    }
}

#[derive(Resource, Default)]
pub struct GameAssets {
    pub weapon_model: Handle<Scene>,
    pub character_model: Handle<Scene>,
    pub assets_loaded: bool,
    pub loading_progress: f32,
}

#[derive(Component)]
pub struct LoadingScreen;

#[derive(Component)]
pub struct CharacterModel;

#[derive(Component)]
pub struct WeaponModel;

fn load_game_assets(
    mut game_assets: ResMut<GameAssets>,
    asset_server: Res<AssetServer>,
) {
    info!("Loading game assets...");
    
    // Load weapon model from fpsview
    game_assets.weapon_model = asset_server.load("fpsview/fps_saiga_animations/scene.gltf#Scene0");
    
    // For character, we'll load the base model (FBX will be converted to GLTF in a real scenario)
    // For now, let's use a placeholder or create a simple character representation
    // Note: Bevy doesn't natively support FBX, so we'd need to convert or use a different approach
    
    info!("Asset loading initiated");
}

fn check_asset_loading(
    mut game_assets: ResMut<GameAssets>,
    asset_server: Res<AssetServer>,
    mut loading_events: EventWriter<AssetLoadingComplete>,
) {
    if game_assets.assets_loaded {
        return;
    }

    let weapon_state = asset_server.get_load_state(&game_assets.weapon_model);
    
    match weapon_state {
        Some(bevy::asset::LoadState::Loaded) => {
            if !game_assets.assets_loaded {
                game_assets.assets_loaded = true;
                game_assets.loading_progress = 1.0;
                loading_events.write(AssetLoadingComplete);
                info!("All game assets loaded successfully!");
            }
        },
        Some(bevy::asset::LoadState::Failed(_)) => {
            warn!("Failed to load weapon asset");
            // For now, mark as loaded to continue
            game_assets.assets_loaded = true;
            game_assets.loading_progress = 1.0;
            loading_events.write(AssetLoadingComplete);
        },
        _ => {
            // Still loading
            game_assets.loading_progress = 0.5; // Simplified progress
        }
    }
}

#[derive(Event)]
pub struct AssetLoadingComplete;

pub fn spawn_weapon_model(
    commands: &mut Commands,
    game_assets: &GameAssets,
    position: Vec3,
) -> Entity {
    commands.spawn((
        SceneRoot(game_assets.weapon_model.clone()),
        Transform::from_translation(position),
        WeaponModel,
    )).id()
}

pub fn spawn_character_model(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) -> Entity {
    // For now, create a simple character representation
    // In a full implementation, this would load the actual FBX character
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(0.4, 1.8))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.6, 0.4), // Skin-like color
            ..default()
        })),
        Transform::from_translation(position),
        CharacterModel,
    )).id()
}

pub fn create_simple_level_geometry(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // Create a simple level to showcase the assets
    // This represents what would be loaded from the BSP file
    
    // Create walls
    let wall_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.7, 0.7, 0.6),
        perceptual_roughness: 0.8,
        ..default()
    });

    // Back wall
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(20.0, 5.0, 0.5))),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_xyz(0.0, 2.5, -10.0),
        crate::maps::MapGeometry,
    ));

    // Side walls
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 5.0, 20.0))),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_xyz(-10.0, 2.5, 0.0),
        crate::maps::MapGeometry,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 5.0, 20.0))),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_xyz(10.0, 2.5, 0.0),
        crate::maps::MapGeometry,
    ));

    // Create some platforms and obstacles
    let platform_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.6, 0.8),
        ..default()
    });

    for i in 0..3 {
        let x = (i as f32 - 1.0) * 6.0;
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(2.0, 0.2, 2.0))),
            MeshMaterial3d(platform_material.clone()),
            Transform::from_xyz(x, 1.0 + i as f32 * 0.5, -5.0),
            crate::maps::MapGeometry,
        ));
    }

    info!("Created simple level geometry");
}