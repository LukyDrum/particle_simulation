use std::collections::HashMap;

use rand::Rng;

use crate::{frame::Frame, offset::Offset, particle::Particle};

struct SimInfo {
    pub particle_count: u32,
    pub moves_made_last_frame: u32,
}

impl SimInfo {
    pub fn new() -> SimInfo {
        SimInfo {
            particle_count: 0,
            moves_made_last_frame: 0,
        }
    }
}

pub struct Simulation {
    width: usize,
    height: usize,
    bg_color: u32,
    particles: Vec<Option<Particle>>,
    moves: HashMap<usize, Vec<usize>>, // Destination index, Indexes of particles that want to move there
    sim_info: SimInfo,
}

impl Simulation {
    pub fn new(width: usize, height: usize) -> Simulation {
        Simulation {
            width,
            height,
            bg_color: 0x00000000,
            particles: vec![None; width * height],
            moves: HashMap::new(),
            sim_info: SimInfo::new(),
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

        // Update Sim Info
        self.sim_info.particle_count += 1;

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

                // Update Sim Info
                self.sim_info.particle_count -= 1;

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

    pub fn simulate_step(&mut self) -> () {
        self.find_moves();
        self.apply_moves();

        // Print simulation informations.
        self.print_sim_info();

        self.clear_moves();
    }
}

impl Simulation {
    /// Finds desired moves for each particle
    fn find_moves(&mut self) -> () {
        for i in 0..(self.width * self.height) {
            let opt = &self.particles[i];

            match opt {
                None => {}
                Some(p) => {
                    // Particles current offset
                    let p_offset = self.index_to_offset(i);

                    for offset in p.get_offsets() {
                        // Create and check new particle offset
                        let new_offset = p_offset + offset;
                        if !self.is_within(&new_offset) {
                            continue;
                        }

                        // Convert to index and try to move
                        let new_index = self.offset_to_index(&new_offset);
                        if self.particles[new_index].is_none() {
                            self.add_move(i, new_index);
                            break;
                        }
                    }
                }
            }
        }
    }

    /// Adds a move to the moves map
    fn add_move(&mut self, from: usize, to: usize) -> () {
        if self.moves.contains_key(&to) {
            // Safe to unwrap as we checked for the key
            self.moves.get_mut(&to).unwrap().push(from);
        } else {
            self.moves.insert(to, vec![from]);
        }
    }

    /// Apply the moves in moves map
    fn apply_moves(&mut self) -> () {
        for (to, from_vec) in self.moves.iter() {
            let rand_index = rand::thread_rng().gen_range(0..from_vec.len());
            let chosen_from = from_vec[rand_index];

            // Move the particle
            self.particles[*to] = self.particles[chosen_from];
            // Free the old sport
            self.particles[chosen_from] = None;

            // Update Sim Info
            self.sim_info.moves_made_last_frame += 1;
        }
    }

    /// Clears the moves map
    fn clear_moves(&mut self) -> () {
        self.moves.clear();

        // Update Sim Info
        self.sim_info.moves_made_last_frame = 0;
    }

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

    fn print_sim_info(&self) -> () {
        println!("Particle count: {}", self.sim_info.particle_count);
        println!(
            "Move made last frame: {}",
            self.sim_info.moves_made_last_frame
        );
    }
}
