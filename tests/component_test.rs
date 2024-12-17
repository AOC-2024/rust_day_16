use day_16::{solve, count_tiles};

#[test]
fn should_count_lowest_sore() {
    assert_eq!(solve("tests/resources/puzzle.txt"), 7036);
}

#[test]
fn should_count_tiles() {
    assert_eq!(count_tiles("tests/resources/puzzle.txt"), 45);
}
