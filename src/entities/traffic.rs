use crate::config::*;
use crate::types::*;
use super::vehicle::Vehicle;
use rand::Rng;

/// Represents the entire intersection with roads, traffic lights, and vehicles
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Intersection {
    // Vehicles waiting at lights
    pub vehicles_waiting_north: Vec<Vehicle>,
    pub vehicles_waiting_south: Vec<Vehicle>,
    pub vehicles_waiting_east: Vec<Vehicle>,
    pub vehicles_waiting_west: Vec<Vehicle>,
    
    // Vehicles in the intersection
    pub vehicles_in_intersection: Vec<Vehicle>,
    
    // Vehicles that have passed through intersection
    pub vehicles_passed_north: Vec<Vehicle>,
    pub vehicles_passed_south: Vec<Vehicle>,
    pub vehicles_passed_east: Vec<Vehicle>,
    pub vehicles_passed_west: Vec<Vehicle>,
    
    // Traffic lights
    pub north_light: TrafficLight,
    pub south_light: TrafficLight,
    pub east_light: TrafficLight,
    pub west_light: TrafficLight,
    
    // Metrics
    pub total_vehicles_processed: usize,
    pub simulation_time: u32,
}

impl Intersection {
    /// Creates a new intersection with all lights red and no vehicles
    pub fn new() -> Self {
        Self {
            vehicles_waiting_north: vec![],
            vehicles_waiting_south: vec![],
            vehicles_waiting_east: vec![],
            vehicles_waiting_west: vec![],
            vehicles_in_intersection: vec![],
            vehicles_passed_north: vec![],
            vehicles_passed_south: vec![],
            vehicles_passed_east: vec![],
            vehicles_passed_west: vec![],
            north_light: TrafficLight::red(),
            south_light: TrafficLight::red(),
            east_light: TrafficLight::red(),
            west_light: TrafficLight::red(),
            total_vehicles_processed: 0,
            simulation_time: 0,
        }
    }
    
    /// Performs a single update step for the entire simulation
    pub fn update(&mut self) {
        // Update traffic light states
        self.update_traffic_lights();
        
        // Process vehicles in the intersection
        self.process_intersection_vehicles();
        
        // Process vehicles that have passed through
        self.process_passed_vehicles();
        
        // Process vehicles waiting at lights
        self.process_waiting_vehicles();
        
        // Increment simulation time
        self.simulation_time += 1;
    }
    
    /// Updates the traffic light states based on traffic conditions
    fn update_traffic_lights(&mut self) {
        // Reset all lights to red first
        let all_green = self.east_light.state == LightState::Green || 
                        self.west_light.state == LightState::Green || 
                        self.north_light.state == LightState::Green || 
                        self.south_light.state == LightState::Green;
                        
        if all_green {
            // If any light is green, turn all lights red (reset for next cycle)
            self.east_light.state = LightState::Red;
            self.west_light.state = LightState::Red;
            self.north_light.state = LightState::Red;
            self.south_light.state = LightState::Red;
        } else {
            // Set a light to green based on queue length
            let east_count = self.vehicles_waiting_east.len();
            let west_count = self.vehicles_waiting_west.len();
            let north_count = self.vehicles_waiting_north.len();
            let south_count = self.vehicles_waiting_south.len();
            
            // Find the direction with the most waiting vehicles
            if east_count >= west_count && east_count >= north_count && east_count >= south_count && east_count > 0 {
                self.east_light.state = LightState::Green;
            } else if west_count >= east_count && west_count >= north_count && west_count >= south_count && west_count > 0 {
                self.west_light.state = LightState::Green;
            } else if north_count >= east_count && north_count >= west_count && north_count >= south_count && north_count > 0 {
                self.north_light.state = LightState::Green;
            } else if south_count > 0 {
                self.south_light.state = LightState::Green;
            }
        }
    }
    
