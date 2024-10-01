use crate::{
    frame::Frame,
    offset::Offset,
    particle::{Acidity, Burnability, Particle},
};

pub enum SimMove {
    None,
    MoveTo,
    SwitchWith(Particle),
    Replace,
}

pub struct Simulation {
    width: usize,
    height: usize,
    bg_color: u32,
    particles: Vec<Vec<Option<Particle>>>,
}

impl Simulation {
    pub fn new(width: usize, height: usize) -> Simulation {
        Simulation {
            width,
            height,
            bg_color: 0x00000000,
            particles: vec![vec![None; width]; height],
        }
    }

    pub fn draw_to_frame(&self, frame: &mut Frame) -> () {
        for y in 0..self.height {
            for x in 0..self.width {
                let opt = &self.particles[y][x];

                let color = match opt {
                    Some(p) => p.color,
                    None => self.bg_color,
                };

                let _ = frame.draw_pixel(x, y, color);
            }
        }
    }

    pub fn add_particle(&mut self, offset: Offset, particle: Particle) -> bool {
        if !self.is_within(&offset) {
            return false;
        }

        if self.particles[offset.y as usize][offset.x as usize].is_none() {
            self.particles[offset.y as usize][offset.x as usize] = Some(particle);

            return true;
        }

        false
    }

    pub fn remove_particle(&mut self, offset: Offset) -> bool {
        if !self.is_within(&offset) {
            return false;
        }

        let opt = self.particles[offset.y as usize][offset.x as usize];

        match opt {
            None => return false,
            Some(_) => {
                self.particles[offset.y as usize][offset.x as usize] = None;

                return true;
            }
        }
    }

    pub fn get_particle(&self, offset: &Offset) -> &Option<Particle> {
        if !self.is_within(&offset) {
            return &None;
        }

        &self.particles[offset.y as usize][offset.x as usize]
    }

    pub fn change_particle(&mut self, offset: &Offset, new_particle: &Particle) -> () {
        if !self.is_within(&offset) {
            return;
        }

        self.particles[offset.y as usize][offset.x as usize] = Some(*new_particle);
    }

    pub fn simulate_step(&mut self) -> () {
        // Iterate through the grid
        for y in 0..self.height {
            for x in 0..self.width {
                // Get particle at that position
                let opt = self.particles[y][x];

                match opt {
                    // No particle there => continue
                    None => continue,
                    // Some particle there => proccess it
                    Some(p) => {
                        let mut particle = p;

                        // Skip if it was already updated
                        if particle.was_update {
                            continue;
                        }

                        // Check if the particle is moveable
                        if !particle.is_moveable {
                            continue; // There is no need to set is_updated
                        }

                        // Get a vec of offsets to which the particle would like to move to in order of importance
                        let offsets_to_try = particle.get_offsets_to_try();

                        // Try each offset
                        let mut made_move = false;
                        for offset in offsets_to_try {
                            // Get the new position after applying that offset
                            let new_x = x as i32 + offset.x;
                            let new_y = y as i32 + offset.y;
                            let new_pos = Offset::new(new_x, new_y);

                            // Get the possible move for that offset
                            let sim_move = self.try_offset(&new_pos, &particle);

                            // Check what actions you should do based on the SimMove
                            let mut should_break = false;
                            match sim_move {
                                // No action should be taken, continue searching
                                SimMove::None => {}
                                // Move to a free spot or replace the particle there
                                SimMove::MoveTo | SimMove::Replace => {
                                    // Set particle as updated
                                    particle.was_update = true;
                                    // Set the old spot as free
                                    self.particles[y][x] = None;
                                    // Set the new spot as occupied by the current particle
                                    self.particles[new_pos.y as usize][new_pos.x as usize] =
                                        Some(particle);
                                    // Set the flag that a move has been made
                                    made_move = true;
                                    // Exit the loop
                                    should_break = true;
                                }
                                // Switch yourself with the other particle; both particles should be set as updated after this step
                                SimMove::SwitchWith(other_particle) => {
                                    let mut other_particle = other_particle;
                                    // Set both particles as updated
                                    particle.was_update = true;
                                    other_particle.was_update = true;

                                    // Switch them in grid
                                    self.particles[new_pos.y as usize][new_pos.x as usize] =
                                        Some(particle);
                                    self.particles[y][x] = Some(other_particle);

                                    // Set the move flag and exit loop
                                    made_move = true;
                                    should_break = true;
                                }
                            }

                            if should_break {
                                // Handle if the particle is burning
                                if particle.burnability == Burnability::IsBurning {
                                    self.handle_burning(new_pos);
                                }

                                break;
                            }
                        }

                        // If no move was made = particle stayed in place, set it as updated
                        if !made_move {
                            particle.was_update = true;
                            self.particles[y][x] = Some(particle);

                            // Handle if the particle is burning
                            if particle.burnability == Burnability::IsBurning {
                                self.handle_burning(Offset::new(x as i32, y as i32));
                            }
                        }
                    }
                }
            }
        }

        // Reset was_updated on each particle
        for y in 0..self.height {
            for x in 0..self.width {
                let opt = &mut self.particles[y][x];

                match opt {
                    Some(p) => p.was_update = false,
                    None => {}
                }
            }
        }
    }
}

