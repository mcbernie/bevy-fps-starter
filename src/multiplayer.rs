use bevy::prelude::*;

pub struct MultiplayerPlugin;

impl Plugin for MultiplayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_multiplayer)
            .add_systems(Update, handle_multiplayer_input);
    }
}

#[derive(Resource)]
pub struct NetworkSettings {
    pub is_server: bool,
    pub is_client: bool,
    pub server_address: String,
    pub port: u16,
    pub max_players: usize,
}

impl Default for NetworkSettings {
    fn default() -> Self {
        Self {
            is_server: false,
            is_client: false,
            server_address: "127.0.0.1".to_string(),
            port: 5000,
            max_players: 16,
        }
    }
}

#[derive(Component)]
pub struct NetworkPlayer {
    pub player_id: u32,
    pub is_local: bool,
}

#[derive(Component)]
pub struct NetworkTransform {
    pub last_sent: Transform,
    pub interpolation_target: Transform,
}

fn setup_multiplayer(mut commands: Commands) {
    commands.insert_resource(NetworkSettings::default());
    
    info!("Multiplayer system initialized");
    info!("Press F8 to start server, F9 to connect as client");
}

fn handle_multiplayer_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut network_settings: ResMut<NetworkSettings>,
) {
    if keyboard_input.just_pressed(KeyCode::F8) {
        if !network_settings.is_server && !network_settings.is_client {
            start_server(&mut network_settings);
        }
    }

    if keyboard_input.just_pressed(KeyCode::F9) {
        if !network_settings.is_server && !network_settings.is_client {
            connect_to_server(&mut network_settings);
        }
    }

    if keyboard_input.just_pressed(KeyCode::F10) {
        if network_settings.is_server || network_settings.is_client {
            disconnect(&mut network_settings);
        }
    }
}

fn start_server(network_settings: &mut NetworkSettings) {
    info!("Starting server on port {}", network_settings.port);
    network_settings.is_server = true;
    
    // TODO: Initialize Lightyear server
    // This would involve:
    // 1. Setting up the server socket
    // 2. Configuring replication rules
    // 3. Setting up player management
    // 4. Implementing game state synchronization
}

fn connect_to_server(network_settings: &mut NetworkSettings) {
    info!("Connecting to server at {}:{}", network_settings.server_address, network_settings.port);
    network_settings.is_client = true;
    
    // TODO: Initialize Lightyear client
    // This would involve:
    // 1. Connecting to the server
    // 2. Setting up client-side prediction
    // 3. Implementing input handling
    // 4. Setting up entity interpolation
}

fn disconnect(network_settings: &mut NetworkSettings) {
    info!("Disconnecting from network");
    network_settings.is_server = false;
    network_settings.is_client = false;
    
    // TODO: Clean up network connections
}

// Placeholder for network message types
#[derive(Debug)]
pub enum NetworkMessage {
    PlayerJoined { player_id: u32 },
    PlayerLeft { player_id: u32 },
    PlayerMove { player_id: u32, transform: Transform },
    Chat { player_id: u32, message: String },
}

// Utility functions for multiplayer
pub fn spawn_network_player(
    commands: &mut Commands,
    player_id: u32,
    is_local: bool,
    spawn_position: Vec3,
) -> Entity {
    commands.spawn((
        Transform::from_translation(spawn_position),
        NetworkPlayer { player_id, is_local },
        NetworkTransform {
            last_sent: Transform::from_translation(spawn_position),
            interpolation_target: Transform::from_translation(spawn_position),
        },
    )).id()
}

pub fn send_player_update(
    player_id: u32,
    transform: Transform,
    network_settings: &NetworkSettings,
) {
    if network_settings.is_client || network_settings.is_server {
        // TODO: Send transform update over network
        info!("Sending player {} update: {:?}", player_id, transform.translation);
    }
}

// Systems for networked gameplay (stubs for now)
pub fn interpolate_network_players(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut NetworkTransform), With<NetworkPlayer>>,
) {
    for (mut transform, mut net_transform) in query.iter_mut() {
        // Simple linear interpolation towards the target
        let t = time.delta_secs() * 10.0; // Interpolation speed
        transform.translation = transform.translation.lerp(net_transform.interpolation_target.translation, t);
        transform.rotation = transform.rotation.slerp(net_transform.interpolation_target.rotation, t);
    }
}

pub fn send_local_player_updates(
    query: Query<(Entity, &Transform, &NetworkPlayer)>,
    network_settings: Res<NetworkSettings>,
) {
    for (_, transform, player) in query.iter() {
        if player.is_local {
            send_player_update(player.player_id, *transform, &network_settings);
        }
    }
}

// Chat system
#[derive(Resource, Default)]
pub struct ChatHistory {
    pub messages: Vec<String>,
}

pub fn handle_chat_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut chat_history: ResMut<ChatHistory>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        // TODO: Open chat input
        info!("Chat system not yet implemented");
    }
}