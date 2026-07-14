use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};

use crate::simulation::{Direction, LightState, TrafficLightController, Vehicle, Route};

// --- Bitmap Font Generator & Renderer ---

fn get_char_bitmap(c: char) -> [u8; 7] {
    match c.to_ascii_uppercase() {
        '0' => [0x0E, 0x11, 0x13, 0x15, 0x19, 0x11, 0x0E],
        '1' => [0x04, 0x0C, 0x04, 0x04, 0x04, 0x04, 0x0E],
        '2' => [0x0E, 0x11, 0x01, 0x02, 0x04, 0x08, 0x1F],
        '3' => [0x1F, 0x02, 0x04, 0x02, 0x01, 0x11, 0x0E],
        '4' => [0x02, 0x06, 0x0A, 0x12, 0x1F, 0x02, 0x02],
        '5' => [0x1F, 0x10, 0x1E, 0x01, 0x01, 0x11, 0x0E],
        '6' => [0x0E, 0x11, 0x10, 0x1E, 0x11, 0x11, 0x0E],
        '7' => [0x1F, 0x01, 0x02, 0x04, 0x08, 0x08, 0x08],
        '8' => [0x0E, 0x11, 0x11, 0x0E, 0x11, 0x11, 0x0E],
        '9' => [0x0E, 0x11, 0x11, 0x0F, 0x01, 0x11, 0x0E],
        
        'A' => [0x0E, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11],
        'B' => [0x1E, 0x11, 0x11, 0x1E, 0x11, 0x11, 0x1E],
        'C' => [0x0E, 0x11, 0x10, 0x10, 0x10, 0x11, 0x0E],
        'D' => [0x1C, 0x12, 0x11, 0x11, 0x11, 0x12, 0x1C],
        'E' => [0x1F, 0x10, 0x10, 0x1E, 0x10, 0x10, 0x1F],
        'F' => [0x1F, 0x10, 0x10, 0x1E, 0x10, 0x10, 0x10],
        'G' => [0x0E, 0x11, 0x10, 0x17, 0x11, 0x11, 0x0F],
        'H' => [0x11, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11],
        'I' => [0x0E, 0x04, 0x04, 0x04, 0x04, 0x04, 0x0E],
        'J' => [0x07, 0x02, 0x02, 0x02, 0x02, 0x12, 0x0C],
        'K' => [0x11, 0x12, 0x14, 0x18, 0x14, 0x12, 0x11],
        'L' => [0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x1F],
        'M' => [0x11, 0x1B, 0x15, 0x11, 0x11, 0x11, 0x11],
        'N' => [0x11, 0x19, 0x15, 0x13, 0x11, 0x11, 0x11],
        'O' => [0x0E, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0E],
        'P' => [0x1E, 0x11, 0x11, 0x1E, 0x10, 0x10, 0x10],
        'Q' => [0x0E, 0x11, 0x11, 0x11, 0x15, 0x12, 0x0D],
        'R' => [0x1E, 0x11, 0x11, 0x1E, 0x14, 0x12, 0x11],
        'S' => [0x0F, 0x10, 0x10, 0x0E, 0x01, 0x01, 0x1E],
        'T' => [0x1F, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04],
        'U' => [0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0E],
        'V' => [0x11, 0x11, 0x11, 0x11, 0x11, 0x0A, 0x04],
        'W' => [0x11, 0x11, 0x11, 0x15, 0x15, 0x1B, 0x11],
        'X' => [0x11, 0x11, 0x0A, 0x04, 0x0A, 0x11, 0x11],
        'Y' => [0x11, 0x11, 0x0A, 0x04, 0x04, 0x04, 0x04],
        'Z' => [0x1F, 0x01, 0x02, 0x04, 0x08, 0x10, 0x1F],
        
        ':' => [0x00, 0x0C, 0x0C, 0x00, 0x0C, 0x0C, 0x00],
        '/' => [0x01, 0x02, 0x02, 0x04, 0x08, 0x08, 0x10],
        '-' => [0x00, 0x00, 0x00, 0x1F, 0x00, 0x00, 0x00],
        '.' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x0C],
        '%' => [0x19, 0x19, 0x02, 0x04, 0x08, 0x13, 0x13],
        _ =>   [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    }
}

