use std::collections::HashSet;

use crate::etc::Solution;
use crate::etc::SolutionPair;
use crate::input_reader;


#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Move {
    direction: Direction,
}


type Moves = Vec<Move>;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Rope = Vec<Position>;

impl From<&str> for Direction {
    fn from(v: &str) -> Self {
        match v {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("invalid direction"),
        }
    }
}

fn cartesian_product<T: Clone>(a: &[T], b: &[T]) -> Vec<(T, T)> {
    let mut result = Vec::new();
    for x in a {
        for y in b {
            result.push((x.clone(), y.clone()));
        }
    }
    result
}

fn parse_moves(input: &String) -> Moves {
    let mut moves: Moves = Vec::new();
    for line in input.lines() {
        let (dir, steps) = line.split_once(" ").unwrap();
        let dir = Direction::from(dir);
        let steps = steps.parse::<i32>().unwrap();
        for _ in 0..steps {
            moves.push(Move { direction: dir });
        }
    }
    moves
}

fn part1(moves: &Moves) -> i32{
    rope_calculator(moves, 2)
}

fn part2(moves: &Moves) -> i32 {
    rope_calculator(moves, 10)
}

fn rope_calculator(moves: &Moves, length: usize) -> i32{
    let mut rope: Rope = vec![Position { x: 0, y: 0 }; length];
    let mut head_position = rope.first().unwrap().clone();
    let mut known_tail_positions: HashSet<Position> = HashSet::from([rope.last().unwrap().clone()]);
    println!("Rope has {} knots", rope.len());

    println!("Analyzing {} moves", moves.len());
    for mv in moves {
        // println!("Moving in the direction {:?}", mv.direction);
        match mv.direction {
            Direction::Up => head_position.y += 1,
            Direction::Down => head_position.y -= 1,
            Direction::Right => head_position.x += 1,
            Direction::Left => head_position.x -= 1,
        }
        rope[0] = head_position.clone();

        // skipping head
        for knotidx in 1..rope.len() {
            // println!("Checking knot {}", knot);

            let mut knot = rope[knotidx].clone();
            let previous_knot = rope[knotidx - 1].clone();

            // 9x9 Grid points to check if tail is adjacent
            let y_range: Vec<i32> = ((previous_knot.y -1)..=(previous_knot.y + 1)).collect();
            let x_range: Vec<i32> = ((previous_knot.x -1)..=(previous_knot.x + 1)).collect();

            // Check if knot is in the grid
            let rope_connected = cartesian_product(&y_range, &x_range).iter()
                .any(|(y, x)| {
                    let pos = Position { x: *x, y: *y };
                    pos == knot
                });

            if !rope_connected {
                // println!("Knot disconnected from leader");
                // println!("Leader position: {:?}", leader_position);
                // println!("Knot position: {:?}", knot_position);

                // Knot is not in the same row AND column as the previous knot
                // Move x & y axis
                if previous_knot.x != knot.x && previous_knot.y != knot.y {
                    knot.x += correct_move(previous_knot.x - knot.x);
                    knot.y += correct_move(previous_knot.y - knot.y);
                }
                // Move knot y axis
                else if previous_knot.y != knot.y {
                    knot.y += correct_move(previous_knot.y - knot.y);
                }
                // Move knot x axis
                else if previous_knot.x != knot.x {
                    knot.x += correct_move(previous_knot.x - knot.x);
                }

                // println!("New knot position: {:?}", knot);
                rope[knotidx] = knot.clone();

                if knotidx == rope.len() -1 {
                    // println!("Adding new tail position: {:?}", knot);
                    known_tail_positions.insert(knot.clone());
                }
            }
        }
        println!("Rope: {:#?}", rope);
    }
    known_tail_positions.len() as i32
}

fn correct_move(input: i32) -> i32 {
    if input > 0 {
        1
    } else if input < 0 {
        -1
    } else {
        0
    }
}

pub fn solve() -> SolutionPair {
    let input = input_reader::read_file_in_cwd("../inputs/day09.txt");
    let moves = parse_moves(&input);
    let result_1 = part1(&moves);
    let result_2 = part2(&moves);
    (Solution::I32(result_1), Solution::I32(result_2))
}