    /// Processes vehicles currently in the intersection
    fn process_intersection_vehicles(&mut self) {
        if self.vehicles_in_intersection.is_empty() {
            return;
        }
        
        // Move the vehicle in the intersection
        self.vehicles_in_intersection[0].update_position(&TrafficLight::green());
        
        // Check if the vehicle has left the intersection area
        let vehicle = &self.vehicles_in_intersection[0];
        let outside_x = vehicle.x > WINDOW_WIDTH / 2 + VEHICLE_WIDTH || 
                        vehicle.x < WINDOW_WIDTH / 2 - 2 * VEHICLE_WIDTH;
        let outside_y = vehicle.y > WINDOW_HEIGHT / 2 + VEHICLE_HEIGHT || 
                        vehicle.y < WINDOW_HEIGHT / 2 - 2 * VEHICLE_HEIGHT;
                        
        if outside_x || outside_y {
            let vehicle = self.vehicles_in_intersection.remove(0);
            self.total_vehicles_processed += 1;
            
            // Determine which exit vector to add the vehicle to
            match (vehicle.side, vehicle.direction) {
                // Vehicles going north
                (Side::FromEast, Direction::Left) | 
                (Side::FromSouth, Direction::Straight) | 
                (Side::FromWest, Direction::Right) => {
                    self.vehicles_passed_north.push(vehicle);
                },
                
                // Vehicles going east
                (Side::FromSouth, Direction::Left) | 
                (Side::FromWest, Direction::Straight) | 
                (Side::FromNorth, Direction::Right) => {
                    self.vehicles_passed_east.push(vehicle);
                },
                
                // Vehicles going south
                (Side::FromWest, Direction::Left) | 
                (Side::FromNorth, Direction::Straight) | 
                (Side::FromEast, Direction::Right) => {
                    self.vehicles_passed_south.push(vehicle);
                },
                
                // Vehicles going west
                (Side::FromNorth, Direction::Left) | 
                (Side::FromEast, Direction::Straight) | 
                (Side::FromSouth, Direction::Right) => {
                    self.vehicles_passed_west.push(vehicle);
                },
            }
        }
    }
    
    /// Processes vehicles that have passed through the intersection
    fn process_passed_vehicles(&mut self) {
        // Process vehicles that have exited east
        if !self.vehicles_passed_east.is_empty() {
            self.vehicles_passed_east[0].update_position(&TrafficLight::green());
            
            // Process following vehicles with safety distance
            if self.vehicles_passed_east.len() > 1 {
                let mut front_vehicle = self.vehicles_passed_east[0].clone();
                
                for i in 1..self.vehicles_passed_east.len() {
                    if self.vehicles_passed_east[i].x - self.vehicles_passed_east[i].velocity > 
                       front_vehicle.x + SAFETY_DISTANCE {
                        self.vehicles_passed_east[i].update_position(&TrafficLight::green());
                        front_vehicle = self.vehicles_passed_east[i].clone();
                    }
                }
            }
        }
        
        // Process vehicles that have exited north
        if !self.vehicles_passed_north.is_empty() {
            self.vehicles_passed_north[0].update_position(&TrafficLight::green());
            
            // Process following vehicles with safety distance
            if self.vehicles_passed_north.len() > 1 {
                let mut front_vehicle = self.vehicles_passed_north[0].clone();
                
                for i in 1..self.vehicles_passed_north.len() {
                    if self.vehicles_passed_north[i].y - self.vehicles_passed_north[i].velocity > 
                       front_vehicle.y + SAFETY_DISTANCE {
                        self.vehicles_passed_north[i].update_position(&TrafficLight::green());
                        front_vehicle = self.vehicles_passed_north[i].clone();
                    }
                }
            }
        }
        
        // Process vehicles that have exited west
        if !self.vehicles_passed_west.is_empty() {
            self.vehicles_passed_west[0].update_position(&TrafficLight::green());
            
            // Process following vehicles with safety distance
            if self.vehicles_passed_west.len() > 1 {
                let mut front_vehicle = self.vehicles_passed_west[0].clone();
                
                for i in 1..self.vehicles_passed_west.len() {
                    if self.vehicles_passed_west[i].x + self.vehicles_passed_west[i].velocity < 
                       front_vehicle.x - SAFETY_DISTANCE {
                        self.vehicles_passed_west[i].update_position(&TrafficLight::green());
                        front_vehicle = self.vehicles_passed_west[i].clone();
                    }
                }
            }
        }
        
        // Process vehicles that have exited south
        if !self.vehicles_passed_south.is_empty() {
            self.vehicles_passed_south[0].update_position(&TrafficLight::green());
            
            // Process following vehicles with safety distance
            if self.vehicles_passed_south.len() > 1 {
                let mut front_vehicle = self.vehicles_passed_south[0].clone();
                
                for i in 1..self.vehicles_passed_south.len() {
                    if self.vehicles_passed_south[i].y + self.vehicles_passed_south[i].velocity < 
                       front_vehicle.y - SAFETY_DISTANCE {
                        self.vehicles_passed_south[i].update_position(&TrafficLight::green());
                        front_vehicle = self.vehicles_passed_south[i].clone();
                    }
                }
            }
        }
    }
    
