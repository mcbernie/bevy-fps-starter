//! User interface systems module
//! 
//! Contains all UI-related systems:
//! - Game UI and HUD
//! - Networking interface

pub mod ui;
pub mod networking;

pub use ui::GameUIPlugin;
pub use networking::multiplayer::MultiplayerPlugin;