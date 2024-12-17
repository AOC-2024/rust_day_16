use crate::Direction::{DOWN, LEFT, RIGHT, UP};
use std::fs::read_to_string;
use pathfinding::prelude::dijkstra;


pub fn solve(input_path: &str) -> isize {
    let puzzle = extract_puzzle(input_path);
    let start = Node {
        position: puzzle.start.clone(),
        direction: RIGHT,
    };
    let result = dijkstra(&start, |node| successors(node, &puzzle), |node| node.position == puzzle.end).expect("No path found");

    result.1
}

fn successors(node: &Node, puzzle: &Puzzle) -> Vec<(Node, isize)> {
    let Position(x, y) = node.position;

    let neighbors = Direction::neighbors(&node.direction);

    let mut next_nodes = Vec::new();
    for (new_direction, dx, dy) in neighbors {
        let new_x = x + dx;
        let new_y = y + dy;
        if puzzle.is_position_valid(new_x, new_y) {
            let cost = Direction::rotation_cost(&node.direction, &new_direction) + 1;
            next_nodes.push((
                Node {
                    position: Position(new_x, new_y),
                    direction: new_direction,
                },
                cost,
            ));
        }
    }

    next_nodes
}


#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    position: Position,
    direction: Direction,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position(isize, isize);

fn extract_puzzle(input_path: &str) -> Puzzle {
    let mut puzzle: Puzzle = Puzzle::new();
    let map = read_to_string(input_path)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    'S' => {
                        puzzle.start = Position(j as isize, i as isize);
                        c
                    }
                    'E' => {
                        puzzle.end = Position(j as isize, i as isize);
                        c
                    }
                    _ => c,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    puzzle.map = map;
    puzzle
}



#[derive(Debug, PartialEq)]
struct Puzzle {
    start: Position,
    end: Position,
    map: Vec<Vec<char>>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            start: Position(0, 0),
            end: Position(0, 0),
            map: Vec::new(),
        }
    }

    fn width(&self) -> isize {
        self.map[0].len() as isize
    }

    fn height(&self) -> isize {
        self.map.len() as isize
    }

    fn is_position_valid(&self, x: isize, y: isize) -> bool {
        if x < 0 || x >= self.height() || y < 0 || y >= self.width() {
            return false;
        }
        self.map[y as usize][x as usize] != '#'
    }

}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn val(&self) -> (Direction, isize, isize) {
        match *self {
            UP => (UP, 0, 1),
            DOWN => (DOWN, 0, -1),
            LEFT => (LEFT, -1, 0),
            RIGHT => (RIGHT, 1, 0),
        }
    }

    fn neighbors(direction: &Direction) -> Vec<(Direction, isize, isize)> {
        match direction {
            UP => vec![
                UP.val(),
                RIGHT.val(),
                LEFT.val(),
            ],
            RIGHT => vec![
                RIGHT.val(),
                UP.val(),
                DOWN.val(),
            ],
            DOWN => vec![
                DOWN.val(),
                RIGHT.val(),
                LEFT.val(),
            ],
            LEFT => vec![
                LEFT.val(),
                UP.val(),
                DOWN.val(),
            ],
        }
    }

    fn rotation_cost(from_direction: &Direction, to_direction: &Direction) -> isize {
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

