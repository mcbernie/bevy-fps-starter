# Bevy FPS Starter

A comprehensive FPS starter project for Bevy 0.16 in the style of Half-Life 1/Counter-Strike, featuring modular architecture, physics integration, and networking capability.

## Features

- ✅ **Modular Architecture**: Well-organized plugin system for easy extensibility
- ✅ **FPS Controller**: Physics-based first-person controller with proper jumping mechanics
- ✅ **Physics Integration**: Avian3D physics engine for realistic movement and interactions
- ✅ **Weapon System**: Complete weapon handling with pickup, firing, and different weapon types
- ✅ **Interaction System**: E key interactions with health packs, ammo packs, and weapon pickups
- ✅ **Enhanced HUD**: Real-time health, ammo, and weapon displays with professional layout
- ✅ **Advanced Lighting**: Dynamic lighting with flickering effects and shadows
- 🚧 **BSP Map Loading**: Foundation for loading Quake/Half-Life style BSP maps
- 🚧 **Multiplayer**: Networking foundation (Lightyear integration planned)
- 🚧 **Audio System**: 3D positional audio support

## Quick Start

### Prerequisites

- Rust 1.70+ 
- Git

### Installation

1. Clone the repository:
```bash
git clone https://github.com/mcbernie/bevy-fps-starter.git
cd bevy-fps-starter
```

2. Build and run:
```bash
cargo run
```

## Controls

| Key/Input | Action |
|-----------|--------|
| **WASD** | Move around |
| **Mouse** | Look around |
| **Space** | Jump (physics-based, only when grounded) |
| **E** | Interact with items/pickups |
| **Left Mouse** | Fire equipped weapon |
| **Tab** | Toggle cursor lock/unlock |
| **Esc** | Exit game |
| **F3** | Toggle physics debug |
| **F4** | Toggle shadows |
| **F5** | Toggle ambient lighting |
| **F7** | Reload map |
| **F8** | Start server (placeholder) |
| **F9** | Connect as client (placeholder) |
| **F10** | Disconnect (placeholder) |
| **F11** | Toggle debug info |

## Project Structure

```
src/
├── main.rs              # Main application entry point
├── fps_controller.rs    # First-person controller system
├── physics.rs           # Physics integration (Avian3D)
├── lighting.rs          # Dynamic lighting system
├── maps.rs              # Map loading and BSP support
├── multiplayer.rs       # Networking and multiplayer
└── ui.rs                # User interface system
```

## Architecture

The project follows Bevy's plugin architecture for maximum modularity:

### Core Plugins

- **FpsControllerPlugin**: Handles first-person camera movement and input
- **PhysicsPlugin**: Integrates Avian3D physics engine
- **LightingPlugin**: Manages dynamic lighting and shadows
- **MapLoadingPlugin**: Foundation for BSP map loading
- **MultiplayerPlugin**: Networking infrastructure
- **GameUIPlugin**: User interface and HUD

### Key Components

- `FpsController`: First-person camera controller
- `PhysicsObject`: Physics-enabled entities
- `DynamicLight`: Lights with flickering effects
- `MapEntity`: Map geometry and entities
- `NetworkPlayer`: Multiplayer player representation

## Physics System

The physics system is built on Avian3D, providing:

- **Rigid Body Dynamics**: Static and dynamic objects
- **Collision Detection**: Precise collision handling
- **Physics Materials**: Different surface properties
- **Debug Rendering**: Visual physics debugging (F3)

### Adding Physics to Objects

```rust
// Static collider (walls, floors)
commands.spawn((
    Transform::from_xyz(0.0, 0.0, 0.0),
    RigidBody::Static,
    Collider::cuboid(1.0, 1.0, 1.0),
));

// Dynamic object (moveable)
commands.spawn((
    Transform::from_xyz(0.0, 5.0, 0.0),
    RigidBody::Dynamic,
    Collider::cuboid(1.0, 1.0, 1.0),
    Mass(1.0),
));
```

## Lighting System

Advanced lighting features inspired by classic FPS games:

