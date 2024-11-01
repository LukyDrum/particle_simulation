use dyn_clone::clone_box;
use fastrand;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rustc_hash::FxHashMap;
use std::collections::LinkedList;

use crate::{
    offset::Offset,
    particles::{Particle, ParticleChange},
    sprite::Sprite,
    Cell, Neighborhood,
};

pub struct SimInfo {
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

#[derive(Clone, Copy)]
enum SimMove {
    Move(usize),   // FROM
    Switch(usize), // FROM
}

pub struct Simulation {
    width: usize,
    height: usize,
    particles: Vec<Option<Box<dyn Particle>>>,
    moves: FxHashMap<usize, Vec<SimMove>>, // Destination index, Moves to be done ending at that index
    sim_info: SimInfo,
}

impl Simulation {
    pub fn new(width: usize, height: usize) -> Simulation {
        Simulation {
            width,
            height,
            particles: vec![None; width * height],
            moves: FxHashMap::default(),
            sim_info: SimInfo::new(),
        }
    }

    pub fn particles_iter(&self) -> std::slice::Iter<'_, Option<Box<dyn Particle>>> {
        self.particles.iter()
    }

    pub fn add_particle(&mut self, offset: &Offset, particle: Box<dyn Particle>) -> bool {
        if !self.is_within(&offset) {
            return false;
        }

        let index = self.offset_to_index(offset);
        if self.particles[index].is_none() {
            self.particles[index] = Some(particle);

            // Update Sim Info
            self.sim_info.particle_count += 1;

            return true;
        }

        false
    }

