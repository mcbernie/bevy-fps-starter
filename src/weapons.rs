use bevy::prelude::*;
use avian3d::prelude::*;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_weapon_system)
            .add_systems(Update, (
                weapon_pickup_system,
                weapon_usage_system,
                update_weapon_sway,
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

#[derive(Clone, Debug)]
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

fn setup_weapon_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create some weapon pickups in the world
    spawn_weapon_pickup(
        &mut commands,
        &mut meshes,
        &mut materials,
        WeaponType::Pistol,
        Vec3::new(-5.0, 1.0, -5.0),
        30,
    );

    spawn_weapon_pickup(
        &mut commands,
        &mut meshes,
        &mut materials,
        WeaponType::Rifle,
        Vec3::new(5.0, 1.0, -5.0),
        90,
    );

    spawn_weapon_pickup(
        &mut commands,
        &mut meshes,
        &mut materials,
        WeaponType::Shotgun,
        Vec3::new(0.0, 1.0, -8.0),
        12,
    );
}

fn spawn_weapon_pickup(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    weapon_type: WeaponType,
    position: Vec3,
    ammo: u32,
) {
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
    ));
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

// Utility function to add weapon inventory to player
pub fn add_weapon_inventory_to_player(commands: &mut Commands, player_entity: Entity) {
    commands.entity(player_entity).insert(PlayerInventory::default());
}