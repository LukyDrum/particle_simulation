use rand::Rng;
use rustc_hash::FxHashMap;
use std::thread;

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

const DEFAULT_THREAD_COUNT: usize = 8;

pub struct Simulation {
    width: usize,
    height: usize,
    bg_color: u32,
    particles: Vec<Option<Particle>>,
    moves: FxHashMap<usize, Vec<usize>>, // Destination index, Indexes of particles that want to move there
    sim_info: SimInfo,
    pub print_debug: bool,
    pub thread_count: usize,
}

impl Simulation {
    pub fn new(width: usize, height: usize) -> Simulation {
        Simulation {
            width,
            height,
            bg_color: 0x00000000,
            particles: vec![None; width * height],
            moves: FxHashMap::default(),
            sim_info: SimInfo::new(),
            print_debug: false,
            thread_count: DEFAULT_THREAD_COUNT,
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
        // Multithread finding of the moves
        // Create a thread scope
        self.moves = thread::scope(|scope| {
            // This will hold thread handles
            // let mut handles = Vec::new();
            // This tells the size of each chunk for a thread
            let chunk_size = (self.width * self.height) / self.thread_count;

            // Iterate over threads
            /*
            for i in 0..1 {
                let start = i * chunk_size;
                // Define the end for each chunk
                let end = if i == self.thread_count - 1 {
                    self.width * self.height // ensure the last chunk goes to the end
                } else {
                    (i + 1) * chunk_size
                };

                let closure = {
                    let (start, end, slf) = (start, end, &self);
                    move || slf.find_moves(0, end)
                };

                // Spawn threads for each part
                handles.push(scope.spawn(closure));
            }
            */

            // let scoped_join_handle = handles.pop().unwrap();
            // let partial_moves = scoped_join_handle.join().unwrap();

            let start = 0;
            let end = self.width * self.height;
            let closure = {
                let (start, end, slf) = (start, end, &self);
                move || slf.find_moves(start, end)
            };
            let handle = scope.spawn(closure);

            // let mut final_moves: FxHashMap<usize, Vec<usize>> = FxHashMap::default();
            let partial_moves = handle.join().unwrap();
            let mut final_moves = FxHashMap::default();

            for (to, froms) in partial_moves.iter() {
                for f in froms {
                    Self::add_move(&mut final_moves, *f, *to);
                }
            }

            println!("{}", final_moves.len());

            final_moves
        });

        self.apply_moves();
        // Print simulation informations.
        if self.print_debug {
            self.print_sim_info();
        }

        self.clear_moves();
    }
}

impl Simulation {
    /// Finds desired moves for each particle
    fn find_moves(&self, start: usize, end: usize) -> FxHashMap<usize, Vec<usize>> {
        let mut moves = FxHashMap::default();

        for i in start..end {
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
                            // Add the value to moves map
                            Self::add_move(&mut moves, i, new_index);
                            break;
                        }
                    }
                }
            }
        }

        moves
    }

    /// Adds a move to the moves map
    fn add_move(moves_map: &mut FxHashMap<usize, Vec<usize>>, from: usize, to: usize) -> () {
        if moves_map.contains_key(&to) {
            // Safe to unwrap as we checked for the key
            moves_map.get_mut(&to).unwrap().push(from);
        } else {
            moves_map.insert(to, vec![from]);
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