    pub fn remove_particle(&mut self, offset: &Offset) -> bool {
        if !self.is_within(&offset) {
            return false;
        }

        let index = self.offset_to_index(offset);
        let opt = &self.particles[index];

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

    pub fn get_particle(&self, offset: &Offset) -> &Option<Box<dyn Particle>> {
        if !self.is_within(&offset) {
            return &None;
        }

        &self.particles[self.offset_to_index(offset)]
    }

    pub fn change_particle(&mut self, offset: &Offset, new_particle: Box<dyn Particle>) -> () {
        if !self.is_within(&offset) {
            return;
        }

        let index = self.offset_to_index(offset);
        self.particles[index] = Some(new_particle);
    }

    pub fn simulate_step(&mut self) -> () {
        // Reset moves in sim info
        self.sim_info.moves_made_last_frame = 0;

        self.find_moves_multithreaded();
        self.apply_moves();

        self.clear_moves();

        // Update inner state of particles
        self.update_inner_states();
    }

    /// Inserts a sprite object into the simulation.
    /// Start offset represents where the top left corner of the sprite will be.
    /// Translate function defines how each color translates to a particle.
    pub fn insert_sprite(
        &mut self,
        sprite: Sprite,
        start_offset: &Offset,
        translate_fn: fn(u32) -> Box<dyn Particle>,
    ) -> () {
        for i in 0..sprite.pixels.len() {
            // Get color
            let color = sprite.pixels[i];
            // Calculate offset of the particle in the simulation
            let y = (i as u32) / sprite.width;
            let x = (i as u32) - (y * sprite.width);
            let p_offset = *start_offset + Offset::new(x as i32, y as i32);
            // Add particle into simulation
            self.add_particle(&p_offset, translate_fn(color));
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn info(&self) -> &SimInfo {
        &self.sim_info
    }
}

impl Simulation {
    /// This will find the possible moves and store them in "self" using multiple threads. This method calls "find_moves".
    fn find_moves_multithreaded(&mut self) -> () {
        // Multithread finding of the moves and store it in self

        // Find the start and end points for threads
        let mut start_end_tuples: LinkedList<(usize, usize)> = LinkedList::new();
        let thread_count = rayon::current_num_threads();
        let chunk_size = (self.width * self.height) / thread_count;
        for i in 0..thread_count {
            let start = i * chunk_size;
            // Define the end for each chunk
            let end = if i == thread_count - 1 {
                self.width * self.height // Ensure the last chunk goes to the end
            } else {
                (i + 1) * chunk_size
            };

            start_end_tuples.push_back((start, end));
        }

        // Using rayon find moves for each start end tuple
        let vec_of_partial_moves: Vec<LinkedList<(usize, SimMove)>> = start_end_tuples
            .par_iter()
            .map(|(start, end)| self.find_moves_in_range(*start, *end))
            .collect();

        // Join the moves into a map
        for part in vec_of_partial_moves {
            for (to, sim_move) in part.iter() {
                self.add_move(*to, *sim_move);
            }
        }
    }

    /// Finds desired moves for each particle
    fn find_moves_in_range(&self, start: usize, end: usize) -> LinkedList<(usize, SimMove)> {
        // Use list for more efficiency. These moves still has to be copied over to the total moves.
        // Contains tuples (from, to)
        let mut moves_list: LinkedList<(usize, SimMove)> = LinkedList::new();

        // Look at the given range
        for i in start..end {
            // Get the possible particle
            let opt = &self.particles[i];

            if let Some(p) = opt {
                // If the particle is not moveable, we can just skip it
                if !p.is_moveable() {
                    continue;
                }

                // Particles current offset
                let p_offset = self.index_to_offset(i);
                // Check the maximum offsets the particle would like to move to
                let max_offset = p.get_movement();
                // Continue to next particle if the offset is (0, 0)
                if max_offset.is_zero() {
                    continue;
                }
                // Find the maximum offset to which the particle CAN move
                // All necceseary check are done here
                let new_offset = self.find_max_offset(p_offset, max_offset, p);
                // Convert to index
                let new_index = self.offset_to_index(&new_offset);
                match self.particles[new_index] {
                    None => moves_list.push_back((new_index, SimMove::Move(i))),
                    Some(_) => moves_list.push_back((new_index, SimMove::Switch(i))),
                }
            }
        }

        moves_list
    }

    /// Adds a move to the moves map
    fn add_move(&mut self, to: usize, sim_move: SimMove) -> () {
        if let Some(vec) = self.moves.get_mut(&to) {
            vec.push(sim_move);
        } else {
            self.moves.insert(to, vec![sim_move]);
        }
    }

    /// Apply the moves in moves map
    fn apply_moves(&mut self) -> () {
        for (to, move_vec) in self.moves.iter() {
            let to = *to;

            let rand_index = fastrand::usize(0..move_vec.len());
            let chosen_move = move_vec[rand_index];

            match chosen_move {
                // Move to spot and increase velocity, as if by gravity
                SimMove::Move(from) => {
                    let opt = &self.particles[from];

                    if let Some(p) = opt {
                        let p = *clone_box(&*p);
                        // Move the particle
                        self.particles[to] = Some(p);
                        // Free the old sport
                        self.particles[from] = None;
                    }
                }
                // Switch particles on "to" and "with"
                SimMove::Switch(with) => {
                    let opt_on_to = &self.particles[to];
                    let opt_on_with = self.particles[with].clone();

                    if let Some(p) = opt_on_to {
                        let p = *clone_box(&*p);
                        self.particles[with] = Some(p);
                    } else {
                        self.particles[with] = None;
                    }

                    if let Some(p) = opt_on_with {
                        let p = clone_box(&*p);
                        self.particles[to] = Some(p);
                    } else {
                        self.particles[to] = None;
                    }
                }
            }

            // Update Sim Info
            self.sim_info.moves_made_last_frame += 1;
        }
    }

    /// Clears the moves map
    fn clear_moves(&mut self) -> () {
        self.moves.clear();
    }

    /// Updates the inner state of each particle
    fn update_inner_states(&mut self) -> () {
        // Get new particles, meaning new states
        let new_particles: Vec<(usize, Option<Box<dyn Particle>>)> = self
            .particles
            .par_iter()
            .enumerate()
            .map(|(index, opt)| {
                if let Some(p) = opt {
                    let offset = self.index_to_offset(index);
                    let neigborhood: Neighborhood = self.get_neighborhood(offset);
                    let p_change = p.update(neigborhood);

                    (index, p_change)
                } else {
                    (index, ParticleChange::None)
                }
            })
            .filter_map(|(index, p_change)| {
                if let ParticleChange::Changed(opt) = p_change {
                    Some((index, opt))
                } else {
                    None
                }
            })
            .collect();

        for (index, opt) in new_particles {
            self.particles[index] = opt;
        }
    }

    // Find the maximum offset to which a particle can either move to or switch to
    fn find_max_offset(
        &self,
        p_offset: Offset,
        max_offset: Offset,
        particle: &Box<dyn Particle>,
    ) -> Offset {
        // Get all the offsets between
        let max_pos = p_offset + max_offset;
        let offsets_between = p_offset.between(&max_pos);

        // Check if there is any obstacle, return the one furthest away but before an obstacle
        for i in 1..offsets_between.len() {
            let offset = offsets_between[i];
            // Check bounds
            if !self.is_within(&offset) {
                return offsets_between[i - 1];
            }

            let index = self.offset_to_index(&offset);
            let opt = &self.particles[index];

            if let Some(other_p) = opt {
                // If other_p does not have lower density, then we won't be able to switch
                if !(particle.can_switch_with(other_p)) {
                    return offsets_between[i - 1];
                }
            }
        }

        max_pos
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

    fn get_neighborhood(&self, offset: Offset) -> Neighborhood {
        let mut neigh: Neighborhood = Neighborhood(vec![vec![&Cell::Outside; 3]; 3]);

        for row_off in -1..=1 {
            for col_off in -1..=1 {
                let new_offset = offset + Offset::new(col_off, row_off);
                let row = (row_off + 1) as usize;
                let col = (col_off + 1) as usize;

                if self.is_within(&new_offset) {
                    neigh.0[row][col] = &Cell::Inside(self.get_particle(&new_offset).clone());
                }
            }
        }

        neigh
    }
}
