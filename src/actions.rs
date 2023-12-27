use bevy::prelude::*;
use crate::pleb::Pleb;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

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

// https://stackoverflow.com/questions/48490049/how-do-i-choose-a-random-value-from-an-enum
impl Distribution<CardinalDirection> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CardinalDirection {
        match rng.gen_range(0..4) { // rand 0.8
            0 => CardinalDirection::North,
            1 => CardinalDirection::East,
            2 => CardinalDirection::South,
            _ => CardinalDirection::West // We use underscore matching because gen_range does not know the limits of its generation
        }
    }
}

pub enum Action { 
    Walk(CardinalDirection), 
    Stay,
}

pub(crate) fn walk(transform: &mut Transform, pleb: &mut Pleb, direction: CardinalDirection) { 
    transform.translation += direction.to_velocity();
    pleb.energy = pleb.energy.checked_sub(1).unwrap_or(0);
}