pub fn draw_string<T: sdl2::render::RenderTarget>(
    canvas: &mut Canvas<T>,
    x: i32,
    y: i32,
    s: &str,
    scale: i32,
    color: Color,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    let mut current_x = x;
    
    for c in s.chars() {
        if c == ' ' {
            current_x += 6 * scale;
            continue;
        }
        let bitmap = get_char_bitmap(c);
        for row in 0..7 {
            let row_val = bitmap[row];
            for col in 0..5 {
                let mask = 1 << (4 - col);
                if (row_val & mask) != 0 {
                    let rect = Rect::new(
                        current_x + (col as i32) * scale,
                        y + (row as i32) * scale,
                        scale as u32,
                        scale as u32,
                    );
                    canvas.fill_rect(rect)?;
                }
            }
        }
        current_x += 6 * scale;
    }
    Ok(())
}

// --- Helper Drawing Functions ---

pub fn fill_circle<T: sdl2::render::RenderTarget>(
    canvas: &mut Canvas<T>,
    cx: i32,
    cy: i32,
    radius: i32,
    color: Color,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    let r2 = radius * radius;
    for y in -radius..=radius {
        let y2 = y * y;
        let w = ((r2 - y2) as f64).sqrt() as i32;
        let rect = Rect::new(cx - w, cy + y, (w * 2) as u32, 1);
        canvas.fill_rect(rect)?;
    }
    Ok(())
}

pub fn create_car_texture<'a, T>(
    texture_creator: &'a TextureCreator<T>,
    color: Color,
) -> Result<Texture<'a>, String> {
    let mut surface = sdl2::surface::Surface::new(
        30,
        18,
        sdl2::pixels::PixelFormatEnum::RGBA8888,
    )?;
    
    surface.fill_rect(None, Color::RGBA(0, 0, 0, 0))?;
    
    // Draw car body
    surface.fill_rect(Some(Rect::new(2, 2, 26, 14)), color)?;
    
    // Draw bumper blocks
    surface.fill_rect(Some(Rect::new(0, 3, 2, 12)), Color::RGB(30, 30, 30))?;
    surface.fill_rect(Some(Rect::new(28, 3, 2, 12)), Color::RGB(30, 30, 30))?;
    
    // Windshield (facing East, right side)
    surface.fill_rect(Some(Rect::new(17, 3, 5, 12)), Color::RGB(100, 200, 255))?;
    
    // Side windows
    surface.fill_rect(Some(Rect::new(8, 2, 8, 2)), Color::RGB(100, 200, 255))?;
    surface.fill_rect(Some(Rect::new(8, 14, 8, 2)), Color::RGB(100, 200, 255))?;
    
    // Headlights (East side front corners)
    surface.fill_rect(Some(Rect::new(27, 2, 2, 3)), Color::RGB(255, 255, 200))?;
    surface.fill_rect(Some(Rect::new(27, 13, 2, 3)), Color::RGB(255, 255, 200))?;
    
    // Taillights (West side rear corners)
    surface.fill_rect(Some(Rect::new(1, 2, 2, 3)), Color::RGB(255, 50, 50))?;
    surface.fill_rect(Some(Rect::new(1, 13, 2, 3)), Color::RGB(255, 50, 50))?;
    
    texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())
}

