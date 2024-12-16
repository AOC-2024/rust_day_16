use day_16::lowest_score;

#[test]
fn should_compute_safety_factor() {
    assert_eq!(lowest_score("tests/resources/puzzle.txt"), 12);
}