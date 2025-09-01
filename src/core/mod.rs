//! Core systems module
//! 
//! Contains fundamental systems that the game engine relies on:
//! - Physics engine integration
//! - Player controller and input handling

pub mod physics;
pub mod fps_controller;

pub use physics::PhysicsPlugin;
pub use fps_controller::FpsControllerPlugin;