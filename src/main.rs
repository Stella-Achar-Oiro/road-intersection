extern crate rand;
extern crate sdl2;

mod config;
mod types;
mod entities;

use entities::{Intersection, Vehicle};
use config::*;
use types::{Side, LightState};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

fn main() {
    // Initialize SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Create window
    let window = video_subsystem
        .window(WINDOW_TITLE, WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    // Create canvas
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Create intersection
    let mut intersection = Intersection::new();
    
    // Track last key press time to prevent spamming
    let mut last_key_press = std::time::Instant::now();
    let key_cooldown = std::time::Duration::from_millis(300);

    // Main simulation loop
    'running: loop {
        // Process events
        for event in event_pump.poll_iter() {
            match event {
                // Quit events
                Event::Quit { .. } | 
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                
                // Vehicle spawn events
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    let now = std::time::Instant::now();
                    if now.duration_since(last_key_press) < key_cooldown {
                        continue; // Prevent key spamming
                    }
                    
                    let spawn_successful = match keycode {
                        Keycode::Left => intersection.spawn_vehicle_from_east(),
                        Keycode::Right => intersection.spawn_vehicle_from_west(),
                        Keycode::Up => intersection.spawn_vehicle_from_south(),
                        Keycode::Down => intersection.spawn_vehicle_from_north(),
                        Keycode::R => intersection.spawn_vehicle_random(),
                        _ => false,
                    };
                    
                    if spawn_successful {
                        last_key_press = now;
                    }
                },
                _ => {},
            }
        }

        // Clear canvas
        canvas.set_draw_color(BACKGROUND_COLOR);
        canvas.clear();

        // Draw roads
        draw_roads(&mut canvas);
        
        // Draw traffic lights
        draw_traffic_lights(&mut canvas, &intersection);
        
        // Draw vehicles
        draw_vehicles(&mut canvas, &intersection);
        
        // Draw UI info
        draw_ui_info(&mut canvas, &intersection);

        // Update simulation
        intersection.update();

        // Present canvas
        canvas.present();

        // Control frame rate
        std::thread::sleep(Duration::from_millis(1000 / FPS));
    }
}

/// Draws the road layout
fn draw_roads(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    // Draw background road area
    canvas.set_draw_color(ROAD_COLOR);
    
    // Horizontal road
    canvas.fill_rect(Rect::new(
        0, 
        WINDOW_HEIGHT / 2 - VEHICLE_HEIGHT * 2, 
        WINDOW_WIDTH as u32, 
        VEHICLE_HEIGHT as u32 * 4
    )).unwrap();
    
    // Vertical road
    canvas.fill_rect(Rect::new(
        WINDOW_WIDTH / 2 - VEHICLE_WIDTH * 2, 
        0, 
        VEHICLE_WIDTH as u32 * 4, 
        WINDOW_HEIGHT as u32
    )).unwrap();
    
    // Draw road markings
    canvas.set_draw_color(ROAD_MARKING_COLOR);
    
    // North-South lane divider
    let dash_length = 20;
    let gap_length = 10;
    let mut y_pos = 0;
    
    while y_pos < WINDOW_HEIGHT {
        canvas.fill_rect(Rect::new(
            WINDOW_WIDTH / 2 - 1,
            y_pos,
            2,
            dash_length as u32
        )).unwrap();
        
        y_pos += dash_length + gap_length;
    }
    
    // East-West lane divider
    let mut x_pos = 0;
    
    while x_pos < WINDOW_WIDTH {
        canvas.fill_rect(Rect::new(
            x_pos,
            WINDOW_HEIGHT / 2 - 1,
            dash_length as u32,
            2
        )).unwrap();
        
        x_pos += dash_length + gap_length;
    }
    
    // Draw intersection boundary
    canvas.draw_rect(Rect::new(
        WINDOW_WIDTH / 2 - VEHICLE_WIDTH * 2,
        WINDOW_HEIGHT / 2 - VEHICLE_HEIGHT * 2,
        VEHICLE_WIDTH as u32 * 4,
        VEHICLE_HEIGHT as u32 * 4
    )).unwrap();
}

/// Draws the traffic lights
fn draw_traffic_lights(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    intersection: &Intersection
) {
    // North light
    if intersection.north_light.state == LightState::Green {
        canvas.set_draw_color(LIGHT_GREEN);
    } else {
        canvas.set_draw_color(LIGHT_RED);
    }
    canvas.fill_rect(Rect::new(
        WINDOW_WIDTH / 2 - 2 * VEHICLE_WIDTH,
        WINDOW_HEIGHT / 2 - 2 * VEHICLE_HEIGHT,
        VEHICLE_WIDTH as u32,
        VEHICLE_HEIGHT as u32
    )).unwrap();
    
    // East light
    if intersection.east_light.state == LightState::Green {
        canvas.set_draw_color(LIGHT_GREEN);
    } else {
        canvas.set_draw_color(LIGHT_RED);
    }
    canvas.fill_rect(Rect::new(
        WINDOW_WIDTH / 2 - 2 * VEHICLE_WIDTH,
        WINDOW_HEIGHT / 2 + VEHICLE_HEIGHT,
        VEHICLE_WIDTH as u32,
        VEHICLE_HEIGHT as u32
    )).unwrap();
    
    // South light
    if intersection.south_light.state == LightState::Green {
        canvas.set_draw_color(LIGHT_GREEN);
    } else {
        canvas.set_draw_color(LIGHT_RED);
    }
    canvas.fill_rect(Rect::new(
        WINDOW_WIDTH / 2 + VEHICLE_WIDTH,
        WINDOW_HEIGHT / 2 + VEHICLE_HEIGHT,
        VEHICLE_WIDTH as u32,
        VEHICLE_HEIGHT as u32
    )).unwrap();
    
    // West light
    if intersection.west_light.state == LightState::Green {
        canvas.set_draw_color(LIGHT_GREEN);
    } else {
        canvas.set_draw_color(LIGHT_RED);
    }
    canvas.fill_rect(Rect::new(
        WINDOW_WIDTH / 2 + VEHICLE_WIDTH,
        WINDOW_HEIGHT / 2 - 2 * VEHICLE_HEIGHT,
        VEHICLE_WIDTH as u32,
        VEHICLE_HEIGHT as u32
    )).unwrap();
}

