use crate::etc::Solution;
use crate::etc::SolutionPair;
use crate::input_reader;


fn part1(rucksacks: Vec<&str>) -> i32 {
    let mut priority_items = 0;
    for rucksack in rucksacks {
        let (comp1, comp2) = rucksack.split_at(rucksack.len()/2);
        for c in comp1.chars() {
            if comp2.contains(c) {
                println!("{} => {}", c, char_to_priority(c));
                priority_items += char_to_priority(c);
                break
            }
        }
    }
    priority_items
}

fn part2(rucksacks: Vec<&str>) -> i32 {
    let mut priority_items = 0;
    for i in (0..rucksacks.len()).step_by(3) {
        for c in rucksacks[i].chars() {
            if rucksacks[i+1].contains(c) && rucksacks[i+2].contains(c) {
                priority_items += char_to_priority(c);
                break
            }
        }
    }
    priority_items
}

fn char_to_priority(item: char) -> i32 {
    let ascii = item as i32;
    // a - z
    if (97..123).contains(&ascii) {
        return ascii - 97 + 1;
    }
    // A - Z
    return ascii - 65 + 27;
}

pub fn solve() -> SolutionPair {
    let input = input_reader::read_file_in_cwd("src/day_03_input.txt");
    let rucksacks: Vec<&str> = input.lines().collect();
    let result_1 = part1(rucksacks.clone());
    let result_2 = part2(rucksacks);
    (Solution::I32(result_1), Solution::I32(result_2))
}
