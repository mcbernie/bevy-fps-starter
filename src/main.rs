use bevy::prelude::*;

// Organized module structure following game development principles
mod core;
mod gameplay;
mod content;
mod ui;
mod rendering;
mod debug;
mod utils;

use bevy_egui::EguiPlugin;

// Import plugins from organized modules
use core::{FpsControllerPlugin, PhysicsPlugin};
use gameplay::{WeaponPlugin, InteractionPlugin, AudioPlugin};
use content::{AssetLoadingPlugin, MapLoadingPlugin};
use ui::{GameUIPlugin, MultiplayerPlugin};
use rendering::LightingPlugin;
use debug::DebugPlugin;
use utils::GameSetupPlugin;

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
        
        // Debug tools (enhanced inspector integration)
        .add_plugins(DebugPlugin)
        
        // Core game systems
        .add_plugins((
            FpsControllerPlugin,
            PhysicsPlugin,
        ))
        
        // Rendering systems
        .add_plugins(LightingPlugin)
        
        // Content systems
        .add_plugins((
            AssetLoadingPlugin,
            MapLoadingPlugin,
        ))
        
        // Gameplay systems
        .add_plugins((
            WeaponPlugin,
            InteractionPlugin,
            AudioPlugin,
        ))
        
        // UI systems
        .add_plugins((
            GameUIPlugin,
            MultiplayerPlugin,
        ))
        
        // Game setup
        .add_plugins(GameSetupPlugin)
        
        // Input handling
        .add_systems(Update, handle_input);

    app.run();
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}