/// Draws all vehicles
fn draw_vehicles(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    intersection: &Intersection
) {
    // Draw vehicles that have passed through intersection
    for vehicle in &intersection.vehicles_passed_east {
        draw_vehicle(canvas, vehicle);
    }
    for vehicle in &intersection.vehicles_passed_west {
        draw_vehicle(canvas, vehicle);
    }
    for vehicle in &intersection.vehicles_passed_north {
        draw_vehicle(canvas, vehicle);
    }
    for vehicle in &intersection.vehicles_passed_south {
        draw_vehicle(canvas, vehicle);
    }
    
    // Draw vehicles in the intersection
    for vehicle in &intersection.vehicles_in_intersection {
        draw_vehicle(canvas, vehicle);
    }
    
    // Draw vehicles waiting at lights
    for vehicle in &intersection.vehicles_waiting_east {
        draw_vehicle(canvas, vehicle);
    }
    for vehicle in &intersection.vehicles_waiting_west {
        draw_vehicle(canvas, vehicle);
    }
    for vehicle in &intersection.vehicles_waiting_north {
        draw_vehicle(canvas, vehicle);
    }
    for vehicle in &intersection.vehicles_waiting_south {
        draw_vehicle(canvas, vehicle);
    }
}

/// Draws a single vehicle
fn draw_vehicle(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    vehicle: &Vehicle
) {
    // Draw the vehicle body
    canvas.set_draw_color(vehicle.color);
    canvas.fill_rect(Rect::new(
        vehicle.x as i32,
        vehicle.y as i32,
        VEHICLE_WIDTH as u32,
        VEHICLE_HEIGHT as u32
    )).unwrap();
    
    // Add simple vehicle details (windows/lights)
    let dark_color = Color::RGB(
        vehicle.color.r / 2,
        vehicle.color.g / 2,
        vehicle.color.b / 2
    );
    
    canvas.set_draw_color(dark_color);
    
    // Draw "windows" - different patterns based on direction to help visualize
    match vehicle.side {
        Side::FromEast | Side::FromWest => {
            canvas.fill_rect(Rect::new(
                vehicle.x as i32 + 3,
                vehicle.y as i32 + 3,
                (VEHICLE_WIDTH - 6) as u32,
                5
            )).unwrap();
        },
        Side::FromNorth | Side::FromSouth => {
            canvas.fill_rect(Rect::new(
                vehicle.x as i32 + 3,
                vehicle.y as i32 + 3,
                5,
                (VEHICLE_HEIGHT - 6) as u32
            )).unwrap();
        }
    }
}

/// Draws UI information
fn draw_ui_info(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    intersection: &Intersection
) {
    // Get statistics
    let (waiting, passing, _passed, total, _) = intersection.stats();
    
    // Draw color key at the bottom of the screen
    let margin = 10;
    let key_width = 15;
    let key_height = 15;
    let spacing = 85;
    let y_position = WINDOW_HEIGHT - margin - key_height;
    
    // Left turn key (red)
    canvas.set_draw_color(VEHICLE_COLOR_LEFT);
    canvas.fill_rect(Rect::new(
        margin,
        y_position,
        key_width as u32,
        key_height as u32
    )).unwrap();
    
    // Straight key (blue)
    canvas.set_draw_color(VEHICLE_COLOR_STRAIGHT);
    canvas.fill_rect(Rect::new(
        margin + spacing,
        y_position,
        key_width as u32,
        key_height as u32
    )).unwrap();
    
    // Right turn key (green)
    canvas.set_draw_color(VEHICLE_COLOR_RIGHT);
    canvas.fill_rect(Rect::new(
        margin + spacing * 2,
        y_position,
        key_width as u32,
        key_height as u32
    )).unwrap();
    
    // Draw labels
    // Note: In a real implementation, you would render text using SDL2_ttf
    // For this simplified version, we'll just use colored blocks for the key
    
    // Vehicle counts
    let stats_y = margin;
    let stats_height = 10;
    let stats_spacing = 15;
    
    // Waiting vehicles indicator
    canvas.set_draw_color(Color::RGB(255, 255, 0));
    canvas.fill_rect(Rect::new(
        margin,
        stats_y,
        waiting as u32 * 5,
        stats_height as u32
    )).unwrap();
    
    // Passing vehicles indicator
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.fill_rect(Rect::new(
        margin,
        stats_y + stats_spacing,
        passing as u32 * 20,
        stats_height as u32
    )).unwrap();
    
    // Passed vehicles indicator
    canvas.set_draw_color(Color::RGB(255, 0, 255));
    canvas.fill_rect(Rect::new(
        margin,
        stats_y + stats_spacing * 2,
        (total.min(50)) as u32 * 2,
        stats_height as u32
    )).unwrap();
    
    // Controls reminder
    canvas.set_draw_color(Color::RGB(200, 200, 200));
    canvas.fill_rect(Rect::new(
        WINDOW_WIDTH - margin - 150,
        stats_y,
        150 as u32,
        stats_height as u32
    )).unwrap();
}