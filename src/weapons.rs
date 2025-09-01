use std::{collections::VecDeque, time::Duration};

use bevy::{prelude::*, scene::SceneInstanceReady};
use avian3d::prelude::*;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<WeaponAnimSet>()
            .add_systems(Startup, setup_weapon_system)
            .add_systems(Update, (
                weapon_pickup_system,
                weapon_usage_system,
                //update_weapon_sway,
                spawn_held_weapon_view,
                on_scene_ready_mark_player,
                start_idle_when_ready,
                build_anim_graph,
            ));
    }
}

#[derive(Component)]
pub struct Weapon {
    pub name: String,
    pub damage: f32,
    pub fire_rate: f32,
    pub ammo: u32,
    pub max_ammo: u32,
    pub range: f32,
    pub last_shot: f32,
}

#[derive(Component)]
pub struct WeaponPickup {
    pub weapon_type: WeaponType,
    pub ammo_count: u32,
}

#[derive(Component)]
pub struct PlayerInventory {
    pub held_weapon: Option<Entity>,
    pub weapons: Vec<Entity>,
}

#[derive(Component)]
pub struct WeaponSway {
    pub base_position: Vec3,
    pub base_rotation: Quat,
    pub sway_intensity: f32,
    pub bob_intensity: f32,
}

#[derive(Clone, Debug, Copy)]
pub enum WeaponType {
    Pistol,
    Rifle,
    Shotgun,
}

impl Default for PlayerInventory {
    fn default() -> Self {
        Self {
            held_weapon: None,
            weapons: Vec::new(),
        }
    }
}

impl Default for WeaponSway {
    fn default() -> Self {
        Self {
            base_position: Vec3::new(0.5, -0.3, -0.8),
            base_rotation: Quat::IDENTITY,
            sway_intensity: 0.02,
            bob_intensity: 0.01,
        }
    }
}

#[derive(Component)]
struct WeaponAnimRoot;      // Marker am Root der View-Model-Scene
#[derive(Component)]
struct WeaponAnimPlayer;    // Marker am echten AnimationPlayer-Entity




fn setup_weapon_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_assets: Res<crate::assets::GameAssets>,
) {
    // Create some weapon pickups in the world
    spawn_weapon_pickup(
        &mut commands,
        &mut meshes,
        &mut materials,
        &game_assets,
        WeaponType::Pistol,
        Vec3::new(-5.0, 1.0, -5.0),
        30,
    );

    spawn_weapon_pickup(
        &mut commands,
        &mut meshes,
        &mut materials,
        &game_assets,
        WeaponType::Rifle,
        Vec3::new(5.0, 1.0, -5.0),
        90,
    );

    spawn_weapon_pickup(
        &mut commands,
        &mut meshes,
        &mut materials,
        &game_assets,
        WeaponType::Shotgun,
        Vec3::new(0.0, 1.0, -8.0),
        12,
    );
}

