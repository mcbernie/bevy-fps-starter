use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use avian3d::prelude::*;

pub struct FpsControllerPlugin;

impl Plugin for FpsControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_fps_controller)
            .add_systems(FixedUpdate, (
                fps_controller_look,       // kann auch in Update bleiben, aber hier ok
                fps_controller_move,
                check_grounded,
                toggle_cursor_lock,        // kann in Update bleiben, funktioniert aber auch hier
            ));
    }
}

#[derive(Component)]
pub struct FpsController {
    pub speed: f32,
    pub sensitivity: f32,
    pub enabled: bool,
    pub jump_height: f32,
    pub is_grounded: bool,
    pub coyote_time: f32,
    pub jump_buffer_time: f32,
    pub air_control: f32,
    pub accel_ground: f32,
    pub accel_air: f32,
    pub max_slope_deg: f32,
    pub skin_width: f32,

    // Anti-Mehrfachsprung:
    pub jump_was_released: bool, // Space muss einmal losgelassen werden
    pub jump_locked: bool,       // bis wieder stabil am Boden
    pub ground_frames: u8,       // wie viele Frames hintereinander Boden
    pub max_air_jumps: u8,       // 0 = kein Double-Jump
    pub used_air_jumps: u8,      // Anzahl in aktueller Luftphase

    // Timer intern:
    pub coyote_timer: f32,
    pub jump_buffer_timer: f32,
}

