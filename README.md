# Road Intersection Simulation

A traffic control simulation for a four-way intersection, implementing traffic lights and vehicle movement according to traffic regulations.

## Features

- Four-way intersection with single-lane roads in each direction
- Traffic light system to prevent collisions
- Color-coded vehicles based on their intended route
- Adaptive traffic management algorithm
- Safe distance maintenance between vehicles

## Controls

- **↑ Up Arrow**: Spawn a vehicle from the south
- **↓ Down Arrow**: Spawn a vehicle from the north
- **→ Right Arrow**: Spawn a vehicle from the west
- **← Left Arrow**: Spawn a vehicle from the east
- **R Key**: Spawn a vehicle from a random direction
- **Escape**: Exit the simulation

## Vehicle Color Codes

- **Red**: Vehicle will turn left
- **Green**: Vehicle will turn right
- **Blue**: Vehicle will go straight

## Building and Running

```bash
# Build the project
cargo build --release

# Run the simulation
cargo run --release
```

## Requirements

- Rust 2021 or newer
- SDL2 development libraries installed on your system

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

