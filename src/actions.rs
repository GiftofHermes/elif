use bevy::prelude::*;

pub enum CardinalDirection { 
    North, 
    East, 
    South, 
    West
}

impl CardinalDirection { 
    pub(crate) fn to_velocity(&self) -> Vec3 {
    match self {
        CardinalDirection::North => Vec3::new(0.0, 1.0, 0.0),
        CardinalDirection::East => Vec3::new(1.0, 0.0, 0.0),
        CardinalDirection::South => Vec3::new(0.0, -1.0, 0.0),
        CardinalDirection::West => Vec3::new(-1.0, 0.0, 0.0),
        }
    }
}

pub enum Action { 
    Walk(CardinalDirection), 
    Stay,
}