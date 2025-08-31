use bevy::prelude::*;

pub struct MapLoadingPlugin;

impl Plugin for MapLoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_map_loader)
            .add_systems(Update, handle_map_loading_input);
    }
}

#[derive(Resource)]
pub struct MapLoader {
    pub current_map: Option<String>,
    pub maps_directory: String,
}

impl Default for MapLoader {
    fn default() -> Self {
        Self {
            current_map: None,
            maps_directory: "assets/maps".to_string(),
        }
    }
}

#[derive(Component)]
pub struct MapGeometry;

#[derive(Component)]
pub struct MapEntity;

// BSP Map structure (simplified for now)
#[derive(Debug, Clone)]
pub struct BspMap {
    pub name: String,
    pub entities: Vec<MapEntityData>,
    pub brushes: Vec<BspBrush>,
}

#[derive(Debug, Clone)]
pub struct MapEntityData {
    pub classname: String,
    pub origin: Vec3,
    pub properties: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct BspBrush {
    pub vertices: Vec<Vec3>,
    pub texture: String,
    pub is_solid: bool,
}

fn setup_map_loader(mut commands: Commands) {
    commands.insert_resource(MapLoader::default());
    
    // Load a default test map
    load_test_map(&mut commands);
}

fn load_test_map(commands: &mut Commands) {
    // Create a simple test "map" with some basic geometry
    // In a real implementation, this would parse BSP files
    
    info!("Loading test map...");
    
    // This is where BSP loading would happen
    // For now, we create a simple test level structure
}

fn handle_map_loading_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut map_loader: ResMut<MapLoader>,
    mut commands: Commands,
    map_entities: Query<Entity, With<MapEntity>>,
) {
    if keyboard_input.just_pressed(KeyCode::F7) {
        // Unload current map
        for entity in map_entities.iter() {
            commands.entity(entity).despawn();
        }
        
        info!("Reloading test map...");
        load_test_map(&mut commands);
    }
}

// BSP file loading functions (stubbed for now)
pub fn load_bsp_map(file_path: &str) -> Result<BspMap, String> {
    // TODO: Implement actual BSP file parsing
    // This would involve:
    // 1. Reading the BSP file format
    // 2. Parsing lumps (vertices, faces, entities, etc.)
    // 3. Converting to Bevy-compatible geometry
    
    Err(format!("BSP loading not yet implemented for {}", file_path))
}

pub fn spawn_map_geometry(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    map: &BspMap,
) {
    // Convert BSP brushes to Bevy meshes
    for brush in &map.brushes {
        if brush.is_solid {
            // Create collision geometry
        } else {
            // Create visual geometry
        }
    }
}

pub fn spawn_map_entities(
    commands: &mut Commands,
    map: &BspMap,
) {
    for entity in &map.entities {
        match entity.classname.as_str() {
            "info_player_start" => {
                // Spawn player start position
                commands.spawn((
                    Transform::from_translation(entity.origin),
                    MapEntity,
                ));
            },
            "light" => {
                // Spawn light entity
                if let Some(intensity_str) = entity.properties.get("_light") {
                    if let Ok(intensity) = intensity_str.parse::<f32>() {
                        commands.spawn((
                            PointLight {
                                intensity,
                                range: 20.0,
                                shadows_enabled: true,
                                ..default()
                            },
                            Transform::from_translation(entity.origin),
                            MapEntity,
                        ));
                    }
                }
            },
            _ => {
                // Handle other entity types
                info!("Unknown entity type: {}", entity.classname);
            }
        }
    }
}

// Utility functions for map creation
pub fn create_simple_room(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    center: Vec3,
    size: Vec3,
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.6, 0.6),
        ..default()
    });

    // Floor
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(size.x, size.z))),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(center - Vec3::new(0.0, size.y / 2.0, 0.0)),
        MapGeometry,
        MapEntity,
    ));

    // Walls
    for i in 0..4 {
        let angle = i as f32 * std::f32::consts::FRAC_PI_2;
        let wall_pos = center + Vec3::new(
            (size.x / 2.0) * angle.cos(),
            0.0,
            (size.z / 2.0) * angle.sin(),
        );
        
        commands.spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(size.z, size.y))),
            MeshMaterial3d(material.clone()),
            Transform::from_translation(wall_pos)
                .with_rotation(Quat::from_rotation_y(angle + std::f32::consts::FRAC_PI_2)),
            MapGeometry,
            MapEntity,
        ));
    }
}