    /// Processes vehicles waiting at traffic lights
    fn process_waiting_vehicles(&mut self) {
        // Process vehicles waiting at the north light
        self.process_north_waiting_vehicles();
        
        // Process vehicles waiting at the south light
        self.process_south_waiting_vehicles();
        
        // Process vehicles waiting at the east light
        self.process_east_waiting_vehicles();
        
        // Process vehicles waiting at the west light
        self.process_west_waiting_vehicles();
    }
    
    /// Processes vehicles waiting at the north traffic light
    fn process_north_waiting_vehicles(&mut self) {
        if self.vehicles_waiting_north.is_empty() {
            return;
        }
        
        // Check if first vehicle should enter the intersection
        let first_at_light = self.vehicles_waiting_north[0].y >= (WINDOW_HEIGHT / 2 - 2 * VEHICLE_HEIGHT);
        if first_at_light && self.north_light.state == LightState::Green && self.vehicles_in_intersection.is_empty() {
            let vehicle = self.vehicles_waiting_north.remove(0);
            self.vehicles_in_intersection.push(vehicle);
            return;
        }
        
        // Move vehicles before the light
        if !self.vehicles_waiting_north.is_empty() && 
           self.vehicles_waiting_north[0].y < (WINDOW_HEIGHT / 2 - 2 * VEHICLE_HEIGHT) {
            self.vehicles_waiting_north[0].update_position(&self.north_light);
            
            // Move following vehicles with safety distance
            if self.vehicles_waiting_north.len() > 1 {
                let mut front_y = self.vehicles_waiting_north[0].y;
                
                for i in 1..self.vehicles_waiting_north.len() {
                    if self.vehicles_waiting_north[i].y + self.vehicles_waiting_north[i].velocity < 
                       front_y - SAFETY_DISTANCE {
                        self.vehicles_waiting_north[i].update_position(&self.north_light);
                        front_y = self.vehicles_waiting_north[i].y;
                    }
                }
            }
        }
    }
    
    /// Processes vehicles waiting at the south traffic light
    fn process_south_waiting_vehicles(&mut self) {
        if self.vehicles_waiting_south.is_empty() {
            return;
        }
        
        // Check if first vehicle should enter the intersection
        let first_at_light = self.vehicles_waiting_south[0].y <= (WINDOW_HEIGHT / 2 + 2 * VEHICLE_HEIGHT);
        if first_at_light && self.south_light.state == LightState::Green && self.vehicles_in_intersection.is_empty() {
            let vehicle = self.vehicles_waiting_south.remove(0);
            self.vehicles_in_intersection.push(vehicle);
            return;
        }
        
        // Move vehicles before the light
        if !self.vehicles_waiting_south.is_empty() && 
           self.vehicles_waiting_south[0].y > (WINDOW_HEIGHT / 2 + 2 * VEHICLE_HEIGHT) {
            self.vehicles_waiting_south[0].update_position(&self.south_light);
            
            // Move following vehicles with safety distance
            if self.vehicles_waiting_south.len() > 1 {
                let mut front_y = self.vehicles_waiting_south[0].y;
                
                for i in 1..self.vehicles_waiting_south.len() {
                    if self.vehicles_waiting_south[i].y - self.vehicles_waiting_south[i].velocity > 
                       front_y + SAFETY_DISTANCE {
                        self.vehicles_waiting_south[i].update_position(&self.south_light);
                        front_y = self.vehicles_waiting_south[i].y;
                    }
                }
            }
        }
    }
    
    /// Processes vehicles waiting at the east traffic light
    fn process_east_waiting_vehicles(&mut self) {
        if self.vehicles_waiting_east.is_empty() {
            return;
        }
        
        // Check if first vehicle should enter the intersection
        let first_at_light = self.vehicles_waiting_east[0].x >= (WINDOW_WIDTH / 2 - 2 * VEHICLE_WIDTH);
        if first_at_light && self.east_light.state == LightState::Green && self.vehicles_in_intersection.is_empty() {
            let vehicle = self.vehicles_waiting_east.remove(0);
            self.vehicles_in_intersection.push(vehicle);
            return;
        }
        
        // Move vehicles before the light
        if !self.vehicles_waiting_east.is_empty() && 
           self.vehicles_waiting_east[0].x < (WINDOW_WIDTH / 2 - 2 * VEHICLE_WIDTH) {
            self.vehicles_waiting_east[0].update_position(&self.east_light);
            
            // Move following vehicles with safety distance
            if self.vehicles_waiting_east.len() > 1 {
                let mut front_x = self.vehicles_waiting_east[0].x;
                
                for i in 1..self.vehicles_waiting_east.len() {
                    if self.vehicles_waiting_east[i].x + self.vehicles_waiting_east[i].velocity < 
                       front_x - SAFETY_DISTANCE {
                        self.vehicles_waiting_east[i].update_position(&self.east_light);
                        front_x = self.vehicles_waiting_east[i].x;
                    }
                }
            }
        }
    }
    