impl Simulation {
    fn try_offset(&self, offset: &Offset, particle: &Particle) -> SimMove {
        if self.is_within(offset) {
            let on_offset = self.particles[offset.y as usize][offset.x as usize];

            match on_offset {
                Some(other_particle) => {
                    // Replace the other as if by dissolving it.
                    if particle.acidity == Acidity::IsAcid
                        && other_particle.acidity == Acidity::DoesDissolve
                    {
                        return SimMove::Replace;
                    }
                    // Switch with accordance to density.
                    else if other_particle.density < particle.density {
                        return SimMove::SwitchWith(other_particle);
                    } else {
                        return SimMove::None;
                    }
                }
                // If there is no particle just move there.
                None => {
                    return SimMove::MoveTo;
                }
            }
        }

        SimMove::None
    }

    /// Destroy the burning particle and spread the burning to neighbors
    fn handle_burning(&mut self, offset: Offset) {
        // Spread burn to all neighbors and set them as updated; +2 because the end is exlusive
        for x in (offset.x - 1)..(offset.x + 2) {
            for y in (offset.y - 1)..(offset.y + 2) {
                if !self.is_within(&Offset::new(x, y)) {
                    continue;
                }

                let opt = &mut self.particles[y as usize][x as usize];
                match opt {
                    None => {}
                    Some(p) => {
                        // Skip particles that cannot burn
                        if p.burnability != Burnability::DoesBurn {
                            continue;
                        }

                        // Create a new fire particle and set it as updated
                        let mut fire_particle = Particle::fire();
                        fire_particle.was_update = true;
                        // Make it inherit the durability value from the original particle
                        fire_particle.durability = p.durability;
                        // Inherit offsets
                        fire_particle.primary_offset = p.primary_offset;
                        fire_particle.secondary_offsets = p.secondary_offsets;
                        fire_particle.ternary_offsets = p.ternary_offsets;

                        self.particles[y as usize][x as usize] = Some(fire_particle);
                    }
                }
            }
        }

        // Decrease durability - safe to unwrap, it is guaranteed the particle is here
        let mut p = self.particles[offset.y as usize][offset.x as usize].unwrap();
        p.durability -= 1;
        p.was_update = true;

        // If durability drops to 0 destroy the old particle
        if p.durability <= 0 {
            self.particles[offset.y as usize][offset.x as usize] = None;
        } else {
            self.particles[offset.y as usize][offset.x as usize] = Some(p);
        }
    }

    fn is_within(&self, offset: &Offset) -> bool {
        offset.x >= 0
            && offset.y >= 0
            && offset.x < self.width as i32
            && offset.y < self.height as i32
    }
}
