use std::collections::LinkedList;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::particles::constants::*;
use crate::particles::{get_near_color, Particle};
use crate::utility::get_value_around;
use crate::Offset;

use super::properties::PropertyCheckResult;
use super::{Burnability, Neighborhood, ParticleChange};

const COLOR: u32 = FIRE_COLOR;
/// Default lifetime in number of updates
const DEFAULT_LIFETIME: u8 = 200;
const LIFETIME_OFF: u8 = 50;

#[derive(Clone)]
pub struct Spark {
    color: u32,
    offsets: [Offset; 3],
    burnability: Burnability,
}

impl Spark {
    pub fn new() -> Box<dyn Particle> {
        Box::new(Spark {
            color: get_near_color(COLOR),
            offsets: [Offset::new(1, 0), Offset::new(-1, 0), Offset::new(0, 1)],
            burnability: Burnability::IsBurning(get_value_around(DEFAULT_LIFETIME, LIFETIME_OFF)),
        })
    }
}

impl Particle for Spark {
    fn get_name(&self) -> &str {
        "Spark"
    }

    fn get_color(&self) -> u32 {
        self.color
    }

    fn get_density(&self) -> u8 {
        MAX_DENSITY
    }

    fn _get_offsets(&self) -> LinkedList<Offset> {
        let mut indexes: Vec<usize> = (0..self.offsets.len()).collect();
        indexes.shuffle(&mut thread_rng());

        let mut lst = LinkedList::new();
        for i in indexes {
            lst.push_back(self.offsets[i]);
        }

        lst
    }

    fn is_moveable(&self) -> bool {
        true
    }

    fn is_solid(&self) -> bool {
        true
    }

    fn get_burnability(&self) -> Burnability {
        self.burnability
    }

    fn set_burnability(&mut self, new_burnability: Burnability) -> () {
        self.burnability = new_burnability;
    }

    fn update(&self, neigborhood: Neighborhood) -> ParticleChange {
        // Decrease burnability time or destroy the particle based on time left
        let mut new_spark = self.clone();

        let res = Burnability::check(&mut new_spark, &neigborhood, DEFAULT_LIFETIME, true);

        match res {
            PropertyCheckResult::Updated => ParticleChange::Changed(Some(Box::new(new_spark))),
            PropertyCheckResult::Destroyed => ParticleChange::Changed(None),
            PropertyCheckResult::None => ParticleChange::None,
        }
    }
}
