use dyn_clone::clone_box;
use rand::Rng;
use rayon::{
    iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use rustc_hash::FxHashMap;
use std::collections::LinkedList;

use crate::{
    frame::Frame,
    offset::Offset,
    particles::{constants::*, Particle},
    sprite::Sprite,
};

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

#[derive(Clone, Copy)]
enum SimMove {
    Move(usize),   // FROM
    Switch(usize), // FROM
    Stop,          // Happens when particle with velocity stops in place
}

#[derive(Clone, Copy)]
enum SimUpdate {
    SetOnFire(usize), // Sets the particle at index on fire
}

pub struct Simulation {
    width: usize,
    height: usize,
    bg_color: u32,
    particles: Vec<Option<Box<dyn Particle>>>,
    moves: FxHashMap<usize, Vec<SimMove>>, // Destination index, Moves to be done ending at that index
    updates: LinkedList<SimUpdate>,
    sim_info: SimInfo,
    pub print_debug: bool,
}

impl Simulation {
    pub fn new(width: usize, height: usize) -> Simulation {
        Simulation {
            width,
            height,
            bg_color: 0x00000000,
            particles: vec![None; width * height],
            moves: FxHashMap::default(),
            updates: LinkedList::new(),
            sim_info: SimInfo::new(),
            print_debug: false,
        }
    }

    pub fn draw_to_frame(&self, frame: &mut Frame) -> () {
        let logical_pixel_size = frame.logical_scale;
        let real_row_width = frame.width();
        let chunk_size = real_row_width * frame.logical_scale; // 1 row of log. pixel correspond to this much of real pixels
        let buffer = &mut frame.buffer;

        buffer
            .par_chunks_mut(chunk_size)
            .enumerate()
            .for_each(|(row, chunk)| {
                // Draw logical pixels
                for col in (0..real_row_width).step_by(logical_pixel_size) {
                    let particle_index = row * self.width + (col / logical_pixel_size);
                    let opt = &self.particles[particle_index];
                    let color = match opt {
                        Some(p) => p.get_color(),
                        None => self.bg_color,
                    };

                    for sub_col in 0..logical_pixel_size {
                        for sub_row in 0..logical_pixel_size {
                            let pixel_index = sub_row * real_row_width + col + sub_col;
                            chunk[pixel_index] = color;
                        }
                    }
                }
            });
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
        self.find_moves_multithreaded();
        self.apply_moves();
        // Print simulation informations.
        if self.print_debug {
            self.print_sim_info();
        }

        self.clear_moves();
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
                let mut did_move = false;
                // Check the maximum offsets the particle would like to move to
                for max_offset in p.get_max_offsets() {
                    // Find the maximum offset to which the particle CAN move
                    let new_offset = self.find_max_offset(p_offset, max_offset, p);

                    // Check for out of bounds
                    if !self.is_within(&new_offset) {
                        continue;
                    }

                    // Convert to index
                    let new_index = self.offset_to_index(&new_offset);

                    // Try for SimMove::MOVE
                    if self.particles[new_index].is_none() {
                        // Add the value to moves list
                        moves_list.push_back((new_index, SimMove::Move(i)));
                        did_move = true;
                        break;
                    }

                    // Try for SimMove::SWITCH
                    if let Some(other_p) = &self.particles[new_index] {
                        if Self::can_switch(p, other_p) {
                            // Add the value to moves list
                            moves_list.push_back((new_index, SimMove::Switch(i)));
                            did_move = true;
                            break;
                        }
                    }
                }

                // If the particle did not move (that means there was no chance for it to move) we should stop it's velocity (if it is greater than default velocity)
                if !did_move && p.get_velocity() > DEFAULT_VELOCITY {
                    moves_list.push_back((i, SimMove::Stop));
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

            let rand_index = rand::thread_rng().gen_range(0..move_vec.len());
            let chosen_move = move_vec[rand_index];

            match chosen_move {
                // Move to spot and increase velocity, as if by gravity
                SimMove::Move(from) => {
                    let opt = &self.particles[from];

                    if let Some(p) = opt {
                        let mut p = *clone_box(&*p);
                        // Check if the direction is aiming down
                        let direction = self.index_to_offset(to) - self.index_to_offset(from);
                        if direction.is_down() {
                            p.apply_acceleration(GRAVITY);
                        }

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

                    // TODO: Maybe calculate this value based on density?
                    let slow_down = 0.1;

                    if let Some(p) = opt_on_to {
                        let mut p = *clone_box(&*p);
                        p.apply_acceleration(-slow_down);
                        self.particles[with] = Some(p);
                    } else {
                        self.particles[with] = None;
                    }

                    if let Some(p) = opt_on_with {
                        let mut p = clone_box(&*p);
                        p.apply_acceleration(-slow_down);
                        self.particles[to] = Some(p);
                    } else {
                        self.particles[to] = None;
                    }
                }
                // Partcile does no move but still has velocity, then we should reset it's velocity
                SimMove::Stop => {
                    let mut opt = &mut self.particles[to];
                    if let Some(p) = &mut opt {
                        p.reset_velocity();
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

        // Update Sim Info
        self.sim_info.moves_made_last_frame = 0;
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
                if !(Self::can_switch(particle, other_p)) {
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

    fn print_sim_info(&self) -> () {
        println!("Particle count: {}", self.sim_info.particle_count);
        println!(
            "Move made last frame: {}",
            self.sim_info.moves_made_last_frame
        );
    }

    /// Particles can switch if other particle has lower density
    /// OR the particle has high enough speed and the other particle is not completely solid.
    fn can_switch(p: &Box<dyn Particle>, other_p: &Box<dyn Particle>) -> bool {
        other_p.get_density() < p.get_density()
            || (p.get_velocity() > DEFAULT_VELOCITY && !other_p.is_solid())
    }
}
