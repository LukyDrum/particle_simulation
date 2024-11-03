use crate::particles::constants::CELL_DEFAULT_PRESSURE;
use crate::particles::Particle;

#[derive(Clone)]
pub struct Cell {
    particle: Option<Box<dyn Particle>>,
    pressure: i32,
}

impl Cell {
    pub fn empty() -> Self {
        Cell {
            particle: None,
            pressure: CELL_DEFAULT_PRESSURE,
        }
    }

    pub fn default_pressure() -> i32 {
        CELL_DEFAULT_PRESSURE
    }

    pub fn is_empty(&self) -> bool {
        match self.particle {
            Some(_) => false,
            None => true,
        }
    }

    pub fn get_particle(&self) -> &Option<Box<dyn Particle>> {
        &self.particle
    }

    pub fn set_particle(&mut self, new_particle: Box<dyn Particle>) -> () {
        self.particle = Some(new_particle)
    }

    pub fn set_particle_option(&mut self, new_particle_option: Option<Box<dyn Particle>>) -> () {
        self.particle = new_particle_option
    }

    pub fn remove_particle(&mut self) -> () {
        self.particle = None
    }

    pub fn get_pressure(&self) -> i32 {
        self.pressure
    }

    pub fn set_pressure(&mut self, new_pressure: i32) -> () {
        self.pressure = new_pressure
    }
}
