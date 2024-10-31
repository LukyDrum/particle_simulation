use particle_simulation::{particles::Sand, Offset, Simulation};

const WIDTH: usize = 200;
const HEIGHT: usize = 200;
const STEPS: u32 = 1000;

fn main() {
    let mut sim = Simulation::new(WIDTH, HEIGHT);

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            if (x + y) % 3 == 0 {
                sim.add_particle(&Offset::new(x as i32, y as i32), Sand::new());
            }
        }
    }

    let mut step = 0;
    while step < STEPS {
        sim.simulate_step();

        step += 1;
    }
}
