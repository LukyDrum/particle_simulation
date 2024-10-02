use rand::random;

use crate::{frame::Frame, offset::Offset, particle::Particle};

pub struct Simulation {
    width: usize,
    height: usize,
    bg_color: u32,
    particles: Vec<Option<Particle>>,
}

impl Simulation {
    pub fn new(width: usize, height: usize) -> Simulation {
        Simulation {
            width,
            height,
            bg_color: 0x00000000,
            particles: vec![None; width * height],
        }
    }

    pub fn draw_to_frame(&self, frame: &mut Frame) -> () {
        for y in 0..self.height {
            for x in 0..self.width {
                let opt = &self.particles[y * self.width + x];

                let color = match opt {
                    Some(p) => p.color,
                    None => self.bg_color,
                };

                let _ = frame.draw_pixel(x, y, color);
            }
        }
    }

    pub fn add_particle(&mut self, offset: &Offset, particle: Particle) -> bool {
        if !self.is_within(&offset) {
            return false;
        }

        let index = self.offset_to_index(offset);
        if self.particles[index].is_none() {
            self.particles[index] = Some(particle);

            return true;
        }

        false
    }

    pub fn remove_particle(&mut self, offset: &Offset) -> bool {
        if !self.is_within(&offset) {
            return false;
        }

        let index = self.offset_to_index(offset);
        let opt = self.particles[index];

        match opt {
            None => return false,
            Some(_) => {
                self.particles[index] = None;

                return true;
            }
        }
    }

    pub fn get_particle(&self, offset: &Offset) -> &Option<Particle> {
        if !self.is_within(&offset) {
            return &None;
        }

        &self.particles[self.offset_to_index(offset)]
    }

    pub fn change_particle(&mut self, offset: &Offset, new_particle: &Particle) -> () {
        if !self.is_within(&offset) {
            return;
        }

        let index = self.offset_to_index(offset);
        self.particles[index] = Some(*new_particle);
    }

    pub fn simulate_step(&mut self) -> () {}
}

impl Simulation {
    fn is_within(&self, offset: &Offset) -> bool {
        offset.x >= 0
            && offset.y >= 0
            && offset.x < self.width as i32
            && offset.y < self.height as i32
    }

    fn offset_to_index(&self, offset: &Offset) -> usize {
        self.width * offset.y as usize + offset.x as usize
    }

    fn index_to_offset(&self, index: usize) -> Offset {
        let y = index / self.width;
        let x = index - (y * self.width);

        Offset::new(x as i32, y as i32)
    }
}
