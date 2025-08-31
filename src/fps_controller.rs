use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use avian3d::prelude::*;

pub struct FpsControllerPlugin;

impl Plugin for FpsControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_fps_controller)
            .add_systems(Update, (
                fps_controller_look,
                fps_controller_move,
                check_grounded,
                toggle_cursor_lock,
            ));
    }
}

#[derive(Component)]
pub struct FpsController {
    pub speed: f32,
    pub sensitivity: f32,
    pub enabled: bool,
    pub jump_force: f32,
    pub is_grounded: bool,
}

impl Default for FpsController {
    fn default() -> Self {
        Self {
            speed: 10.0,
            sensitivity: 0.002,
            enabled: true,
            jump_force: 300.0,
            is_grounded: false,
        }
    }
}

#[derive(Resource)]
pub struct CursorLocked(pub bool);

fn setup_fps_controller(
    mut commands: Commands,
    mut windows: Query<&mut Window>,
) {
    // Lock cursor by default
    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor_options.grab_mode = bevy::window::CursorGrabMode::Locked;
        window.cursor_options.visible = false;
    }
    
    commands.insert_resource(CursorLocked(true));

    // Spawn FPS camera with physics
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.8, 5.0),
        FpsController::default(),
        RigidBody::Dynamic,
        Collider::capsule(0.4, 1.8), // Capsule collider for player
        Mass(70.0), // Player mass in kg
        LockedAxes::ROTATION_LOCKED, // Prevent physics rotation
        Friction::new(0.1),
        Restitution::new(0.0),
    ));
}

fn fps_controller_look(
    mut mouse_motion: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<FpsController>>,
    controller_query: Query<&FpsController>,
    cursor_locked: Res<CursorLocked>,
) {
    if !cursor_locked.0 {
        return;
    }

    let mut mouse_delta = Vec2::ZERO;
    for motion in mouse_motion.read() {
        mouse_delta += motion.delta;
    }

    for mut transform in query.iter_mut() {
        if let Ok(controller) = controller_query.get_single() {
            if !controller.enabled {
                continue;
            }

            // Horizontal rotation (yaw)
            transform.rotate_y(-mouse_delta.x * controller.sensitivity);
            
            // Vertical rotation (pitch)
            let pitch_delta = -mouse_delta.y * controller.sensitivity;
            let current_pitch = transform.rotation.to_euler(EulerRot::YXZ).1;
            let new_pitch = (current_pitch + pitch_delta).clamp(-1.5, 1.5);
            
            // Reset rotation and apply yaw then pitch
            let yaw = transform.rotation.to_euler(EulerRot::YXZ).0;
            transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, new_pitch, 0.0);
        }
    }
}

fn fps_controller_move(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut LinearVelocity, &Transform, &mut FpsController), With<FpsController>>,
) {
    for (mut linear_velocity, transform, mut controller) in query.iter_mut() {
        if !controller.enabled {
            continue;
        }

        // Calculate movement direction
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0.0, local_z.z);
        let right = Vec3::new(local_z.z, 0.0, -local_z.x);

        let mut movement = Vec3::ZERO;

        // Movement input
        if keyboard_input.pressed(KeyCode::KeyW) {
            movement += forward;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            movement -= forward;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            movement -= right;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            movement += right;
        }

        // Normalize movement and apply speed
        if movement.length() > 0.0 {
            movement = movement.normalize() * controller.speed;
        }

        // Apply horizontal movement, preserve vertical velocity
        linear_velocity.x = movement.x;
        linear_velocity.z = movement.z;

        // Jumping - only when grounded and space is just pressed
        if keyboard_input.just_pressed(KeyCode::Space) && controller.is_grounded {
            linear_velocity.y = controller.jump_force;
        }

        // Crouch/downward movement (ShiftLeft)
        if keyboard_input.pressed(KeyCode::ShiftLeft) && !controller.is_grounded {
            linear_velocity.y -= controller.speed * 2.0;
        }
    }
}

fn toggle_cursor_lock(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window>,
    mut cursor_locked: ResMut<CursorLocked>,
) {
    if keyboard_input.just_pressed(KeyCode::Tab) {
        cursor_locked.0 = !cursor_locked.0;
        
        if let Ok(mut window) = windows.get_single_mut() {
            if cursor_locked.0 {
                window.cursor_options.grab_mode = bevy::window::CursorGrabMode::Locked;
                window.cursor_options.visible = false;
            } else {
                window.cursor_options.grab_mode = bevy::window::CursorGrabMode::None;
                window.cursor_options.visible = true;
            }
        }
    }
}

fn check_grounded(
    mut controllers: Query<(&Transform, &mut FpsController)>,
    spatial_query: SpatialQuery,
) {
    for (transform, mut controller) in controllers.iter_mut() {
        // Cast a ray downward to check if player is grounded
        let ray_start = transform.translation;
        let ray_direction = Dir3::NEG_Y;
        let ray_distance = 1.0; // Check 1 unit below player
        
        if let Some(_hit) = spatial_query.cast_ray(
            ray_start,
            ray_direction,
            ray_distance,
            false,
            &SpatialQueryFilter::default(),
        ) {
            controller.is_grounded = true;
        } else {
            controller.is_grounded = false;
        }
    }
}