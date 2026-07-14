# Traffic Control Simulation

A high-fidelity 2D traffic simulation of a 4-way intersection built in **Rust** using the **SDL2** library. 

This project implements a dynamic traffic control strategy that optimizes light phases to prevent queue overflow, backed by an active vehicle collision-avoidance mechanism.

## Features

- **Crossing Roads & Turns**: A standard 4-way intersection (North, South, East, West) with single lanes. Vehicles can dynamically go straight, turn left, or turn right.
- **Dynamic Traffic Lights**: Uses a 4-phase color code system (Red and Green only) that optimizes green phases based on lane queues.
- **Dynamic Congestion Rule**: Calculates the physical queue capacity for each lane dynamically:
  $$\text{capacity} = \lfloor \frac{\text{lane\_length}}{\text{vehicle\_length} + \text{safety\_gap}} \rfloor$$
  If a lane's queue reaches capacity, the controller adjusts the light logic to extend green times or shorten current green lights.
- **Vehicle Routing & Aesthetics**: Vehicles are drawn with rich details (wheels, windshield, headlights, taillights) and dynamically rotate along curves. They are colored based on their route:
  - **Cyan**: Continuing Straight
  - **Orange**: Left Turn
  - **Yellow**: Right Turn
- **Anti-Spam Spawning**: Spamming vehicle spawns is blocked. Spawning is allowed only when there is a safe distance between vehicles.
- **Sleek Sidebar Dashboard**: Displays live stats, queue congestion levels (with colored safety bars), traffic light timers, active phases, and controls.

## Control Guide

Use your keyboard to spawn vehicles from the corresponding direction:

- **`↑` Up Arrow**: Spawn a vehicle coming from the **South** (moving North).
- **`↓` Down Arrow**: Spawn a vehicle coming from the **North** (moving South).
- **`→` Right Arrow**: Spawn a vehicle coming from the **West** (moving East).
- **`←` Left Arrow**: Spawn a vehicle coming from the **East** (moving West).
- **`R` Key**: Spawn a vehicle from a **random** direction.
- **`Esc` Key**: Terminate the simulation.

## How to Build & Run

### Prerequisites

You need the SDL2 development libraries installed on your machine. On Ubuntu/Debian:

```bash
sudo apt-get install libsdl2-dev
```

### Run Command

Simply run:

```bash
cargo run --release
```
