use crate::config::*;
use crate::types::*;

use rand::Rng;
use sdl2::pixels::Color;

/// Represents a vehicle in the simulation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vehicle {
    /// X position of the vehicle
    pub x: i32,
    /// Y position of the vehicle
    pub y: i32,
    /// Color based on the intended direction
    pub color: Color,
    /// Direction the vehicle will take at the intersection
    pub direction: Direction,
    /// Side from which the vehicle enters the intersection
    pub side: Side,
    /// Movement speed of the vehicle
    pub velocity: i32,
}

impl Vehicle {
    /// Creates a new vehicle entering from the specified side
    pub fn new(side: Side) -> Self {
        let mut rng = rand::thread_rng();
        let random_direction = rng.gen_range(0..3);
        let velocity = rng.gen_range(MIN_VELOCITY..MAX_VELOCITY);
        
        // Determine direction and color
        let (direction, color) = match random_direction {
            0 => (Direction::Left, VEHICLE_COLOR_LEFT),
            1 => (Direction::Straight, VEHICLE_COLOR_STRAIGHT),
            _ => (Direction::Right, VEHICLE_COLOR_RIGHT),
        };
        
        // Set initial position based on entry side
        let (x, y) = match side {
            Side::FromEast => (0, WINDOW_HEIGHT / 2),
            Side::FromNorth => (WINDOW_WIDTH / 2 - VEHICLE_WIDTH, 0),
            Side::FromSouth => (WINDOW_WIDTH / 2, WINDOW_HEIGHT - VEHICLE_HEIGHT),
            Side::FromWest => (WINDOW_WIDTH - VEHICLE_WIDTH, WINDOW_HEIGHT / 2 - VEHICLE_HEIGHT),
        };
        
        Self {
            x, y, color, direction, side, velocity
        }
    }
    
    /// Updates the vehicle position based on its state and traffic light
    pub fn update_position(&mut self, traffic_light: &TrafficLight) {
        match self.side {
            Side::FromEast => self.update_from_east(traffic_light),
            Side::FromNorth => self.update_from_north(traffic_light),
            Side::FromSouth => self.update_from_south(traffic_light),
            Side::FromWest => self.update_from_west(traffic_light),
        }
    }
    
    /// Handles movement for vehicles coming from the east
    fn update_from_east(&mut self, traffic_light: &TrafficLight) {
        if self.x + self.velocity < WINDOW_WIDTH / 2 - 2 * VEHICLE_WIDTH {
            // Regular movement before intersection
            self.x += self.velocity;
        } else if self.x + self.velocity < WINDOW_WIDTH / 2 - VEHICLE_WIDTH {
            // Approaching intersection, check traffic light
            if traffic_light.state == LightState::Green {
                self.x += self.velocity;
            } else {
                // Stop at the light
                self.x = WINDOW_WIDTH / 2 - 2 * VEHICLE_WIDTH;
            }
        } else {
            // In intersection, route based on direction
            match self.direction {
                Direction::Left => {
                    // Turn left (north)
                    self.x = WINDOW_WIDTH / 2;
                    self.y -= self.velocity;
                },
                Direction::Right => {
                    // Turn right (south)
                    self.x = WINDOW_WIDTH / 2 - VEHICLE_WIDTH;
                    self.y += self.velocity;
                },
                Direction::Straight => {
                    // Continue west
                    self.x += self.velocity;
                },
            }
        }
    }
    
    /// Handles movement for vehicles coming from the north
    fn update_from_north(&mut self, traffic_light: &TrafficLight) {
        if self.y + self.velocity < WINDOW_HEIGHT / 2 - 2 * VEHICLE_HEIGHT {
            // Regular movement before intersection
            self.y += self.velocity;
        } else if self.y + self.velocity < WINDOW_HEIGHT / 2 - VEHICLE_HEIGHT {
            // Approaching intersection, check traffic light
            if traffic_light.state == LightState::Green {
                self.y += self.velocity;
            } else {
                // Stop at the light
                self.y = WINDOW_HEIGHT / 2 - 2 * VEHICLE_HEIGHT;
            }
        } else {
            // In intersection, route based on direction
            match self.direction {
                Direction::Left => {
                    // Turn left (east)
                    self.y = WINDOW_HEIGHT / 2;
                    self.x += self.velocity;
                },
                Direction::Right => {
                    // Turn right (west)
                    self.y = WINDOW_HEIGHT / 2 - VEHICLE_HEIGHT;
                    self.x -= self.velocity;
                },
                Direction::Straight => {
                    // Continue south
                    self.y += self.velocity;
                },
            }
        }
    }
    
    /// Handles movement for vehicles coming from the west
    fn update_from_west(&mut self, traffic_light: &TrafficLight) {
        if self.x - self.velocity > WINDOW_WIDTH / 2 + 2 * VEHICLE_WIDTH {
            // Regular movement before intersection
            self.x -= self.velocity;
        } else if self.x - self.velocity > WINDOW_WIDTH / 2 {
            // Approaching intersection, check traffic light
            if traffic_light.state == LightState::Green {
                self.x -= self.velocity;
            } else {
                // Stop at the light
                self.x = WINDOW_WIDTH / 2 + VEHICLE_WIDTH;
            }
        } else {
            // In intersection, route based on direction
            match self.direction {
                Direction::Left => {
                    // Turn left (south)
                    self.x = WINDOW_WIDTH / 2 - VEHICLE_WIDTH;
                    self.y += self.velocity;
                },
                Direction::Right => {
                    // Turn right (north)
                    self.x = WINDOW_WIDTH / 2;
                    self.y -= self.velocity;
                },
                Direction::Straight => {
                    // Continue east
                    self.x -= self.velocity;
                },
            }
        }
    }
    
    /// Handles movement for vehicles coming from the south
    fn update_from_south(&mut self, traffic_light: &TrafficLight) {
        if self.y - self.velocity > WINDOW_HEIGHT / 2 + 2 * VEHICLE_HEIGHT {
            // Regular movement before intersection
            self.y -= self.velocity;
        } else if self.y - self.velocity > WINDOW_HEIGHT / 2 {
            // Approaching intersection, check traffic light
            if traffic_light.state == LightState::Green {
                self.y -= self.velocity;
            } else {
                // Stop at the light
                self.y = WINDOW_HEIGHT / 2 + VEHICLE_HEIGHT;
            }
        } else {
            // In intersection, route based on direction
            match self.direction {
                Direction::Left => {
                    // Turn left (west)
                    self.y = WINDOW_HEIGHT / 2 - VEHICLE_HEIGHT;
                    self.x -= self.velocity;
                },
                Direction::Right => {
                    // Turn right (east)
                    self.y = WINDOW_HEIGHT / 2;
                    self.x += self.velocity;
                },
                Direction::Straight => {
                    // Continue north
                    self.y -= self.velocity;
                },
            }
        }
    }
}