impl Default for FpsController {
    fn default() -> Self {
        Self {
            speed: 8.0,                 // Increased from 7.0 for more responsive movement
            sensitivity: 0.002,
            enabled: true,
            jump_height: 1.8,           // Increased from 1.4 for higher jumps
            is_grounded: false,
            coyote_time: 0.12,
            jump_buffer_time: 0.12,
            air_control: 0.45,          // Increased from 0.35 for better air control
            accel_ground: 50.0,         // Increased from 40.0 for faster acceleration
            accel_air: 15.0,            // Increased from 10.0 for better air movement
            max_slope_deg: 50.0,
            skin_width: 0.06,

            jump_was_released: true,
            jump_locked: false,
            ground_frames: 0,
            max_air_jumps: 0,
            used_air_jumps: 0,

            coyote_timer: 0.0,
            jump_buffer_timer: -1.0,
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
    if let Ok(mut window) = windows.single_mut() {
        window.cursor_options.grab_mode = bevy::window::CursorGrabMode::Locked;
        window.cursor_options.visible = false;
    }
    
    commands.insert_resource(CursorLocked(true));

    // Spawn FPS camera with physics and weapon inventory
    let player_entity = commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.8, 5.0),
        FpsController::default(),
        RigidBody::Dynamic,
        Collider::capsule(0.4, 1.8), // Capsule collider for player
        Mass(70.0), // Player mass in kg
        LockedAxes::ROTATION_LOCKED, // Prevent physics rotation
        Friction::new(0.1),
        Restitution::new(0.0),
        crate::gameplay::weapons::PlayerInventory::default(),
        crate::gameplay::interaction::PlayerHealth::default(),
        crate::gameplay::audio::FootstepEmitter::default(),
    )).id();
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
        if let Ok(controller) = controller_query.single() {
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
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut LinearVelocity, &Transform, &mut FpsController)>,
) {
    let dt = time.delta_secs();

    for (mut lv, transform, mut ctrl) in &mut query {
        if !ctrl.enabled { continue; }

        // --- Jump-Input handling
        // Taste losgelassen?
        if !keyboard.pressed(KeyCode::Space) {
            ctrl.jump_was_released = true;
        }

        // Buffer aktualisieren
        if keyboard.just_pressed(KeyCode::Space) {
            ctrl.jump_buffer_timer = ctrl.jump_buffer_time;
        } else {
            ctrl.jump_buffer_timer -= dt;
        }

        // --- Bewegungswunsch (wie gehabt, hier gekürzt)
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0.0, local_z.z);
        let right   =  Vec3::new(local_z.z, 0.0, -local_z.x);

        let mut wish = Vec3::ZERO;
        if keyboard.pressed(KeyCode::KeyW) { wish += forward; }
        if keyboard.pressed(KeyCode::KeyS) { wish -= forward; }
        if keyboard.pressed(KeyCode::KeyA) { wish -= right; }
        if keyboard.pressed(KeyCode::KeyD) { wish += right; }
        if wish.length_squared() > 0.0 { wish = wish.normalize() * ctrl.speed; }

        let target_v = Vec2::new(wish.x, wish.z);
        let curr_v   = Vec2::new(lv.x, lv.z);
        
        // Improved acceleration system
        if ctrl.is_grounded {
            // Ground movement with better deceleration
            if target_v.length_squared() > 0.0 {
                // Accelerating
                let accel = ctrl.accel_ground;
                let dv = (target_v - curr_v).clamp_length_max(accel * dt);
                lv.x += dv.x;
                lv.z += dv.y;
            } else {
                // Decelerating when no input
                let decel = ctrl.accel_ground * 1.5; // Faster deceleration for more responsive feel
                let dv = curr_v.clamp_length_max(decel * dt);
                lv.x -= dv.x;
                lv.z -= dv.y;
            }
        } else {
            // Air movement with reduced control
            let air_accel = ctrl.accel_air.max(ctrl.accel_ground * ctrl.air_control);
            let dv = (target_v - curr_v).clamp_length_max(air_accel * dt);
            lv.x += dv.x;
            lv.z += dv.y;
        }

        // --- Sprungbedingungen
        let buffer_ok = ctrl.jump_buffer_timer >= 0.0;
        let ground_ok = ctrl.is_grounded || ctrl.coyote_timer > 0.0;
        let air_ok    = !ground_ok && (ctrl.used_air_jumps < ctrl.max_air_jumps);
        let input_ok  = ctrl.jump_was_released;      // Taste seit letztem Sprung losgelassen
        let lock_ok   = !ctrl.jump_locked;           // nicht gesperrt (bis wieder Boden)

        let can_jump = buffer_ok && (ground_ok || air_ok) && input_ok && lock_ok;

        if can_jump {
            // Improved jump calculation for more responsive jumping
            let g = 12.0; // Increased gravity for faster falling (was 9.81)
            let v0 = (2.0 * g * ctrl.jump_height).sqrt();

            // Down-V eliminieren für konsistente Höhe
            if lv.y < 0.0 { lv.y = 0.0; }
            lv.y = v0;

            // Zustände updaten
            ctrl.jump_buffer_timer = -1.0;
            ctrl.coyote_timer = 0.0;
            ctrl.jump_was_released = false;
            ctrl.jump_locked = true; // bleibt true, bis wieder stabil Boden
            if !ground_ok { ctrl.used_air_jumps += 1; }
        }

        // Improved gravity and air movement
        if !ctrl.is_grounded {
            // Apply stronger gravity for faster falling
            let gravity = 12.0; // Increased from 9.81
            lv.y -= gravity * dt;
            
            // Optional: Faster fall when holding shift
            if keyboard.pressed(KeyCode::ShiftLeft) {
                lv.y -= gravity * 1.5 * dt; // Extra downward force
            }
            
            // Terminal velocity cap
            lv.y = lv.y.max(-25.0); // Faster terminal velocity (was no cap)
        } else {
            // Light ground adhesion (only when really grounded)
            if lv.y < 0.0 {
                lv.y = lv.y.max(-2.0);
            }
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
        
        if let Ok(mut window) = windows.single_mut() {
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
    time: Res<Time>,
    mut controllers: Query<(&Transform, &Collider, &mut FpsController, Option<&LinearVelocity>)>,
    spatial_query: SpatialQuery,
) {
    for (transform, collider, mut ctrl, lv_opt) in controllers.iter_mut() {
        ctrl.coyote_timer -= time.delta_secs();

        // Collider-Maße (optional: aus shape() auslesen; ansonsten fest)
        let mut radius = 0.4;
        let mut half_height = 0.9;
        if let Some(capsule) = collider.shape().as_capsule() {
            radius = capsule.radius;
            half_height = capsule.half_height();
        }

        let foot_y = transform.translation.y - (half_height + radius);
        let skin = ctrl.skin_width.max(0.02);
        let start = Vec3::new(transform.translation.x, foot_y + skin, transform.translation.z);

        let hit_ok = if let Some(hit) = spatial_query.cast_ray(
            start,
            Dir3::NEG_Y,
            skin * 2.0,
            false,
            &SpatialQueryFilter::default(),
        ) {
            let max_ny = (ctrl.max_slope_deg.to_radians()).cos();
            hit.normal.y >= max_ny
        } else {
            false
        };

        // Vertikaltrend beachten (optional, verhindert Ground bei starkem Aufwärtsflug)
        let going_up_fast = lv_opt.map_or(false, |lv| lv.y > 1.0);

        if hit_ok && !going_up_fast {
            ctrl.ground_frames = (ctrl.ground_frames + 1).min(3);
        } else {
            if ctrl.ground_frames > 0 {
                // Übergang Boden -> Luft: Coyote starten
                ctrl.coyote_timer = ctrl.coyote_time;
            }
            ctrl.ground_frames = 0;
        }

        let was_grounded = ctrl.is_grounded;
        ctrl.is_grounded = ctrl.ground_frames >= 2;

        // Reset für Luft-Sprünge und Jump-Lock, wenn wir „wirklich“ wieder am Boden sind
        if ctrl.is_grounded && !was_grounded {
            ctrl.used_air_jumps = 0;
            ctrl.jump_locked = false; // erst am Boden wird wieder entsperrt
        }
        
        // Ensure jump lock is maintained until we're stable on ground
        if !ctrl.is_grounded && ctrl.ground_frames == 0 {
            // Still in air, keep jump locked if it was a recent jump
            if !ctrl.jump_was_released {
                ctrl.jump_locked = true;
            }
        }
    }
}