pub fn draw_roads<T: sdl2::render::RenderTarget>(canvas: &mut Canvas<T>) -> Result<(), String> {
    // 1. Grass Green Background
    canvas.set_draw_color(Color::RGB(34, 52, 38));
    canvas.clear();

    // 2. Road Charcoal Background
    canvas.set_draw_color(Color::RGB(48, 48, 52));
    // Vertical road
    canvas.fill_rect(Rect::new(340, 0, 120, 800))?;
    // Horizontal road
    canvas.fill_rect(Rect::new(0, 340, 800, 120))?;

    // 3. Road Markings
    let yellow = Color::RGB(230, 185, 40);
    let white = Color::RGB(220, 220, 220);

    // Outer boundaries (solid white)
    canvas.set_draw_color(white);
    // Vertical road outer lines
    canvas.fill_rect(Rect::new(340, 0, 2, 340))?;
    canvas.fill_rect(Rect::new(460, 0, 2, 340))?;
    canvas.fill_rect(Rect::new(340, 460, 2, 340))?;
    canvas.fill_rect(Rect::new(460, 460, 2, 340))?;
    // Horizontal road outer lines
    canvas.fill_rect(Rect::new(0, 340, 340, 2))?;
    canvas.fill_rect(Rect::new(0, 460, 340, 2))?;
    canvas.fill_rect(Rect::new(460, 340, 340, 2))?;
    canvas.fill_rect(Rect::new(460, 460, 340, 2))?;

    // Center divider: double yellow lines
    canvas.set_draw_color(yellow);
    // Vertical
    canvas.fill_rect(Rect::new(398, 0, 1, 340))?;
    canvas.fill_rect(Rect::new(401, 0, 1, 340))?;
    canvas.fill_rect(Rect::new(398, 460, 1, 340))?;
    canvas.fill_rect(Rect::new(401, 460, 1, 340))?;
    // Horizontal
    canvas.fill_rect(Rect::new(0, 398, 340, 1))?;
    canvas.fill_rect(Rect::new(0, 401, 340, 1))?;
    canvas.fill_rect(Rect::new(460, 398, 340, 1))?;
    canvas.fill_rect(Rect::new(460, 401, 340, 1))?;

    // Crosswalks (Sleek stripes)
    canvas.set_draw_color(white);
    // North Crosswalk
    for i in 0..6 {
        canvas.fill_rect(Rect::new(346 + i * 20, 315, 8, 20))?;
    }
    // South Crosswalk
    for i in 0..6 {
        canvas.fill_rect(Rect::new(346 + i * 20, 465, 8, 20))?;
    }
    // West Crosswalk
    for i in 0..6 {
        canvas.fill_rect(Rect::new(315, 346 + i * 20, 20, 8))?;
    }
    // East Crosswalk
    for i in 0..6 {
        canvas.fill_rect(Rect::new(465, 346 + i * 20, 20, 8))?;
    }

    // Stop lines (thick solid white)
    canvas.fill_rect(Rect::new(340, 337, 60, 3))?; // Southbound (at North entrance)
    canvas.fill_rect(Rect::new(400, 460, 60, 3))?; // Northbound (at South entrance)
    canvas.fill_rect(Rect::new(337, 400, 3, 60))?; // Eastbound (at West entrance)
    canvas.fill_rect(Rect::new(460, 340, 3, 60))?; // Westbound (at East entrance)

    Ok(())
}

fn get_light_colors(
    dir: Direction,
    controller: &TrafficLightController,
) -> (Color, Color, Color) {
    let red_bright = Color::RGB(255, 40, 40);
    let red_dim = Color::RGB(80, 15, 15);
    let yellow_bright = Color::RGB(255, 200, 40);
    let yellow_dim = Color::RGB(80, 60, 15);
    let green_bright = Color::RGB(30, 255, 80);
    let green_dim = Color::RGB(15, 80, 15);

    let is_active = controller.active_direction == dir;
    
    if controller.state == LightState::AllRed {
        (red_bright, yellow_dim, green_dim)
    } else if is_active {
        match controller.state {
            LightState::Green => (red_dim, yellow_dim, green_bright),
            LightState::Yellow => (red_dim, yellow_bright, green_dim),
            LightState::AllRed => unreachable!(),
        }
    } else {
        (red_bright, yellow_dim, green_dim)
    }
}

pub fn draw_traffic_lights<T: sdl2::render::RenderTarget>(
    canvas: &mut Canvas<T>,
    controller: &TrafficLightController,
) -> Result<(), String> {
    let box_color = Color::RGB(20, 20, 20);

    // 1. Southbound light (North-West, facing South, driving down) - Direction::North
    let (c_red, c_yellow, c_green) = get_light_colors(Direction::North, controller);
    canvas.set_draw_color(box_color);
    canvas.fill_rect(Rect::new(318, 274, 16, 58))?;
    fill_circle(canvas, 326, 282, 5, c_red)?;
    fill_circle(canvas, 326, 299, 5, c_yellow)?;
    fill_circle(canvas, 326, 316, 5, c_green)?;

    // 2. Northbound light (South-East, facing North, driving up) - Direction::South
    let (c_red, c_yellow, c_green) = get_light_colors(Direction::South, controller);
    canvas.set_draw_color(box_color);
    canvas.fill_rect(Rect::new(466, 468, 16, 58))?;
    fill_circle(canvas, 474, 476, 5, c_red)?;
    fill_circle(canvas, 474, 493, 5, c_yellow)?;
    fill_circle(canvas, 474, 510, 5, c_green)?;

    // 3. Eastbound light (South-West, facing East, driving right) - Direction::West
    let (c_red, c_yellow, c_green) = get_light_colors(Direction::West, controller);
    canvas.set_draw_color(box_color);
    canvas.fill_rect(Rect::new(274, 466, 58, 16))?;
    fill_circle(canvas, 282, 474, 5, c_red)?;
    fill_circle(canvas, 299, 474, 5, c_yellow)?;
    fill_circle(canvas, 316, 474, 5, c_green)?;

    // 4. Westbound light (North-East, facing West, driving left) - Direction::East
    let (c_red, c_yellow, c_green) = get_light_colors(Direction::East, controller);
    canvas.set_draw_color(box_color);
    canvas.fill_rect(Rect::new(468, 318, 58, 16))?;
    fill_circle(canvas, 518, 326, 5, c_red)?;
    fill_circle(canvas, 501, 326, 5, c_yellow)?;
    fill_circle(canvas, 484, 326, 5, c_green)?;

    Ok(())
}

