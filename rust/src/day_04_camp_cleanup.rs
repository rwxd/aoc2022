use std::ops::RangeInclusive;

use crate::etc::Solution;
use crate::etc::SolutionPair;
use crate::input_reader;


fn part1(input: Vec<Vec<RangeInclusive<i32>>>) -> i32 {
    let mut overlaps = 0;
    for team in input {
        if (team[0].contains(&team[1].start()) && team[0].contains(&team[1].end()))
            || (team[1].contains(&team[0].start()) && team[1].contains(&team[0].end())){
            overlaps += 1;
            continue;
        }
    }
    overlaps
}

fn part2(input: Vec<Vec<RangeInclusive<i32>>>) -> i32 {
    let mut overlaps = 0;
    for team in input {
        for section in team[0].clone() {
            if team[1].contains(&section) {
                overlaps += 1;
                break;
            }
        }
    }
    overlaps
}

fn get_ranges(input: &String) -> Vec<Vec<RangeInclusive<i32>>> {
    let mut ranges = Vec::new();
    for team in input.lines() {
        let mut team_range = Vec::new();
        for elf in team.split(",") {
            let mut sections = elf.split("-");
            let start = sections.next().unwrap().parse::<i32>().unwrap();
            let end = sections.next().unwrap().parse::<i32>().unwrap();
            team_range.push(start..=end);
        }
        ranges.push(team_range);
    }
    ranges
}

pub fn solve() -> SolutionPair {
    let input = input_reader::read_file_in_cwd("../inputs/day04.txt");
    let transformed = get_ranges(&input);
    let result_1 = part1(transformed.clone());
    let result_2 = part2(transformed);
    (Solution::I32(result_1), Solution::I32(result_2))
}
