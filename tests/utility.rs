use particle_simulation::utility::*;

#[test]
fn value_around() -> () {
    let middle = 42;
    let radius = 10;

    let res = get_value_around(middle, radius);

    assert!(res >= (middle - radius) && res <= (middle + radius));
}

#[test]
fn for_else_early() -> () {
    let mut early_flag = false;

    for_else!(
        for x in 1..10 => {
            if x == 6 {
                break;
            }
        } else {
            early_flag = true;
        }
    );

    assert!(!early_flag);
}

#[test]
fn for_else_no_break() -> () {
    let mut early_flag = false;

    // The case for break should never happen
    for_else!(
        for x in 1..10 => {
            if x == 100000 {
                break;
            }
        } else {
            early_flag = true;
        }
    );

    assert!(early_flag);
}