pub fn draw_sidebar<T: sdl2::render::RenderTarget>(
    canvas: &mut Canvas<T>,
    active_cars: usize,
    completed_cars: usize,
    q_north: usize,
    q_south: usize,
    q_east: usize,
    q_west: usize,
    capacity: usize,
    controller: &TrafficLightController,
) -> Result<(), String> {
    // Fill sidebar background
    canvas.set_draw_color(Color::RGB(18, 18, 22));
    canvas.fill_rect(Rect::new(800, 0, 200, 800))?;
    
    // Draw separator line
    canvas.set_draw_color(Color::RGB(55, 55, 62));
    canvas.fill_rect(Rect::new(800, 0, 3, 800))?;
    
    let white = Color::RGB(240, 240, 245);
    let gray = Color::RGB(140, 140, 150);
    let cyan = Color::RGB(0, 191, 255);
    let green = Color::RGB(50, 255, 100);
    let red = Color::RGB(255, 60, 60);
    let orange = Color::RGB(255, 127, 80);
    let yellow = Color::RGB(255, 215, 0);

    // Title
    draw_string(canvas, 815, 20, "TRAFFIC CONTROL", 2, cyan)?;
    draw_string(canvas, 815, 40, "DASHBOARD", 2, cyan)?;
    
    // Line separator
    canvas.set_draw_color(Color::RGB(55, 55, 62));
    canvas.fill_rect(Rect::new(815, 65, 170, 2))?;
    
    // Statistics
    draw_string(canvas, 815, 80, "VEHICLES STATS", 1, gray)?;
    draw_string(canvas, 815, 100, &format!("ACTIVE:   {}", active_cars), 1, white)?;
    draw_string(canvas, 815, 115, &format!("CROSSINGS:{}", completed_cars), 1, white)?;
    
    // Line separator
    canvas.fill_rect(Rect::new(815, 140, 170, 2))?;
    
    // Lanes Queue info
    draw_string(canvas, 815, 155, "QUEUE CONGESTION", 1, gray)?;
    
    let lanes = [
        ("NORTH (↓)", q_north, Direction::North),
        ("SOUTH (↑)", q_south, Direction::South),
        ("EAST  (←)", q_east, Direction::East),
        ("WEST  (→)", q_west, Direction::West),
    ];
    
    let mut y_pos = 175;
    for (name, q_len, dir) in lanes.iter() {
        let is_green = controller.active_direction == *dir;
        let color = if *q_len >= capacity {
            red
        } else if *q_len >= capacity - 2 {
            orange
        } else {
            green
        };
        
        draw_string(canvas, 815, y_pos, &format!("{}: {}/{}", name, q_len, capacity), 1, white)?;
        
        // Draw queue progress bar
        canvas.set_draw_color(Color::RGB(35, 35, 40));
        canvas.fill_rect(Rect::new(815, y_pos + 12, 170, 8))?;
        
        let fill_w = ((170 * q_len) / capacity).min(170) as u32;
        canvas.set_draw_color(color);
        canvas.fill_rect(Rect::new(815, y_pos + 12, fill_w, 8))?;
        
        // Light indicator next to label
        let light_color = if is_green { green } else { red };
        fill_circle(canvas, 975, y_pos + 4, 4, light_color)?;
        
        y_pos += 30;
    }
    
    // Line separator
    canvas.set_draw_color(Color::RGB(55, 55, 62));
    canvas.fill_rect(Rect::new(815, y_pos + 5, 170, 2))?;
    y_pos += 20;
    
    // Light controller status
    draw_string(canvas, 815, y_pos, "LIGHT CONTROLLER", 1, gray)?;
    let phase_name = match controller.active_direction {
        Direction::North => "NORTH (SOUTHBOUND)",
        Direction::South => "SOUTH (NORTHBOUND)",
        Direction::East  => "EAST  (WESTBOUND)",
        Direction::West  => "WEST  (EASTBOUND)",
    };
    let state_str = match controller.state {
        LightState::Green => "GREEN",
        LightState::Yellow => "YELLOW",
        LightState::AllRed => "ALL RED (CLEARING)",
    };
    let state_color = match controller.state {
        LightState::Green => green,
        LightState::Yellow => yellow,
        LightState::AllRed => red,
    };
    draw_string(canvas, 815, y_pos + 20, "ACTIVE DIRECTION:", 1, white)?;
    draw_string(canvas, 815, y_pos + 35, phase_name, 1, green)?;
    draw_string(canvas, 815, y_pos + 50, &format!("STATE: {}", state_str), 1, state_color)?;
    draw_string(canvas, 815, y_pos + 65, &format!("TIMER: {:.1}S", controller.timer), 1, white)?;
    
    if controller.extended {
        draw_string(canvas, 815, y_pos + 80, "* GREEN EXTENDED *", 1, orange)?;
    }
    
    // Line separator
    y_pos += 100;
    canvas.set_draw_color(Color::RGB(55, 55, 62));
    canvas.fill_rect(Rect::new(815, y_pos, 170, 2))?;
    y_pos += 15;
    
    // Route coloring info
    draw_string(canvas, 815, y_pos, "ROUTE LEGEND (COLORS)", 1, gray)?;
    draw_string(canvas, 815, y_pos + 20, "STRAIGHT (CYAN)", 1, cyan)?;
    draw_string(canvas, 815, y_pos + 35, "LEFT TURN (ORANGE)", 1, orange)?;
    draw_string(canvas, 815, y_pos + 50, "RIGHT TURN (YELLOW)", 1, yellow)?;
    
    // Line separator
    y_pos += 70;
    canvas.set_draw_color(Color::RGB(55, 55, 62));
    canvas.fill_rect(Rect::new(815, y_pos, 170, 2))?;
    y_pos += 15;
    
    // Controls Legend
    draw_string(canvas, 815, y_pos, "KEYBOARD CONTROLS", 1, gray)?;
    draw_string(canvas, 815, y_pos + 20, "UP:    SPAWN SOUTH", 1, white)?;
    draw_string(canvas, 815, y_pos + 35, "DOWN:  SPAWN NORTH", 1, white)?;
    draw_string(canvas, 815, y_pos + 50, "RIGHT: SPAWN WEST", 1, white)?;
    draw_string(canvas, 815, y_pos + 65, "LEFT:  SPAWN EAST", 1, white)?;
    draw_string(canvas, 815, y_pos + 80, "R:     SPAWN RANDOM", 1, white)?;
    draw_string(canvas, 815, y_pos + 95, "ESC:   EXIT SIM", 1, white)?;
    
    Ok(())
}

pub fn draw_vehicles<T: sdl2::render::RenderTarget>(
    canvas: &mut Canvas<T>,
    vehicles: &[Vehicle],
    tex_straight: &Texture,
    tex_left: &Texture,
    tex_right: &Texture,
) -> Result<(), String> {
    for car in vehicles {
        if car.current_waypoint >= car.path.len() { continue; }
        
        if car.x >= -30.0 && car.x <= 830.0 && car.y >= -30.0 && car.y <= 830.0 {
            let target = car.path[car.current_waypoint];
            let dx = target.0 - car.x;
            let dy = target.1 - car.y;
            let angle = if (dx*dx + dy*dy).sqrt() > 0.1 {
                dy.atan2(dx).to_degrees() as f64
            } else {
                0.0
            };
            
            let tex = match car.route {
                Route::Straight => tex_straight,
                Route::Left => tex_left,
                Route::Right => tex_right,
            };
            
            let dst = Rect::new(
                (car.x - 15.0) as i32,
                (car.y - 9.0) as i32,
                30,
                18,
            );
            
            canvas.copy_ex(
                tex,
                None,
                Some(dst),
                angle,
                None,
                false,
                false,
            )?;
        }
    }
    Ok(())
}
