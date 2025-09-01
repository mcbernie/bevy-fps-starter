//! Gameplay systems module
//! 
//! Contains game-specific logic and mechanics:
//! - Weapon system and combat
//! - Player interactions and health
//! - Audio and sound effects

pub mod weapons;
pub mod interaction;
pub mod audio;

pub use weapons::WeaponPlugin;
pub use interaction::InteractionPlugin;
pub use audio::AudioPlugin;