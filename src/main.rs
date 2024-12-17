use day_16::{count_tiles, solve};

fn main() {
    let lowest_score = solve("src/resources/puzzle.txt");
    println!("Lowest score: {}", lowest_score);

    let tiles = count_tiles("src/resources/puzzle.txt");
    println!("Tiles: {}", tiles);
}

