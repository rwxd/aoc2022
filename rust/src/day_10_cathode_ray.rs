use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::write;

use crate::etc::Solution;
use crate::etc::SolutionPair;
use crate::input_reader;

type Cycles = Vec<Instruction>;

enum Instruction {
    Noop,
    Addx(i32),
}

fn parse_cycles(input: &String) -> Cycles {
    let mut cycles: Cycles = Vec::new();
    for line in input.lines() {
        let items: Vec<&str> = line.split(" ").collect();
        if items.len() == 2 {
            let amount = items[1].parse::<i32>().unwrap();
            cycles.push(Instruction::Addx(amount));
        } else {
            cycles.push(Instruction::Noop);
        }
    }
    cycles
}

#[derive(Clone)]
struct RenderLine(Vec<bool>);

impl RenderLine {
    fn new(width: i32) -> Self {
        RenderLine(vec![false; width as usize])
    }
}

impl Display for RenderLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for pixel in &self.0 {
            if *pixel {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }
        Ok(())
    }
}

struct Screen(Vec<RenderLine>);

impl Screen {
    fn new() -> Self {
        Screen(vec![])
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in &self.0 {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

fn solution(cycles: &Cycles) -> i32{
    let register_checks = vec![20, 60, 100, 140, 180, 220];
    let mut signals: BTreeMap<i32, i32> = BTreeMap::new();
    let mut instruction_counter = 0;
    let mut register: Option<i32> = None;

    let screen_width = 40;
    let mut screen = Screen::new();
    let mut line = RenderLine::new(screen_width);

    let mut x = 1;
    for cycle in 1..i32::MAX {
        if register_checks.contains(&cycle) {
            signals.insert(cycle, x*cycle);
        }

        let screen_position = (cycle - 1) % screen_width;
        let pixel = screen_position == x -1 || screen_position == x || screen_position == x + 1;
        // println!("cycle: {}, x: {}, screen_position: {}, pixel: {}", cycle, x, screen_position, pixel);
        line.0[screen_position as usize] = pixel;

        if screen_position == screen_width - 1 {
            // println!("{}", line);
            screen.0.push(line);
            line = RenderLine::new(screen_width);
        }

        if let Some(val) = register {
            x += val;
            register = None;
            continue;
        }

        let instruction = cycles.get(instruction_counter);
        if instruction.is_none() {
            break;
        }

        match instruction.unwrap() {
            Instruction::Noop => {}
            Instruction::Addx(amount) => {
                register = Some(*amount);
            }
        }

        instruction_counter += 1;
    }
    println!("Signals: {:#?}", signals);
    println!("{}", screen);
    signals.iter().map(|(_, v)| v).sum::<i32>()
}

pub fn solve() -> SolutionPair {
    let input = input_reader::read_file_in_cwd("../inputs/day10.txt");
    let cycles = parse_cycles(&input);
    let result_1 = solution(&cycles);
    (Solution::I32(result_1), Solution::I32(0))
}
