use bevy::prelude::*;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_lighting)
            .add_systems(Update, (
                update_lighting,
                handle_lighting_debug,
            ));
    }
}

#[derive(Component)]
pub struct DynamicLight {
    pub flicker_speed: f32,
    pub flicker_intensity: f32,
    pub base_intensity: f32,
}

impl Default for DynamicLight {
    fn default() -> Self {
        Self {
            flicker_speed: 2.0,
            flicker_intensity: 0.2,
            base_intensity: 1.0,
        }
    }
}

#[derive(Resource)]
pub struct LightingSettings {
    pub ambient_intensity: f32,
    pub shadow_quality: bool,
}

impl Default for LightingSettings {
    fn default() -> Self {
        Self {
            ambient_intensity: 0.3,
            shadow_quality: true,
        }
    }
}

fn setup_lighting(
    mut commands: Commands,
    mut ambient_light: ResMut<AmbientLight>,
) {
    commands.insert_resource(LightingSettings::default());
    
    // Set ambient lighting
    ambient_light.color = Color::srgb(0.4, 0.4, 0.6);
    ambient_light.brightness = 0.3;

    // Add some point lights to create atmosphere
    commands.spawn((
        PointLight {
            intensity: 500.0,
            range: 20.0,
            color: Color::srgb(1.0, 0.9, 0.7),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-8.0, 4.0, -8.0),
        DynamicLight {
            flicker_speed: 3.0,
            flicker_intensity: 0.3,
            base_intensity: 500.0,
        },
    ));

    commands.spawn((
        PointLight {
            intensity: 300.0,
            range: 15.0,
            color: Color::srgb(0.8, 1.0, 0.9),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(8.0, 3.0, -8.0),
        DynamicLight {
            flicker_speed: 1.5,
            flicker_intensity: 0.1,
            base_intensity: 300.0,
        },
    ));

    // Add a spotlight for dramatic effect
    commands.spawn((
        SpotLight {
            intensity: 1000.0,
            range: 30.0,
            color: Color::srgb(1.0, 1.0, 0.8),
            shadows_enabled: true,
            inner_angle: 0.3,
            outer_angle: 0.8,
            ..default()
        },
        Transform::from_xyz(0.0, 10.0, 10.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    ));
}

fn update_lighting(
    time: Res<Time>,
    mut query: Query<(&mut PointLight, &DynamicLight)>,
) {
    for (mut light, dynamic) in query.iter_mut() {
        let flicker = (time.elapsed_secs() * dynamic.flicker_speed).sin() * dynamic.flicker_intensity;
        light.intensity = dynamic.base_intensity * (1.0 + flicker);
    }
}

fn handle_lighting_debug(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<LightingSettings>,
    mut ambient_light: ResMut<AmbientLight>,
    mut point_lights: Query<&mut PointLight>,
    mut spot_lights: Query<&mut SpotLight>,
    mut directional_lights: Query<&mut DirectionalLight>,
) {
    if keyboard_input.just_pressed(KeyCode::F4) {
        settings.shadow_quality = !settings.shadow_quality;
        
        // Toggle shadows for all lights
        for mut light in point_lights.iter_mut() {
            light.shadows_enabled = settings.shadow_quality;
        }
        for mut light in spot_lights.iter_mut() {
            light.shadows_enabled = settings.shadow_quality;
        }
        for mut light in directional_lights.iter_mut() {
            light.shadows_enabled = settings.shadow_quality;
        }
    }

    if keyboard_input.just_pressed(KeyCode::F5) {
        settings.ambient_intensity = if settings.ambient_intensity > 0.1 { 0.05 } else { 0.3 };
        ambient_light.brightness = settings.ambient_intensity;
    }
}

// Utility functions for creating different types of lights
pub fn create_point_light(
    commands: &mut Commands,
    position: Vec3,
    color: Color,
    intensity: f32,
    range: f32,
) -> Entity {
    commands.spawn((
        PointLight {
            intensity,
            range,
            color,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_translation(position),
    )).id()
}

pub fn create_flickering_light(
    commands: &mut Commands,
    position: Vec3,
    color: Color,
    intensity: f32,
    range: f32,
    flicker_speed: f32,
) -> Entity {
    commands.spawn((
        PointLight {
            intensity,
            range,
            color,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_translation(position),
        DynamicLight {
            flicker_speed,
            flicker_intensity: 0.2,
            base_intensity: intensity,
        },
    )).id()
}