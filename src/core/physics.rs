use bevy::prelude::*;
use avian3d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PhysicsPlugins::default())
            .add_systems(Startup, setup_physics_world)
            .add_systems(Update, handle_physics_debug);
    }
}

#[derive(Component)]
pub struct PhysicsObject;

#[derive(Component)]
pub struct StaticCollider;

#[derive(Component)]
pub struct DynamicCollider;

fn setup_physics_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add some physics objects for testing
    
    // Create a few dynamic cubes
    for i in 0..3 {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.8, 0.2, 0.2),
                ..default()
            })),
            Transform::from_xyz(0.0, 5.0 + i as f32 * 2.0, 0.0),
            RigidBody::Dynamic,
            Collider::cuboid(1.0, 1.0, 1.0),
            PhysicsObject,
            DynamicCollider,
        ));
    }

    // Add ground collider
    commands.spawn((
        Transform::from_xyz(0.0, -0.25, 0.0),
        RigidBody::Static,
        Collider::cuboid(50.0, 0.25, 50.0),
        StaticCollider,
    ));

    // Add wall colliders for the test cubes
    for i in 0..5 {
        commands.spawn((
            Transform::from_xyz(i as f32 * 4.0 - 8.0, 1.0, -10.0),
            RigidBody::Static,
            Collider::cuboid(1.0, 1.0, 1.0),
            StaticCollider,
        ));
    }
}

fn handle_physics_debug(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    // Note: PhysicsDebugConfig might not be available in avian3d 0.3
    // TODO: Check avian3d documentation for proper debug rendering setup
) {
    if keyboard_input.just_pressed(KeyCode::F3) {
        info!("Physics debug toggle pressed (debug rendering not yet implemented)");
    }
}

// Utility functions for adding physics to entities
pub fn add_static_collider(
    commands: &mut Commands,
    entity: Entity,
    collider: Collider,
) {
    commands.entity(entity).insert((
        RigidBody::Static,
        collider,
        StaticCollider,
    ));
}

pub fn add_dynamic_collider(
    commands: &mut Commands,
    entity: Entity,
    collider: Collider,
    mass: f32,
) {
    commands.entity(entity).insert((
        RigidBody::Dynamic,
        collider,
        Mass(mass),
        DynamicCollider,
        PhysicsObject,
    ));
}