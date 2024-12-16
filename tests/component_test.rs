use day_16::lowest_score;

#[test]
fn should_count_lowest_sore() {
    assert_eq!(lowest_score("tests/resources/puzzle.txt"), 7036);
}