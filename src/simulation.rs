#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North, // Moves South (downwards)
    East,  // Moves West (leftwards)
    South, // Moves North (upwards)
    West,  // Moves East (rightwards)
}

impl Direction {
    pub fn next(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn to_index(self) -> usize {
        match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Straight,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vehicle {
    pub id: usize,
    pub spawn_direction: Direction,
    pub route: Route,
    pub path: Vec<(f32, f32)>,
    pub current_waypoint: usize,
    pub x: f32,
    pub y: f32,
    pub is_stopped: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightState {
    Green,
    Yellow,
    AllRed,
}

pub struct TrafficLightController {
    pub active_direction: Direction,
    pub state: LightState,
    pub timer: f32,
    pub extended: bool,
    pub next_direction: Direction,
}

impl TrafficLightController {
    pub fn new() -> Self {
        Self {
            active_direction: Direction::North,
            state: LightState::Green,
            timer: 5.0,
            extended: false,
            next_direction: Direction::North,
        }
    }

    pub fn select_next_direction(&self, queue_lengths: &[usize; 4]) -> Direction {
        let capacity = 7;
        let mut max_q = 0;
        let mut congested_dir = None;

        // Prioritize the most congested red lane
        for i in 0..4 {
            let dir = match i {
                0 => Direction::North,
                1 => Direction::East,
                2 => Direction::South,
                3 => Direction::West,
                _ => unreachable!(),
            };
            if dir != self.active_direction && queue_lengths[i] >= capacity {
                if queue_lengths[i] > max_q {
                    max_q = queue_lengths[i];
                    congested_dir = Some(dir);
                }
            }
        }

        if let Some(dir) = congested_dir {
            dir
        } else {
            // Otherwise, cycle to the next non-empty lane if possible.
            let mut temp_dir = self.active_direction.next();
            for _ in 0..3 {
                if queue_lengths[temp_dir.to_index()] > 0 {
                    return temp_dir;
                }
                temp_dir = temp_dir.next();
            }
            // If all other lanes are empty, cycle next
            self.active_direction.next()
        }
    }

    pub fn update(&mut self, dt: f32, queue_lengths: &[usize; 4]) {
        self.timer -= dt;

        match self.state {
            LightState::Green => {
                let active_idx = self.active_direction.to_index();
                let active_q = queue_lengths[active_idx];
                let capacity = 7;

                // 1. Dynamic Congestion Rule:
                // If the current green lane is congested, extend its green time.
                if active_q >= capacity && !self.extended && self.timer < 2.0 {
                    self.timer += 3.0; // Extend by 3 seconds
                    self.extended = true;
                    println!("TrafficLightController: Extended green for {:?}", self.active_direction);
                }

                // Check if current lane is empty and others are waiting, or if the green timer expired.
                let mut other_has_vehicles = false;
                for i in 0..4 {
                    if i != active_idx && queue_lengths[i] > 0 {
                        other_has_vehicles = true;
                        break;
                    }
                }

                if (active_q == 0 && other_has_vehicles && self.timer < 3.0) || self.timer <= 0.0 {
                    self.next_direction = self.select_next_direction(queue_lengths);
                    self.state = LightState::Yellow;
                    self.timer = 1.5; // 1.5 seconds yellow
                    println!(
                        "TrafficLightController: Green ending for {:?}, switching to Yellow. Next: {:?}",
                        self.active_direction, self.next_direction
                    );
                }
            }
            LightState::Yellow => {
                if self.timer <= 0.0 {
                    self.state = LightState::AllRed;
                    self.timer = 1.0; // 1.0 second all-red clearing phase
                    println!("TrafficLightController: All Red phase started.");
                }
            }
            LightState::AllRed => {
                if self.timer <= 0.0 {
                    self.active_direction = self.next_direction;
                    self.state = LightState::Green;
                    self.timer = 5.0; // Reset to base green time
                    self.extended = false;
                    println!("TrafficLightController: Switched green to {:?}", self.active_direction);
                }
            }
        }
    }
}

pub fn generate_path(spawn: Direction, route: Route) -> Vec<(f32, f32)> {
    let mut path = Vec::new();
    
    let (spawn_pos, stop_pos) = match spawn {
        Direction::North => ((370.0, -50.0), (370.0, 340.0)),
        Direction::South => ((430.0, 850.0), (430.0, 460.0)),
        Direction::West => ((-50.0, 430.0), (340.0, 430.0)),
        Direction::East => ((850.0, 370.0), (460.0, 370.0)),
    };
    
    path.push(spawn_pos);
    path.push(stop_pos);
    
    match route {
        Route::Straight => {
            let end_pos = match spawn {
                Direction::North => (370.0, 850.0),
                Direction::South => (430.0, -50.0),
                Direction::West => (850.0, 430.0),
                Direction::East => (-50.0, 370.0),
            };
            path.push(end_pos);
        }
        Route::Left => {
            let (control, end, exit) = match spawn {
                Direction::North => ((370.0, 430.0), (460.0, 430.0), (850.0, 430.0)),
                Direction::South => ((430.0, 370.0), (340.0, 370.0), (-50.0, 370.0)),
                Direction::West => ((430.0, 430.0), (430.0, 340.0), (430.0, -50.0)),
                Direction::East => ((370.0, 370.0), (370.0, 460.0), (370.0, 850.0)),
            };
            let steps = 8;
            for i in 1..=steps {
                let t = i as f32 / steps as f32;
                let x = (1.0 - t).powi(2) * stop_pos.0 + 2.0 * (1.0 - t) * t * control.0 + t.powi(2) * end.0;
                let y = (1.0 - t).powi(2) * stop_pos.1 + 2.0 * (1.0 - t) * t * control.1 + t.powi(2) * end.1;
                path.push((x, y));
            }
            path.push(exit);
        }
        Route::Right => {
            let (control, end, exit) = match spawn {
                Direction::North => ((370.0, 370.0), (340.0, 370.0), (-50.0, 370.0)),
                Direction::South => ((430.0, 430.0), (460.0, 430.0), (850.0, 430.0)),
                Direction::West => ((370.0, 430.0), (370.0, 460.0), (370.0, 850.0)),
                Direction::East => ((430.0, 370.0), (430.0, 340.0), (430.0, -50.0)),
            };
            let steps = 8;
            for i in 1..=steps {
                let t = i as f32 / steps as f32;
                let x = (1.0 - t).powi(2) * stop_pos.0 + 2.0 * (1.0 - t) * t * control.0 + t.powi(2) * end.0;
                let y = (1.0 - t).powi(2) * stop_pos.1 + 2.0 * (1.0 - t) * t * control.1 + t.powi(2) * end.1;
                path.push((x, y));
            }
            path.push(exit);
        }
    }
    path
}

pub fn can_spawn(direction: Direction, vehicles: &[Vehicle]) -> bool {
    let spawn_pos = match direction {
        Direction::North => (370.0, -50.0),
        Direction::South => (430.0, 850.0),
        Direction::West => (-50.0, 430.0),
        Direction::East => (850.0, 370.0),
    };
    
    let threshold = 45.0; // vehicle_length (30) + safety_gap (15)
    for v in vehicles {
        if v.spawn_direction == direction {
            let dx = v.x - spawn_pos.0;
            let dy = v.y - spawn_pos.1;
            let dist = (dx*dx + dy*dy).sqrt();
            if dist < threshold {
                return false;
            }
        }
    }
    true
}

fn has_priority(a: &Vehicle, b: &Vehicle) -> bool {
    let a_entered = a.current_waypoint > 1;
    let b_entered = b.current_waypoint > 1;
    if a_entered != b_entered {
        return a_entered;
    }

    if a_entered {
        let a_left = a.path.len() - a.current_waypoint;
        let b_left = b.path.len() - b.current_waypoint;
        if a_left != b_left {
            return a_left < b_left;
        }
        return a.id < b.id;
    } else {
        let a_stop = a.path[1];
        let b_stop = b.path[1];
        let a_dist = ((a.x - a_stop.0).powi(2) + (a.y - a_stop.1).powi(2)).sqrt();
        let b_dist = ((b.x - b_stop.0).powi(2) + (b.y - b_stop.1).powi(2)).sqrt();
        if (a_dist - b_dist).abs() > 0.1 {
            return a_dist < b_dist;
        }
        return a.id < b.id;
    }
}

fn predict_position(
    car: &Vehicle,
    steps: usize,
    controller: &TrafficLightController,
    assume_moving: bool,
) -> (f32, f32) {
    if !assume_moving && car.is_stopped {
        return (car.x, car.y);
    }
    
    let mut x = car.x;
    let mut y = car.y;
    let mut wp = car.current_waypoint;
    let speed = 2.0;
    
    for _ in 0..steps {
        if wp >= car.path.len() {
            break;
        }
        
        // Traffic light check
        if wp == 1 {
            let is_active = controller.active_direction == car.spawn_direction;
            let light_is_red = !is_active || controller.state == LightState::AllRed;
            let light_is_yellow = is_active && controller.state == LightState::Yellow;
            
            let target = car.path[1];
            let dx = target.0 - x;
            let dy = target.1 - y;
            let dist = (dx*dx + dy*dy).sqrt();
            
            if light_is_red {
                if dist <= 25.0 {
                    break;
                }
            } else if light_is_yellow {
                if dist > 15.0 && dist <= 45.0 {
                    break;
                }
            }
        }
        
        let target = car.path[wp];
        let dx = target.0 - x;
        let dy = target.1 - y;
        let dist = (dx*dx + dy*dy).sqrt();
        if dist <= speed {
            x = target.0;
            y = target.1;
            wp += 1;
        } else {
            x += speed * dx / dist;
            y += speed * dy / dist;
        }
    }
    (x, y)
}

pub fn update_vehicles(
    vehicles: &mut Vec<Vehicle>,
    controller: &TrafficLightController,
    completed_count: &mut usize,
) {
    let num_vehicles = vehicles.len();
    let mut should_stop_vec = vec![false; num_vehicles];

    for i in 0..num_vehicles {
        let car = &vehicles[i];
        if car.current_waypoint >= car.path.len() {
            continue;
        }
        
        let target = car.path[car.current_waypoint];
        let dx = target.0 - car.x;
        let dy = target.1 - car.y;
        let dist = (dx*dx + dy*dy).sqrt();
        let dist_safe = if dist < 0.001 { 0.001 } else { dist };
        
        let mut stop = false;
        
        // 1. Traffic Light check
        if car.current_waypoint == 1 {
            let is_active = controller.active_direction == car.spawn_direction;
            let light_is_red = !is_active || controller.state == LightState::AllRed;
            let light_is_yellow = is_active && controller.state == LightState::Yellow;
            
            if light_is_red {
                if dist_safe <= 25.0 {
                    stop = true;
                }
            } else if light_is_yellow {
                if dist_safe > 15.0 && dist_safe <= 45.0 {
                    stop = true;
                }
            }
        }
        
        // 2. Predictive collision check
        if !stop {
            for j in 0..num_vehicles {
                if i == j { continue; }
                let other = &vehicles[j];
                if other.current_waypoint >= other.path.len() { continue; }
                
                if has_priority(other, car) {
                    let mut collision = false;
                    for t in 0..40 {
                        let pos_car = predict_position(car, t, controller, true);
                        let pos_other = predict_position(other, t, controller, false);
                        
                        let dx = pos_car.0 - pos_other.0;
                        let dy = pos_car.1 - pos_other.1;
                        let dist = (dx*dx + dy*dy).sqrt();
                        
                        let limit = if car.spawn_direction == other.spawn_direction {
                            42.0 // Queue safety margin
                        } else {
                            35.0 // Crossing safety margin
                        };
                        
                        if dist < limit {
                            collision = true;
                            break;
                        }
                    }
                    if collision {
                        stop = true;
                        break;
                    }
                }
            }
        }
        
        should_stop_vec[i] = stop;
    }
    
    // Move vehicles and clean up finished ones
    let speed = 2.0; // Fixed speed per frame
    let mut i = 0;
    while i < vehicles.len() {
        vehicles[i].is_stopped = should_stop_vec[i];
        
        if !vehicles[i].is_stopped {
            let target = vehicles[i].path[vehicles[i].current_waypoint];
            let dx = target.0 - vehicles[i].x;
            let dy = target.1 - vehicles[i].y;
            let dist = (dx*dx + dy*dy).sqrt();
            
            if dist <= speed {
                vehicles[i].x = target.0;
                vehicles[i].y = target.1;
                vehicles[i].current_waypoint += 1;
            } else {
                vehicles[i].x += speed * dx / dist;
                vehicles[i].y += speed * dy / dist;
            }
        }
        
        if vehicles[i].current_waypoint >= vehicles[i].path.len() {
            vehicles.remove(i);
            *completed_count += 1;
        } else {
            i += 1;
        }
    }
}
