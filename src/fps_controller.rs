use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

pub struct FpsControllerPlugin;

impl Plugin for FpsControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_fps_controller)
            .add_systems(Update, (
                fps_controller_look,
                fps_controller_move,
                toggle_cursor_lock,
            ));
    }
}

#[derive(Component)]
pub struct FpsController {
    pub speed: f32,
    pub sensitivity: f32,
    pub enabled: bool,
}

impl Default for FpsController {
    fn default() -> Self {
        Self {
            speed: 10.0,
            sensitivity: 0.002,
            enabled: true,
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

    // Spawn FPS camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.8, 5.0),
        FpsController::default(),
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
    mut query: Query<&mut Transform, With<FpsController>>,
    controller_query: Query<&FpsController>,
) {
    for mut transform in query.iter_mut() {
        if let Ok(controller) = controller_query.get_single() {
            if !controller.enabled {
                continue;
            }

            let mut velocity = Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = -Vec3::new(local_z.x, 0.0, local_z.z);
            let right = Vec3::new(local_z.z, 0.0, -local_z.x);

            // Movement input
            if keyboard_input.pressed(KeyCode::KeyW) {
                velocity += forward;
            }
            if keyboard_input.pressed(KeyCode::KeyS) {
                velocity -= forward;
            }
            if keyboard_input.pressed(KeyCode::KeyA) {
                velocity -= right;
            }
            if keyboard_input.pressed(KeyCode::KeyD) {
                velocity += right;
            }
            if keyboard_input.pressed(KeyCode::Space) {
                velocity += Vec3::Y;
            }
            if keyboard_input.pressed(KeyCode::ShiftLeft) {
                velocity -= Vec3::Y;
            }

            velocity = velocity.normalize_or_zero();
            transform.translation += velocity * controller.speed * time.delta_secs();
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