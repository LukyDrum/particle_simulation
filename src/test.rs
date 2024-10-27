#[cfg(test)]
mod offset_test {
    use crate::Offset;

    #[test]
    fn between_left() -> () {
        // Setup
        let og_pos = Offset::new(10, 10);
        let max_pos = Offset::new(5, 10); // 5 left, 5 down

        // Tested method
        let between = og_pos.between(&max_pos);

        // Check
        assert_eq!(
            between,
            vec![
                og_pos,
                Offset::new(9, 10),
                Offset::new(8, 10),
                Offset::new(7, 10),
                Offset::new(6, 10),
                max_pos
            ]
        );
    }

    #[test]
    fn between_left_down() -> () {
        // Setup
        let og_pos = Offset::new(10, 10);
        let max_pos = Offset::new(5, 15); // 5 left, 5 down

        // Tested method
        let between = og_pos.between(&max_pos);

        // Check
        assert_eq!(
            between,
            vec![
                og_pos,
                Offset::new(9, 11),
                Offset::new(8, 12),
                Offset::new(7, 13),
                Offset::new(6, 14),
                max_pos
            ]
        );
    }

    #[test]
    fn between_right_down() -> () {
        // Setup
        let og_pos = Offset::new(10, 10);
        let max_pos = Offset::new(15, 15); // 5 right, 5 down

        // Tested method
        let between = og_pos.between(&max_pos);

        // Check
        assert_eq!(
            between,
            vec![
                og_pos,
                Offset::new(11, 11),
                Offset::new(12, 12),
                Offset::new(13, 13),
                Offset::new(14, 14),
                max_pos
            ]
        );
    }
}

#[cfg(test)]
mod sprite_test {
    use crate::sprite::Sprite;

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
}

#[cfg(test)]
mod utility_test {
    use crate::utility::*;

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

        assert!(early_flag);
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

        assert!(!early_flag);
    }
}
