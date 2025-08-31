use bevy::prelude::*;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_ui)
            .add_systems(Update, (
                update_fps_display,
                update_crosshair,
                handle_ui_input,
                update_debug_info,
            ));
    }
}

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct Crosshair;

#[derive(Component)]
pub struct DebugInfo;

#[derive(Resource)]
pub struct UISettings {
    pub show_fps: bool,
    pub show_crosshair: bool,
    pub show_debug: bool,
}

impl Default for UISettings {
    fn default() -> Self {
        Self {
            show_fps: true,
            show_crosshair: true,
            show_debug: false,
        }
    }
}

fn setup_ui(mut commands: Commands) {
    commands.insert_resource(UISettings::default());
    
    // Simple UI - just crosshair for now
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        })
        .with_children(|parent| {
            // Crosshair (center)
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(20.0),
                    height: Val::Px(20.0),
                    left: Val::Percent(50.0),
                    top: Val::Percent(50.0),
                    margin: UiRect::new(Val::Px(-10.0), Val::Px(0.0), Val::Px(-10.0), Val::Px(0.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
                BorderColor(Color::srgb(1.0, 1.0, 1.0)),
                Crosshair,
            ))
            .with_children(|crosshair| {
                // Horizontal line
                crosshair.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Px(12.0),
                        height: Val::Px(2.0),
                        left: Val::Px(4.0),
                        top: Val::Px(9.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(1.0, 1.0, 1.0)),
                ));
                
                // Vertical line
                crosshair.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Px(2.0),
                        height: Val::Px(12.0),
                        left: Val::Px(9.0),
                        top: Val::Px(4.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(1.0, 1.0, 1.0)),
                ));
            });
        });
}

fn update_fps_display(
    // Simplified for now without text dependency
) {
    // TODO: Re-implement when text system is properly configured
}

fn update_crosshair(
    mut query: Query<&mut Node, With<Crosshair>>,
    ui_settings: Res<UISettings>,
) {
    for mut style in &mut query {
        style.display = if ui_settings.show_crosshair {
            Display::Flex
        } else {
            Display::None
        };
    }
}

fn update_debug_info(
    // Simplified for now without text dependency
) {
    // TODO: Re-implement when text system is properly configured
}

fn handle_ui_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ui_settings: ResMut<UISettings>,
) {
    if keyboard_input.just_pressed(KeyCode::F11) {
        ui_settings.show_debug = !ui_settings.show_debug;
    }
    
    if keyboard_input.just_pressed(KeyCode::F12) {
        ui_settings.show_fps = !ui_settings.show_fps;
    }
}

// Utility functions for UI
pub fn create_button(
    commands: &mut Commands,
    text: &str,
    position: (Val, Val),
) -> Entity {
    commands.spawn((
        Button,
        Node {
            position_type: PositionType::Absolute,
            left: position.0,
            top: position.1,
            width: Val::Px(150.0),
            height: Val::Px(40.0),
            border: UiRect::all(Val::Px(2.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        BorderColor(Color::srgb(0.5, 0.5, 0.5)),
    )).id()
}