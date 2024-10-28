use particle_simulation::Sprite;

#[test]
fn load_sprite() -> () {
    let sprite = Sprite::load("./assets/fit_pixel_blue.png");

    assert!(sprite.is_ok());
}

#[test]
fn failed_load_sprite() -> () {
    let sprite = Sprite::load("./assets/this_doesnt_exists.png");

    assert!(sprite.is_err());
}

#[test]
fn unique_colors() -> () {
    // Setup
    let sprite = Sprite::load("./assets/fit_pixel_blue.png");

    // Tested
    let colors = sprite.unwrap().get_unique_colors();

    // Test length
    assert_eq!(colors.len(), 2);
    // Test colors
    assert!(colors.contains(&(0xFF0070BA as u32)));
    assert!(colors.contains(&(0xFFFFFFFF as u32)));
}
