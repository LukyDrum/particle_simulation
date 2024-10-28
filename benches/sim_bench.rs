use criterion::{black_box, criterion_group, criterion_main, Criterion};

use particle_simulation::{
    particles::{Sand, Water},
    Offset, Simulation, Sprite,
};

fn sim200x200(steps: u32) -> () {
    // Create sim and load FIT sprite
    let mut sim = Simulation::new(200, 200);
    let fit_sprite = Sprite::load("assets/fit_pixel_blue.png");
    if let Ok(sprite) = fit_sprite {
        sim.insert_sprite(sprite, &Offset::new(80, 50), |color| match color {
            0xFFFFFFFF => Sand::new(),
            _ => Water::new(),
        });
    }

    // Let sim run for `steps` of steps
    for _ in 0..steps {
        sim.simulate_step();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Sim 200x200", |b| b.iter(|| sim200x200(black_box(1000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
