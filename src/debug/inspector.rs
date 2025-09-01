//! Debug inspector and development tools
//! 
//! Provides enhanced debugging capabilities using bevy-inspector-egui

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(WorldInspectorPlugin::new())
            .add_systems(Startup, setup_debug_tools)
            .add_systems(Update, (
                handle_debug_input,
                update_debug_info,
            ));
    }
}

#[derive(Resource)]
pub struct DebugSettings {
    pub show_inspector: bool,
    pub show_physics_wireframes: bool,
    pub show_fps_overlay: bool,
    pub log_entity_counts: bool,
}

impl Default for DebugSettings {
    fn default() -> Self {
        Self {
            show_inspector: true,
            show_physics_wireframes: false,
            show_fps_overlay: true,
            log_entity_counts: false,
        }
    }
}

fn setup_debug_tools(mut commands: Commands) {
    commands.insert_resource(DebugSettings::default());
    
    info!("Debug tools initialized");
    info!("Press F1 to toggle inspector, F2 for physics wireframes, F3 for FPS overlay");
}

fn handle_debug_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut debug_settings: ResMut<DebugSettings>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        debug_settings.show_inspector = !debug_settings.show_inspector;
        info!("Inspector: {}", if debug_settings.show_inspector { "ON" } else { "OFF" });
    }
    
    if keyboard_input.just_pressed(KeyCode::F2) {
        debug_settings.show_physics_wireframes = !debug_settings.show_physics_wireframes;
        info!("Physics wireframes: {}", if debug_settings.show_physics_wireframes { "ON" } else { "OFF" });
    }
    
    if keyboard_input.just_pressed(KeyCode::F3) {
        debug_settings.show_fps_overlay = !debug_settings.show_fps_overlay;
        info!("FPS overlay: {}", if debug_settings.show_fps_overlay { "ON" } else { "OFF" });
    }
    
    if keyboard_input.just_pressed(KeyCode::F4) {
        debug_settings.log_entity_counts = !debug_settings.log_entity_counts;
        info!("Entity count logging: {}", if debug_settings.log_entity_counts { "ON" } else { "OFF" });
    }
}

fn update_debug_info(
    debug_settings: Res<DebugSettings>,
    query: Query<Entity>,
) {
    if debug_settings.log_entity_counts {
        let entity_count = query.iter().count();
        if entity_count > 0 {
            debug!("Total entities: {}", entity_count);
        }
    }
}