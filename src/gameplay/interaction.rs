use bevy::prelude::*;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_interaction_system)
            .add_systems(Update, (
                handle_interaction_input,
                update_interaction_prompts,
            ));
    }
}

#[derive(Component)]
pub struct Interactable {
    pub prompt_text: String,
    pub interaction_range: f32,
    pub interaction_type: InteractionType,
}

#[derive(Component)]
pub struct InteractionPrompt {
    pub target_entity: Entity,
    pub is_visible: bool,
}

#[derive(Component)]
pub struct HealthPack {
    pub heal_amount: f32,
}

#[derive(Component)]
pub struct AmmoPack {
    pub ammo_amount: u32,
}

#[derive(Component)]
pub struct PlayerHealth {
    pub current: f32,
    pub maximum: f32,
}

#[derive(Clone, Debug)]
pub enum InteractionType {
    HealthPack,
    AmmoPack,
    Button,
    Door,
}

impl Default for PlayerHealth {
    fn default() -> Self {
        Self {
            current: 100.0,
            maximum: 100.0,
        }
    }
}

fn setup_interaction_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create some interactable items in the world
    
    // Health packs
    spawn_health_pack(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(-3.0, 0.5, 3.0),
        25.0,
    );

    spawn_health_pack(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(3.0, 0.5, 3.0),
        50.0,
    );

    // Ammo packs
    spawn_ammo_pack(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(-3.0, 0.5, -3.0),
        30,
    );

    spawn_ammo_pack(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(3.0, 0.5, -3.0),
        60,
    );

    // Spawn interaction prompt UI (initially hidden)
    commands.spawn((
        Text::new("Press E to interact"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(20.0),
            left: Val::Percent(50.0),
            margin: UiRect::new(Val::Px(-100.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0)),
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 0.0)),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        InteractionPrompt {
            target_entity: Entity::PLACEHOLDER,
            is_visible: false,
        },
        Visibility::Hidden,
    ));
}

fn spawn_health_pack(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    heal_amount: f32,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.3, 0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2),
            emissive: Color::srgb(0.3, 0.1, 0.1).into(),
            ..default()
        })),
        Transform::from_translation(position),
        Interactable {
            prompt_text: format!("Health Pack (+{} HP)", heal_amount),
            interaction_range: 2.0,
            interaction_type: InteractionType::HealthPack,
        },
        HealthPack { heal_amount },
    ));
}

fn spawn_ammo_pack(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    ammo_amount: u32,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.4, 0.2, 0.6))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.8, 0.2),
            emissive: Color::srgb(0.3, 0.3, 0.1).into(),
            ..default()
        })),
        Transform::from_translation(position),
        Interactable {
            prompt_text: format!("Ammo Pack (+{} rounds)", ammo_amount),
            interaction_range: 2.0,
            interaction_type: InteractionType::AmmoPack,
        },
        AmmoPack { ammo_amount },
    ));
}

fn handle_interaction_input(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Transform, &mut PlayerHealth, &mut crate::gameplay::weapons::PlayerInventory), With<crate::core::fps_controller::FpsController>>,
    mut interactable_query: Query<(Entity, &Transform, &Interactable, Option<&HealthPack>, Option<&AmmoPack>), Without<crate::core::fps_controller::FpsController>>,
    mut weapon_query: Query<&mut crate::gameplay::weapons::Weapon>,
) {
    if !keyboard_input.just_pressed(KeyCode::KeyE) {
        return;
    }

    for (player_transform, mut player_health, mut inventory) in player_query.iter_mut() {
        for (entity, interactable_transform, interactable, health_pack, ammo_pack) in interactable_query.iter_mut() {
            let distance = player_transform.translation.distance(interactable_transform.translation);
            
            if distance <= interactable.interaction_range {
                match interactable.interaction_type {
                    InteractionType::HealthPack => {
                        if let Some(health_pack) = health_pack {
                            if player_health.current < player_health.maximum {
                                player_health.current = (player_health.current + health_pack.heal_amount)
                                    .min(player_health.maximum);
                                    
                                info!("Used health pack! Health: {}/{}", 
                                    player_health.current, player_health.maximum);
                                
                                commands.entity(entity).despawn();
                            }
                        }
                    },
                    InteractionType::AmmoPack => {
                        if let Some(ammo_pack) = ammo_pack {
                            // Add ammo to current weapon
                            if let Some(weapon_entity) = inventory.held_weapon {
                                if let Ok(mut weapon) = weapon_query.get_mut(weapon_entity) {
                                    let old_ammo = weapon.ammo;
                                    weapon.ammo = (weapon.ammo + ammo_pack.ammo_amount).min(weapon.max_ammo);
                                    let gained = weapon.ammo - old_ammo;
                                    
                                    if gained > 0 {
                                        info!("Picked up {} ammo for {}! Total: {}/{}", 
                                            gained, weapon.name, weapon.ammo, weapon.max_ammo);
                                        commands.entity(entity).despawn();
                                    }
                                }
                            } else {
                                info!("No weapon equipped to receive ammo!");
                            }
                        }
                    },
                    _ => {
                        info!("Interacted with {}", interactable.prompt_text);
                    }
                }
                break;
            }
        }
    }
}

fn update_interaction_prompts(
    player_query: Query<&Transform, With<crate::core::fps_controller::FpsController>>,
    interactable_query: Query<(Entity, &Transform, &Interactable), Without<crate::core::fps_controller::FpsController>>,
    mut prompt_query: Query<(&mut InteractionPrompt, &mut Visibility, &mut Text)>,
) {
    for player_transform in player_query.iter() {
        let mut closest_interactable: Option<(Entity, &Interactable, f32)> = None;
        
        // Find the closest interactable within range
        for (entity, interactable_transform, interactable) in interactable_query.iter() {
            let distance = player_transform.translation.distance(interactable_transform.translation);
            
            if distance <= interactable.interaction_range {
                if let Some((_, _, closest_distance)) = closest_interactable {
                    if distance < closest_distance {
                        closest_interactable = Some((entity, interactable, distance));
                    }
                } else {
                    closest_interactable = Some((entity, interactable, distance));
                }
            }
        }
        
        // Update interaction prompt
        for (mut prompt, mut visibility, mut text) in prompt_query.iter_mut() {
            if let Some((entity, interactable, _)) = closest_interactable {
                prompt.target_entity = entity;
                prompt.is_visible = true;
                *visibility = Visibility::Visible;
                text.0 = format!("Press E - {}", interactable.prompt_text);
            } else {
                prompt.is_visible = false;
                *visibility = Visibility::Hidden;
            }
        }
    }
}

// Utility function to add player health to player entity
pub fn add_player_health_to_player(commands: &mut Commands, player_entity: Entity) {
    commands.entity(player_entity).insert(PlayerHealth::default());
}