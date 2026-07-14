mod rendering;
mod simulation;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use rand::Rng;

use simulation::{Direction, Route, Vehicle, TrafficLightController};
use simulation::{generate_path, can_spawn, update_vehicles};
use rendering::{create_car_texture, draw_roads, draw_traffic_lights, draw_sidebar, draw_vehicles};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let timer_subsystem = sdl_context.timer()?;

    let window = video_subsystem
        .window("Traffic Control Simulation", 1000, 800)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().present_vsync().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    // Generate car textures based on route colors
    let tex_straight = create_car_texture(&texture_creator, Color::RGB(0, 191, 255))?; // Cyan
    let tex_left = create_car_texture(&texture_creator, Color::RGB(255, 127, 80))?;    // Orange
    let tex_right = create_car_texture(&texture_creator, Color::RGB(255, 215, 0))?;   // Yellow

    let mut controller = TrafficLightController::new();
    let mut vehicles: Vec<Vehicle> = Vec::new();
    
    let mut completed_crossings = 0;
    let mut next_vehicle_id = 0;

    let mut event_pump = sdl_context.event_pump()?;
    let mut rng = rand::thread_rng();

    let dt = 1.0 / 60.0; // Fixed delta time for 60 FPS updates

    'running: loop {
        let frame_start = timer_subsystem.ticks();

        // 1. Event Handling
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                
                Event::KeyDown { keycode: Some(k), .. } => {
                    let mut dir_to_spawn = None;
                    match k {
                        Keycode::Up => dir_to_spawn = Some(Direction::South),
                        Keycode::Down => dir_to_spawn = Some(Direction::North),
                        Keycode::Right => dir_to_spawn = Some(Direction::West),
                        Keycode::Left => dir_to_spawn = Some(Direction::East),
                        Keycode::R => {
                            let r_idx = rng.gen_range(0..4);
                            let dir = match r_idx {
                                0 => Direction::North,
                                1 => Direction::East,
                                2 => Direction::South,
                                3 => Direction::West,
                                _ => unreachable!(),
                            };
                            dir_to_spawn = Some(dir);
                        }
                        _ => {}
                    }
                    
                    if let Some(dir) = dir_to_spawn {
                        if can_spawn(dir, &vehicles) {
                            let route_val = rng.gen_range(0..3);
                            let route = match route_val {
                                0 => Route::Straight,
                                1 => Route::Left,
                                2 => Route::Right,
                                _ => unreachable!(),
                            };
                            
                            let path = generate_path(dir, route);
                            let spawn_pos = path[0];
                            
                            vehicles.push(Vehicle {
                                id: next_vehicle_id,
                                spawn_direction: dir,
                                route,
                                path,
                                current_waypoint: 1,
                                x: spawn_pos.0,
                                y: spawn_pos.1,
                                is_stopped: false,
                            });
                            next_vehicle_id += 1;
                        }
                    }
                }
                _ => {}
            }
        }

        // 2. Logic Updates
        let q_north = vehicles.iter().filter(|v| v.spawn_direction == Direction::North && v.current_waypoint <= 1).count();
        let q_south = vehicles.iter().filter(|v| v.spawn_direction == Direction::South && v.current_waypoint <= 1).count();
        let q_east = vehicles.iter().filter(|v| v.spawn_direction == Direction::East && v.current_waypoint <= 1).count();
        let q_west = vehicles.iter().filter(|v| v.spawn_direction == Direction::West && v.current_waypoint <= 1).count();
        
        let queue_lengths = [q_north, q_east, q_south, q_west];
        
        controller.update(dt, &queue_lengths);
        update_vehicles(&mut vehicles, &controller, &mut completed_crossings);

        // 3. Rendering
        draw_roads(&mut canvas)?;
        draw_traffic_lights(&mut canvas, &controller)?;
        draw_vehicles(&mut canvas, &vehicles, &tex_straight, &tex_left, &tex_right)?;

        draw_sidebar(
            &mut canvas,
            vehicles.len(),
            completed_crossings,
            q_north,
            q_south,
            q_east,
            q_west,
            7, // Capacity
            &controller,
        )?;

        canvas.present();

        let frame_time = timer_subsystem.ticks() - frame_start;
        if frame_time < 16 {
            std::thread::sleep(Duration::from_millis((16 - frame_time) as u64));
        }
    }

    Ok(())
}
