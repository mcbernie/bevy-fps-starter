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
                update_health_display,
                update_ammo_display,
                update_weapon_display,
            ));
    }
}

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct Crosshair;

#[derive(Component)]
pub struct DebugInfo;

#[derive(Component)]
pub struct HealthDisplay;

#[derive(Component)]
pub struct AmmoDisplay;

#[derive(Component)]
pub struct WeaponDisplay;

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
    
    // Main UI container
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

            // Health display (bottom left)
            parent.spawn((
                Text::new("Health: 100/100"),
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(20.0),
                    left: Val::Px(20.0),
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.2, 0.2)),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                HealthDisplay,
            ));

            // Ammo display (bottom right)
            parent.spawn((
                Text::new("Ammo: -/-"),
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(20.0),
                    right: Val::Px(20.0),
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.2)),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                AmmoDisplay,
            ));

            // Weapon display (bottom center)
            parent.spawn((
                Text::new("No Weapon"),
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(60.0),
                    left: Val::Percent(50.0),
                    margin: UiRect::new(Val::Px(-75.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0)),
                    ..default()
                },
                TextColor(Color::srgb(0.2, 0.8, 0.2)),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                WeaponDisplay,
            ));
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

fn update_health_display(
    player_query: Query<&crate::interaction::PlayerHealth, With<crate::fps_controller::FpsController>>,
    mut health_display_query: Query<&mut Text, With<HealthDisplay>>,
) {
    for player_health in player_query.iter() {
        for mut text in health_display_query.iter_mut() {
            text.0 = format!("Health: {:.0}/{:.0}", player_health.current, player_health.maximum);
        }
    }
}

fn update_ammo_display(
    player_query: Query<&crate::weapons::PlayerInventory, With<crate::fps_controller::FpsController>>,
    weapon_query: Query<&crate::weapons::Weapon>,
    mut ammo_display_query: Query<&mut Text, With<AmmoDisplay>>,
) {
    for inventory in player_query.iter() {
        for mut text in ammo_display_query.iter_mut() {
            if let Some(weapon_entity) = inventory.held_weapon {
                if let Ok(weapon) = weapon_query.get(weapon_entity) {
                    text.0 = format!("Ammo: {}/{}", weapon.ammo, weapon.max_ammo);
                } else {
                    text.0 = "Ammo: -/-".to_string();
                }
            } else {
                text.0 = "Ammo: -/-".to_string();
            }
        }
    }
}

fn update_weapon_display(
    player_query: Query<&crate::weapons::PlayerInventory, With<crate::fps_controller::FpsController>>,
    weapon_query: Query<&crate::weapons::Weapon>,
    mut weapon_display_query: Query<&mut Text, With<WeaponDisplay>>,
) {
    for inventory in player_query.iter() {
        for mut text in weapon_display_query.iter_mut() {
            if let Some(weapon_entity) = inventory.held_weapon {
                if let Ok(weapon) = weapon_query.get(weapon_entity) {
                    text.0 = weapon.name.clone();
                } else {
                    text.0 = "No Weapon".to_string();
                }
            } else {
                text.0 = "No Weapon".to_string();
            }
        }
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