fn spawn_weapon_pickup(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    game_assets: &crate::assets::GameAssets,
    weapon_type: WeaponType,
    position: Vec3,
    ammo: u32,
) {
    let _pickup_entity = if game_assets.assets_loaded {
        // Use actual weapon model for rifle (since we have the saiga model)
        match weapon_type {
            /*WeaponType::Rifle => {
                commands.spawn((
                    SceneRoot(game_assets.weapon_model.clone()),
                    Transform::from_translation(position).with_scale(Vec3::splat(0.3)),
                    RigidBody::Static,
                    Collider::cuboid(0.2, 0.2, 1.0), // Approximate collider
                    WeaponPickup {
                        weapon_type,
                        ammo_count: ammo,
                    },
                    crate::assets::WeaponModel,
                )).id()
            },*/
            _ => {
                // For other weapons, use placeholder shapes
                let (color, size) = match weapon_type {
                    WeaponType::Pistol => (Color::srgb(0.7, 0.7, 0.9), Vec3::new(0.3, 0.2, 0.6)),
                    WeaponType::Shotgun => (Color::srgb(0.6, 0.4, 0.2), Vec3::new(0.25, 0.25, 1.0)),
                    _ => (Color::srgb(0.5, 0.5, 0.5), Vec3::new(0.2, 0.2, 0.8)),
                };

                commands.spawn((
                    Mesh3d(meshes.add(Cuboid::from_size(size))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: color,
                        metallic: 0.8,
                        perceptual_roughness: 0.2,
                        ..default()
                    })),
                    Transform::from_translation(position),
                    RigidBody::Static,
                    Collider::cuboid(size.x, size.y, size.z),
                    WeaponPickup {
                        weapon_type,
                        ammo_count: ammo,
                    },
                )).id()
            }
        }
    } else {
        // Fallback to placeholder shapes if assets not loaded
        let (color, size) = match weapon_type {
            WeaponType::Pistol => (Color::srgb(0.7, 0.7, 0.9), Vec3::new(0.3, 0.2, 0.6)),
            WeaponType::Rifle => (Color::srgb(0.3, 0.3, 0.3), Vec3::new(0.2, 0.2, 1.2)),
            WeaponType::Shotgun => (Color::srgb(0.6, 0.4, 0.2), Vec3::new(0.25, 0.25, 1.0)),
        };

        commands.spawn((
            Mesh3d(meshes.add(Cuboid::from_size(size))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                metallic: 0.8,
                perceptual_roughness: 0.2,
                ..default()
            })),
            Transform::from_translation(position),
            RigidBody::Static,
            Collider::cuboid(size.x, size.y, size.z),
            WeaponPickup {
                weapon_type,
                ammo_count: ammo,
            },
        )).id()
    };

    info!("Spawned {:?} weapon pickup at {:?}", weapon_type, position);
}

fn weapon_pickup_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Transform, &mut PlayerInventory), With<crate::fps_controller::FpsController>>,
    pickup_query: Query<(Entity, &Transform, &WeaponPickup), Without<crate::fps_controller::FpsController>>,
    time: Res<Time>,
) {
    if !keyboard_input.just_pressed(KeyCode::KeyE) {
        return;
    }

    for (player_transform, mut inventory) in player_query.iter_mut() {
        for (pickup_entity, pickup_transform, weapon_pickup) in pickup_query.iter() {
            let distance = player_transform.translation.distance(pickup_transform.translation);
            
            if distance <= 2.0 { // Within pickup range
                // Create weapon entity
                let weapon = match weapon_pickup.weapon_type {
                    WeaponType::Pistol => Weapon {
                        name: "Pistol".to_string(),
                        damage: 25.0,
                        fire_rate: 0.3,
                        ammo: weapon_pickup.ammo_count,
                        max_ammo: 60,
                        range: 50.0,
                        last_shot: 0.0,
                    },
                    WeaponType::Rifle => Weapon {
                        name: "Rifle".to_string(),
                        damage: 45.0,
                        fire_rate: 0.1,
                        ammo: weapon_pickup.ammo_count,
                        max_ammo: 120,
                        range: 100.0,
                        last_shot: 0.0,
                    },
                    WeaponType::Shotgun => Weapon {
                        name: "Shotgun".to_string(),
                        damage: 80.0,
                        fire_rate: 0.8,
                        ammo: weapon_pickup.ammo_count,
                        max_ammo: 24,
                        range: 15.0,
                        last_shot: 0.0,
                    },
                };

                let weapon_entity = commands.spawn(weapon).id();
                
                // Add to inventory
                inventory.weapons.push(weapon_entity);
                if inventory.held_weapon.is_none() {
                    inventory.held_weapon = Some(weapon_entity);
                }

                // Remove pickup
                commands.entity(pickup_entity).despawn();
                
                info!("Picked up {} with {} ammo", 
                    match weapon_pickup.weapon_type {
                        WeaponType::Pistol => "Pistol",
                        WeaponType::Rifle => "Rifle", 
                        WeaponType::Shotgun => "Shotgun",
                    },
                    weapon_pickup.ammo_count
                );
                break;
            }
        }
    }
}