    /// Processes vehicles waiting at the west traffic light
    fn process_west_waiting_vehicles(&mut self) {
        if self.vehicles_waiting_west.is_empty() {
            return;
        }
        
        // Check if first vehicle should enter the intersection
        let first_at_light = self.vehicles_waiting_west[0].x <= (WINDOW_WIDTH / 2 + VEHICLE_WIDTH);
        if first_at_light && self.west_light.state == LightState::Green && self.vehicles_in_intersection.is_empty() {
            let vehicle = self.vehicles_waiting_west.remove(0);
            self.vehicles_in_intersection.push(vehicle);
            return;
        }
        
        // Move vehicles before the light
        if !self.vehicles_waiting_west.is_empty() && 
           self.vehicles_waiting_west[0].x > (WINDOW_WIDTH / 2 + VEHICLE_WIDTH) {
            self.vehicles_waiting_west[0].update_position(&self.west_light);
            
            // Move following vehicles with safety distance
            if self.vehicles_waiting_west.len() > 1 {
                let mut front_x = self.vehicles_waiting_west[0].x;
                
                for i in 1..self.vehicles_waiting_west.len() {
                    if self.vehicles_waiting_west[i].x - self.vehicles_waiting_west[i].velocity > 
                       front_x + SAFETY_DISTANCE {
                        self.vehicles_waiting_west[i].update_position(&self.west_light);
                        front_x = self.vehicles_waiting_west[i].x;
                    }
                }
            }
        }
    }
    
    /// Creates a new vehicle from the east if there's sufficient space
    pub fn spawn_vehicle_from_east(&mut self) -> bool {
        if self.vehicles_waiting_east.is_empty() || 
           self.vehicles_waiting_east.last().unwrap().x > SAFETY_DISTANCE {
            self.vehicles_waiting_east.push(Vehicle::new(Side::FromEast));
            return true;
        }
        return false;
    }
    
    /// Creates a new vehicle from the west if there's sufficient space
    pub fn spawn_vehicle_from_west(&mut self) -> bool {
        if self.vehicles_waiting_west.is_empty() || 
           self.vehicles_waiting_west.last().unwrap().x < WINDOW_WIDTH - SAFETY_DISTANCE {
            self.vehicles_waiting_west.push(Vehicle::new(Side::FromWest));
            return true;
        }
        return false;
    }
    
    /// Creates a new vehicle from the north if there's sufficient space
    pub fn spawn_vehicle_from_north(&mut self) -> bool {
        if self.vehicles_waiting_north.is_empty() || 
           self.vehicles_waiting_north.last().unwrap().y > SAFETY_DISTANCE {
            self.vehicles_waiting_north.push(Vehicle::new(Side::FromNorth));
            return true;
        }
        return false;
    }
    
    /// Creates a new vehicle from the south if there's sufficient space
    pub fn spawn_vehicle_from_south(&mut self) -> bool {
        if self.vehicles_waiting_south.is_empty() || 
           self.vehicles_waiting_south.last().unwrap().y < WINDOW_HEIGHT - SAFETY_DISTANCE {
            self.vehicles_waiting_south.push(Vehicle::new(Side::FromSouth));
            return true;
        }
        return false;
    }
    
    /// Creates a new vehicle from a random direction if there's sufficient space
    pub fn spawn_vehicle_random(&mut self) -> bool {
        let direction = rand::thread_rng().gen_range(0..4);
        match direction {
            0 => self.spawn_vehicle_from_east(),
            1 => self.spawn_vehicle_from_west(),
            2 => self.spawn_vehicle_from_north(),
            _ => self.spawn_vehicle_from_south(),
        }
    }
    
    /// Returns traffic statistics
    pub fn stats(&self) -> (usize, usize, usize, usize, usize) {
        let waiting = self.vehicles_waiting_east.len() + 
                      self.vehicles_waiting_west.len() + 
                      self.vehicles_waiting_north.len() + 
                      self.vehicles_waiting_south.len();
        
        let passing = self.vehicles_in_intersection.len();
        
        let passed = self.vehicles_passed_east.len() + 
                     self.vehicles_passed_west.len() + 
                     self.vehicles_passed_north.len() + 
                     self.vehicles_passed_south.len();
        
        (
            waiting,
            passing,
            passed,
            self.total_vehicles_processed,
            self.simulation_time as usize
        )
    }
}