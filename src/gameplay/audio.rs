use bevy::prelude::*;
use avian3d::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_audio_system)
            .add_systems(Update, (
                footstep_audio_system,
            ));
    }
}

#[derive(Component)]
pub struct FootstepEmitter {
    pub last_step_time: f32,
    pub step_interval: f32,
    pub volume: f32,
}

impl Default for FootstepEmitter {
    fn default() -> Self {
        Self {
            last_step_time: 0.0,
            step_interval: 0.5, // Steps every 0.5 seconds when moving
            volume: 0.3,
        }
    }
}

#[derive(Component)]
pub struct ImpactSound {
    pub sound_type: ImpactType,
}

#[derive(Clone)]
pub enum ImpactType {
    Metal,
    Wood,
    Glass,
    Concrete,
    Soft,
}

fn setup_audio_system() {
    info!("Audio system initialized (placeholder - audio disabled for build compatibility)");
}

fn footstep_audio_system(
    time: Res<Time>,
    mut footstep_query: Query<(&mut FootstepEmitter, &LinearVelocity)>,
) {
    for (mut emitter, velocity) in footstep_query.iter_mut() {
        let speed = velocity.length();
        
        // Only play footsteps when moving at a reasonable speed
        if speed > 1.0 {
            let current_time = time.elapsed_secs();
            
            // Adjust step interval based on speed
            let adjusted_interval = emitter.step_interval / (speed / 5.0).min(2.0);
            
            if current_time - emitter.last_step_time > adjusted_interval {
                emitter.last_step_time = current_time;
                
                // TODO: Play actual footstep sounds when audio is enabled
                // For now, just log the event
                debug!("Footstep at time: {}", current_time);
            }
        }
    }
}

// Utility function to add footstep emitter to an entity
pub fn add_footstep_emitter(commands: &mut Commands, entity: Entity) {
    commands.entity(entity).insert(FootstepEmitter::default());
}