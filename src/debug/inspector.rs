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
            .init_resource::<DebugSettings>()
            .add_systems(Startup, setup_debug_tools)
            .add_systems(Update, (
                handle_debug_input,
                update_debug_info,
                log_performance_stats,
            ));
    }
}

#[derive(Resource)]
pub struct DebugSettings {
    pub show_inspector: bool,
    pub show_physics_wireframes: bool,
    pub show_fps_overlay: bool,
    pub log_entity_counts: bool,
    pub log_performance: bool,
}

impl Default for DebugSettings {
    fn default() -> Self {
        Self {
            show_inspector: true,
            show_physics_wireframes: false,
            show_fps_overlay: true,
            log_entity_counts: false,
            log_performance: false,
        }
    }
}

fn setup_debug_tools(_commands: Commands) {
    info!("🔧 Debug tools initialized");
    info!("📋 Debug Controls:");
    info!("   F1 - Toggle inspector");
    info!("   F2 - Toggle physics wireframes");
    info!("   F3 - Toggle FPS overlay");
    info!("   F4 - Toggle entity count logging");
    info!("   F5 - Toggle performance logging");
    info!("   ESC - Exit application");
}

fn handle_debug_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut debug_settings: ResMut<DebugSettings>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        debug_settings.show_inspector = !debug_settings.show_inspector;
        info!("🔍 Inspector: {}", if debug_settings.show_inspector { "ON" } else { "OFF" });
    }
    
    if keyboard_input.just_pressed(KeyCode::F2) {
        debug_settings.show_physics_wireframes = !debug_settings.show_physics_wireframes;
        info!("⚡ Physics wireframes: {}", if debug_settings.show_physics_wireframes { "ON" } else { "OFF" });
    }
    
    if keyboard_input.just_pressed(KeyCode::F3) {
        debug_settings.show_fps_overlay = !debug_settings.show_fps_overlay;
        info!("📊 FPS overlay: {}", if debug_settings.show_fps_overlay { "ON" } else { "OFF" });
    }
    
    if keyboard_input.just_pressed(KeyCode::F4) {
        debug_settings.log_entity_counts = !debug_settings.log_entity_counts;
        info!("📈 Entity count logging: {}", if debug_settings.log_entity_counts { "ON" } else { "OFF" });
    }
    
    if keyboard_input.just_pressed(KeyCode::F5) {
        debug_settings.log_performance = !debug_settings.log_performance;
        info!("⚡ Performance logging: {}", if debug_settings.log_performance { "ON" } else { "OFF" });
    }
}

fn update_debug_info(
    debug_settings: Res<DebugSettings>,
    query: Query<Entity>,
) {
    if debug_settings.log_entity_counts {
        let entity_count = query.iter().count();
        if entity_count > 0 {
            debug!("📊 Total entities: {}", entity_count);
        }
    }
}

fn log_performance_stats(
    debug_settings: Res<DebugSettings>,
    time: Res<Time>,
) {
    if debug_settings.log_performance {
        // Log performance stats every 5 seconds
        if time.elapsed_secs() as u32 % 5 == 0 && time.delta_secs() > 0.0 {
            let fps = 1.0 / time.delta_secs();
            debug!("⚡ Performance: {:.1} FPS, {:.2}ms frame time", fps, time.delta_secs() * 1000.0);
        }
    }
}