fn weapon_usage_system(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut player_query: Query<(&Transform, &mut PlayerInventory), With<crate::fps_controller::FpsController>>,
    mut weapon_query: Query<&mut Weapon>,
    time: Res<Time>,
    spatial_query: SpatialQuery,
) {
    for (player_transform, mut inventory) in player_query.iter_mut() {
        if let Some(weapon_entity) = inventory.held_weapon {
            if let Ok(mut weapon) = weapon_query.get_mut(weapon_entity) {
                let current_time = time.elapsed_secs();
                
                // Check if we can fire (fire rate cooldown)
                if mouse_input.pressed(MouseButton::Left) 
                    && current_time - weapon.last_shot >= weapon.fire_rate 
                    && weapon.ammo > 0 {
                    
                    // Fire weapon
                    fire_weapon(&mut weapon, player_transform, &spatial_query, current_time);
                }
            }
        }
    }
}

fn fire_weapon(
    weapon: &mut Weapon,
    player_transform: &Transform,
    spatial_query: &SpatialQuery,
    current_time: f32,
) {
    weapon.ammo -= 1;
    weapon.last_shot = current_time;
    
    // Cast ray from player forward
    let ray_start = player_transform.translation;
    let ray_direction = -player_transform.local_z().normalize();
    
    if let Ok(ray_dir) = Dir3::new(ray_direction) {
        if let Some(hit) = spatial_query.cast_ray(
            ray_start,
            ray_dir,
            weapon.range,
            true,
            &SpatialQueryFilter::default(),
        ) {
            info!("Hit target at distance: {:.2} with {}", hit.distance, weapon.name);
            // Here you could add impact effects, damage to enemies, etc.
        }
    }
    
    info!("Fired {} - Ammo remaining: {}", weapon.name, weapon.ammo);
}

fn update_weapon_sway(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut weapon_query: Query<(&mut Transform, &mut WeaponSway), Without<crate::fps_controller::FpsController>>,
) {
    for (mut transform, mut sway) in weapon_query.iter_mut() {
        let elapsed = time.elapsed_secs();
        
        // Calculate weapon bob based on movement
        let is_moving = input.pressed(KeyCode::KeyW) 
            || input.pressed(KeyCode::KeyS) 
            || input.pressed(KeyCode::KeyA) 
            || input.pressed(KeyCode::KeyD);
            
        let bob_multiplier = if is_moving { 1.0 } else { 0.3 };
        
        // Apply weapon sway and bob
        let bob_y = (elapsed * 4.0).sin() * sway.bob_intensity * bob_multiplier;
        let bob_x = (elapsed * 2.0).sin() * sway.bob_intensity * 0.5 * bob_multiplier;
        
        let new_position = sway.base_position + Vec3::new(bob_x, bob_y, 0.0);
        transform.translation = new_position;
    }
}

fn spawn_held_weapon_view(
    mut commands: Commands,
    game_assets: Res<crate::assets::GameAssets>,
    weapon_arm_asset: Res<crate::assets::WeaponArm>,
    player_query: Query<(Entity, &PlayerInventory), (With<crate::fps_controller::FpsController>, Changed<PlayerInventory>)>,
    weapon_query: Query<&Weapon>,
    existing_view_weapons: Query<Entity, With<HeldWeaponView>>,
) {
    for (player_entity, inventory) in player_query.iter() {
        // Remove existing weapon view
        for entity in existing_view_weapons.iter() {
            commands.entity(entity).despawn();
        }

        // Spawn new weapon view if player has a weapon
        if let Some(weapon_entity) = inventory.held_weapon {
            if let Ok(weapon) = weapon_query.get(weapon_entity) {
                if game_assets.assets_loaded {
                    let weapon_asset = weapon_arm_asset.scene.clone();
                    // Spawn the weapon model in first-person view
                    let weapon_view_entity = commands.spawn((
                        SceneRoot(weapon_asset),
                        Transform::from_translation(Vec3::new(0.2, -0.6, -0.2))
                            .with_scale(Vec3::splat(1.0))
                            .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
                        WeaponSway::default(),
                        HeldWeaponView,
                        WeaponAnimRoot,
                        Name::new(format!("HeldWeaponView - {}", weapon.name)),
                    )).id();

                    // Make the weapon view a child of the player camera
                    commands.entity(player_entity).add_children(&[weapon_view_entity]);
                    
                    info!("Spawned weapon view for {}", weapon.name);   
                }
            }
        }
    }
}

