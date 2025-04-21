use sdl2::pixels::Color;

// Window configuration
pub const WINDOW_WIDTH: i32 = 800;
pub const WINDOW_HEIGHT: i32 = 800;
pub const WINDOW_TITLE: &str = "Road Intersection Simulation";

// Vehicle properties
pub const VEHICLE_WIDTH: i32 = 20;
pub const VEHICLE_HEIGHT: i32 = 20;
pub const MIN_VELOCITY: i32 = 2;
pub const MAX_VELOCITY: i32 = 3;
pub const SAFETY_DISTANCE: i32 = 30;

// Vehicle colors by direction
pub const VEHICLE_COLOR_LEFT: Color = Color::RGB(255, 0, 0);     // Red
pub const VEHICLE_COLOR_RIGHT: Color = Color::RGB(0, 255, 0);    // Green
pub const VEHICLE_COLOR_STRAIGHT: Color = Color::RGB(0, 0, 255); // Blue

// Road colors
pub const ROAD_COLOR: Color = Color::RGB(50, 50, 50);
pub const ROAD_MARKING_COLOR: Color = Color::RGB(255, 255, 255);

// Traffic light colors
pub const LIGHT_GREEN: Color = Color::RGB(0, 255, 0);
pub const LIGHT_RED: Color = Color::RGB(255, 0, 0);

// UI colors
pub const BACKGROUND_COLOR: Color = Color::RGB(0, 0, 0);

// Frame rate
pub const FPS: u64 = 60;