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