- **Directional Lighting**: Sun/moon lighting
- **Point Lights**: Local light sources
- **Spot Lights**: Focused beams
- **Dynamic Effects**: Flickering lights for atmosphere
- **Shadow Mapping**: Real-time shadows
- **Ambient Lighting**: Global illumination

### Creating Dynamic Lights

```rust
use crate::lighting::create_flickering_light;

// Create a flickering light
create_flickering_light(
    &mut commands,
    Vec3::new(0.0, 3.0, 0.0),      // position
    Color::srgb(1.0, 0.9, 0.7),    // warm color
    500.0,                          // intensity
    20.0,                          // range
    2.0,                           // flicker speed
);
```

## Map System

Foundation for loading BSP maps (Quake/Half-Life format):

- **BSP Parser**: Parse BSP file format (planned)
- **Geometry Loading**: Convert BSP brushes to Bevy meshes
- **Entity System**: Spawn map entities (lights, spawn points)
- **Texture Loading**: BSP texture support
- **Collision Generation**: Automatic collision mesh generation

### Map Entities

The system recognizes standard map entities:

- `info_player_start`: Player spawn points
- `light`: Light entities with properties
- `func_wall`: Brush entities
- Custom entities for gameplay

## Multiplayer Foundation

Networking infrastructure ready for Lightyear integration:

- **Client-Server Architecture**: Dedicated server support
- **State Synchronization**: Player positions and game state
- **Input Prediction**: Client-side prediction
- **Entity Interpolation**: Smooth movement
- **Chat System**: In-game communication (planned)

## Development

### Building for Development

```bash
# Debug build with fast compilation
cargo run

# Release build for performance testing
cargo run --release
```

### Adding New Features

1. **Create a new plugin** in a separate module
2. **Add the plugin** to `main.rs`
3. **Use Bevy's ECS** for data management
4. **Follow the existing patterns** for consistency

### Plugin Template

```rust
use bevy::prelude::*;

pub struct MyFeaturePlugin;

impl Plugin for MyFeaturePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_my_feature)
            .add_systems(Update, update_my_feature);
    }
}

fn setup_my_feature(mut commands: Commands) {
    // Initialize your feature
}

fn update_my_feature() {
    // Update logic
}
```

## Performance

### Optimization Tips

- Use `cargo run --release` for performance testing
- Enable Bevy's `dynamic_linking` feature for faster compile times during development
- Profile with `cargo flamegraph` for bottleneck identification
- Consider LOD systems for complex maps

### Frame Rate Targets

- **Development**: 60+ FPS
- **Release**: 120+ FPS (competitive)
- **Minimum**: 30+ FPS (playable)

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

### Code Style

- Follow Rust naming conventions
- Use `rustfmt` for code formatting
- Add documentation for public APIs
- Write tests for new features

## Roadmap

### Phase 1: Core Systems ✅
- [x] Basic FPS controller
- [x] Physics integration
- [x] Lighting system
- [x] UI foundation
- [x] Project structure

### Phase 2: Map System 🚧
- [ ] BSP file parser
- [ ] Texture loading
- [ ] Collision generation
- [ ] Entity spawning
- [ ] Example maps

### Phase 3: Networking 🚧
- [ ] Lightyear integration
- [ ] Client-server architecture
- [ ] State synchronization
- [ ] Input prediction
- [ ] Chat system

### Phase 4: Audio 📋
- [ ] 3D positional audio
- [ ] Sound effects
- [ ] Music system
- [ ] Audio occlusion

### Phase 5: Gameplay 📋
- [ ] Weapon system
- [ ] Health/damage
- [ ] Inventory
- [ ] Game modes

## License

This project is licensed under the MIT OR Apache-2.0 license.

## Acknowledgments

- **Bevy Engine**: Amazing Rust game engine
- **Avian3D**: Excellent physics integration
- **Half-Life/Counter-Strike**: Inspiration for gameplay feel
- **Quake Engine**: BSP format reference
- **Rust Community**: Incredible ecosystem and support

## Support

- [Bevy Discord](https://discord.gg/bevy)
- [Issues](https://github.com/mcbernie/bevy-fps-starter/issues)
- [Discussions](https://github.com/mcbernie/bevy-fps-starter/discussions)

---

**Ready to create the next great FPS game? Start building!** 🎮
