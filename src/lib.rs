use crate::Direction::{DOWN, LEFT, RIGHT, UP};
use std::collections::VecDeque;
use std::fs::read_to_string;

pub fn lowest_score(input_path: &str) -> isize {
    let mut puzzle = extract_puzzle(input_path);
    puzzle.solve();
    puzzle.lowest_score()
}

fn extract_puzzle(input_path: &str) -> Puzzle {
    let mut puzzle: Puzzle = Puzzle::new();
    read_to_string(input_path)
        .unwrap()
        .lines()
        .into_iter()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars()
                .enumerate()
                .for_each(|(x, character)| match character {
                    '#' => puzzle.obstacles.push((x as isize, y as isize)),
                    'S' => puzzle.start = (x as isize, y as isize),
                    'E' => puzzle.end = (x as isize, y as isize),
                    _ => {}
                })
        });

    puzzle
}

#[derive(Debug, PartialEq)]
struct Puzzle {
    start: (isize, isize),
    end: (isize, isize),
    obstacles: Vec<(isize, isize)>,
    paths: Vec<Path>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            start: (0, 0),
            end: (0, 0),
            obstacles: Vec::new(),
            paths: Vec::new(),
        }
    }

    fn lowest_score(&self) -> isize {
        if self.paths.is_empty() {
            return 0;
        }
        let mut sorted_path = self.paths.clone();
        sorted_path.sort_by(|a, b| a.score.cmp(&b.score));
        sorted_path.first().unwrap().score
    }

    fn solve(&mut self) {
        let current_point = self.start.clone();
        let current_direction = RIGHT;

        let mut exploring_path: VecDeque<Exploration> = VecDeque::new();

        exploring_path.push_back(Exploration {
            init_position: current_point,
            init_direction: current_direction,
            prev_positions: vec![],
            prev_rotations: 0,
            prev_score: 0,
        });

        while let Some(exploration) = exploring_path.pop_front() {
            let next_free_spaces = self.find_free_space_points_around(exploration.init_position, exploration.prev_positions.clone());
            for (next_exploration, next_direction) in next_free_spaces {
                let rotation_cost = Direction::rotation_cost(exploration.init_direction, next_direction);
                let mut new_rotation_count = exploration.prev_rotations;
                if rotation_cost > 0 {
                    new_rotation_count += 1;
                }
                if next_exploration == self.end {
                    self.paths.push(Path {
                        points: exploration.prev_positions.clone(),
                        rotations: new_rotation_count,
                        score: exploration.prev_score + rotation_cost + 1,
                    });
                } else {
                    let mut new_positions = exploration.prev_positions.clone();
                    new_positions.push(next_exploration);
                    exploring_path.push_back(Exploration {
                        init_position: next_exploration,
                        init_direction: next_direction,
                        prev_positions: new_positions,
                        prev_rotations: exploration.prev_rotations,
                        prev_score: exploration.prev_score + rotation_cost + 1,
                    })
                }
            }
        }
    }

    fn find_free_space_points_around(
        &self,
        position: (isize, isize),
        prev_position: Vec<(isize, isize)>
    ) -> Vec<((isize, isize), Direction)> {
        let mut free_spaces = Vec::new();
        let next_position = UP.next_position(position);
        if !self.obstacles.contains(&next_position) && !prev_position.contains(&next_position) {
            free_spaces.push((next_position, UP));
        }

        let next_position = DOWN.next_position(position);
        if !self.obstacles.contains(&next_position) && !prev_position.contains(&next_position) {
            free_spaces.push((next_position, DOWN));
        }

        let next_position = RIGHT.next_position(position);
        if !self.obstacles.contains(&next_position) && !prev_position.contains(&next_position) {
            free_spaces.push((next_position, RIGHT));
        }

        let next_position = LEFT.next_position(position);
        if !self.obstacles.contains(&next_position) && !prev_position.contains(&next_position) {
            free_spaces.push((next_position, LEFT));
        }
        free_spaces
    }
}

struct Exploration {
    init_position: (isize, isize),
    init_direction: Direction,
    prev_positions: Vec<(isize, isize)>,
    prev_rotations: isize,
    prev_score: isize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn val(&self) -> (isize, isize) {
        match *self {
            UP => (0, -1),
            DOWN => (0, 1),
            LEFT => (-1, 0),
            RIGHT => (1, 0),
        }
    }

    fn next_position(&self, position: (isize, isize)) -> (isize, isize) {
        (position.0 + self.val().0, position.1 + self.val().1)
    }

    fn rotation_cost(from_direction: Direction, to_direction: Direction) -> isize {
        let rotation_price = 1000;
        match from_direction {
            UP => match to_direction {
                LEFT => rotation_price,
                RIGHT => rotation_price,
                DOWN => rotation_price * 2,
                UP => 0,
            },
            DOWN => match to_direction {
                LEFT => rotation_price,
                RIGHT => rotation_price,
                DOWN => 0,
                UP => rotation_price * 2,
            },
            RIGHT => match to_direction {
                LEFT => rotation_price * 2,
                RIGHT => 0,
                DOWN => rotation_price,
                UP => rotation_price,
            },
            LEFT => match to_direction {
                LEFT => 0,
                RIGHT => rotation_price * 2,
                DOWN => rotation_price,
                UP => rotation_price,
            },
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
struct Path {
    points: Vec<(isize, isize)>,
    rotations: isize,
    score: isize,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_puzzle_with_multiple_rotation() {
        let mut puzzle = extract_puzzle("tests/resources/multiple_rotation_cost_path.txt");
        puzzle.solve();
        assert_eq!(puzzle.lowest_score(), 3004);
    }

    #[test]
    fn should_solve_puzzle_with_one_rotation() {
        let mut puzzle = extract_puzzle("tests/resources/one_rotation_cost_path.txt");
        puzzle.solve();
        assert_eq!(puzzle.lowest_score(), 1003);
    }


    #[test]
    fn should_solve_puzzle_with_multiple_point_path_without_rotating() {
        let mut puzzle = extract_puzzle("tests/resources/two_point_path.txt");
        puzzle.solve();
        assert_eq!(puzzle.lowest_score(), 1004);
    }

    #[test]
    fn should_solve_puzzle_with_one_point_path() {
        let mut puzzle = extract_puzzle("tests/resources/one_point_path.txt");
        puzzle.solve();
        assert_eq!(puzzle.lowest_score(), 2);
    }

    #[test]
    fn should_lowest_score_based_on_score() {
        let mut puzzle = Puzzle::new();
        puzzle.paths.push(Path {
            points: vec![(0, 0), (0, 1)],
            rotations: 1,
            score: 1002,
        });
        puzzle.paths.push(Path {
            points: vec![(0, 0)],
            rotations: 1,
            score: 1001,
        });

        assert_eq!(puzzle.lowest_score(), 1001);
    }

    #[test]
    fn should_lowest_score_return_0_when_paths_enmpty() {
        let mut puzzle = Puzzle::new();

        assert_eq!(puzzle.lowest_score(), 0);
    }

    #[test]
    fn should_extract_puzzle() {
        assert_eq!(
            extract_puzzle("tests/resources/light_puzzle.txt"),
            Puzzle {
                start: (1, 1),
                end: (3, 0),
                obstacles: vec![(0, 0), (0, 1), (3, 1)],
                paths: vec![]
            }
        );
    }
}