#[derive(Component)]
pub struct HeldWeaponView;

// Utility function to add weapon inventory to player
pub fn add_weapon_inventory_to_player(commands: &mut Commands, player_entity: Entity) {
    commands.entity(player_entity).insert(PlayerInventory::default());
}

fn on_scene_ready_mark_player(
    //mut ev: EventReader<SceneInstanceReady>,
    roots: Query<Entity, (With<WeaponAnimRoot>, Without<WeaponAnimPlayer>)>,
    children_q: Query<&Children>,
    players_q: Query<Entity, With<AnimationPlayer>>,
    mut commands: Commands,
) {
    for root in &roots {
        if let Some(player_entity) = find_descendant_with::<AnimationPlayer>(root, &children_q, &players_q) {
            commands.entity(player_entity).insert(WeaponAnimPlayer);
        }
    }
}

/// Suche im Subtree von `root` die erste Entity mit Komponente `T`
fn find_descendant_with<T: Component>(
    root: Entity,
    children_q: &Query<&Children>,
    want_q: &Query<Entity, With<T>>,
) -> Option<Entity> {
    let mut bfs: VecDeque<Entity> = VecDeque::from([root]); // <- explizit Entity
    while let Some(e) = bfs.pop_front() {
        if want_q.get(e).is_ok() {
            return Some(e);
        }
        if let Ok(children) = children_q.get(e) {
            // NICHT extend(); einfach pushen:
            for child in children.iter() {
                bfs.push_back(child);
            }
        }
    }
    None
}

fn start_idle_when_ready(
    mut cmds: Commands,
    mut q: Query<(Entity, &mut AnimationPlayer), Added<WeaponAnimPlayer>>,
    anims: Res<WeaponAnimSet>,
) {
    if let Some(idle_animation) = anims.idle {
        for (e, mut player) in &mut q {
            let mut transitions = AnimationTransitions::new();
            transitions.play(&mut player, idle_animation, Duration::ZERO).repeat();

            cmds.entity(e)
                .insert(AnimationGraphHandle(anims.graph.clone()))
                .insert(transitions);
        }
    }
}

#[derive(Resource, Clone, Default)]
pub struct WeaponAnimSet {
    pub graph: Handle<AnimationGraph>,
    pub idle: Option<AnimationNodeIndex>,
    pub fire: Option<AnimationNodeIndex>,
    pub walk: Option<AnimationNodeIndex>,
    pub reload: Option<AnimationNodeIndex>,
    pub reload_fast: Option<AnimationNodeIndex>,
}

// Beim Assets-Ready-Event:
fn build_anim_graph(
    weapon: Res<crate::assets::WeaponArm>, // enthält deine Clip-Handles
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut out: ResMut<WeaponAnimSet>,
) {
    // Reihenfolge festlegen -> zuordnen
    let clip_order: Vec<Handle<AnimationClip>> = vec![
        weapon.idle.clone(),
        weapon.walk.clone(),
        weapon.fire.clone(),
        weapon.reload.clone(),
        weapon.reload_fast.clone()
    ];

    let (graph, nodes) = AnimationGraph::from_clips(clip_order);
    let graph_handle = graphs.add(graph);

    let idle = Some(nodes[0]);
    let walk = Some(nodes[1]);
    let fire = Some(nodes[2]);
    let reload = Some(nodes[3]);
    let reload_fast = Some(nodes[4]);

    *out = WeaponAnimSet { 
        graph: graph_handle, 
        idle, walk, fire, reload, reload_fast
    };
}