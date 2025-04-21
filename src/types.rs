//! Common types used throughout the simulation

/// Direction a vehicle can take at the intersection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Turn left at the intersection
    Left,
    /// Turn right at the intersection
    Right,
    /// Go straight through the intersection
    Straight,
}

/// Side from which a vehicle enters the intersection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    /// Vehicle coming from the south side
    FromSouth,
    /// Vehicle coming from the north side
    FromNorth,
    /// Vehicle coming from the west side
    FromWest,
    /// Vehicle coming from the east side
    FromEast,
}

/// Traffic light states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightState {
    /// Green light - vehicles can proceed
    Green,
    /// Red light - vehicles must stop
    Red,
}

/// Traffic light representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrafficLight {
    /// Current state of the traffic light
    pub state: LightState,
}

impl TrafficLight {
    /// Create a new traffic light with the given state
    // pub fn new(state: LightState) -> Self {
    //     Self { state }
    //}
    
    /// Create a new traffic light with a red state
    pub fn red() -> Self {
        Self { state: LightState::Red }
    }
    
    /// Create a new traffic light with a green state
    pub fn green() -> Self {
        Self { state: LightState::Green }
    }
}