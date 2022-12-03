use crate::etc::Solution;
use crate::etc::SolutionPair;
use crate::input_reader;

pub fn part1(input: &[i32]) -> i32 {
    *input.iter().max().unwrap()
}

pub fn part2(input: &[i32]) -> i32 {
    input.iter().take(3).sum()
}

fn get_elf_calories(input: String) -> Vec<Vec<i32>> {
    //! Returns vector of elfs with correspondent calories in a vector.
    let elf_calories_str: Vec<&str> = input.split("\n\n").collect();
    let elf_calories: Vec<Vec<i32>> = elf_calories_str
        .iter()
        .map(|&single_elf| {
            let calories: Vec<i32> = single_elf
                .split('\n')
                .filter(|&x| !x.is_empty())
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            calories
        })
        .collect();
    elf_calories
}

fn get_elf_calories_summed(elfs: Vec<Vec<i32>>) -> Vec<i32> {
    //! Returns vector of summed calories for each elf.
    //! It is sorted highest to lowest.
    let mut elfs_total: Vec<i32> = elfs
        .iter()
        .map(|single_elf| single_elf.iter().sum())
        .collect();

    elfs_total.sort();
    elfs_total.reverse();
    elfs_total
}

pub fn solve() -> SolutionPair {
    let input = input_reader::read_file_in_cwd("src/day_01_input.txt");
    let input_summed = get_elf_calories_summed(get_elf_calories(input));
    let elf_with_most_calories = part1(&input_summed);
    let sum_top3_elfs = part2(&input_summed);
    (
        Solution::I32(elf_with_most_calories),
        Solution::I32(sum_top3_elfs),
    )
}
