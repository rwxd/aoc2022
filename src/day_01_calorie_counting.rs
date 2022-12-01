use crate::input_reader;

pub fn part1(input: Vec<Vec<i32>>) -> i32 {
    let mut top_elf = 0;
    for elf in input {
        let mut elf_calories = 0;
        for calories in elf {
            elf_calories += calories
        }
        if elf_calories > top_elf {
            top_elf = elf_calories;
        }
    }
    top_elf
}

pub fn part2(input: Vec<Vec<i32>>) -> i32 {
    let mut elfs_total: Vec<i32> = input
        .iter()
        .map(|single_elf| {
            let mut total_calories_elf = 0;
            for i in single_elf {
                total_calories_elf += i;
            }
            total_calories_elf
        })
        .collect();

    elfs_total.sort();
    elfs_total.reverse();

    elfs_total[0] + elfs_total[1] + elfs_total[2]
}

fn get_elf_calories(input: String) -> Vec<Vec<i32>> {
    // Vector of string for each elf with its calories
    let elf_calories_str: Vec<&str> = input.split("\n\n").collect();

    // Vector with vectors of calories for each elf
    let elf_calories: Vec<Vec<i32>> = elf_calories_str
        .iter()
        .map(|&single_elf| {
            let calories_str: Vec<&str> =
                single_elf.split('\n').filter(|&x| !x.is_empty()).collect();
            let calories_int: Vec<i32> = calories_str
                .iter()
                .map(|&x| x.parse::<i32>().unwrap())
                .collect();
            calories_int
        })
        .collect();
    elf_calories
}

pub fn run() {
    let input = input_reader::read_file_in_cwd("src/day_01_input.txt");
    println!("==> Loaded input");
    let transformed_input = get_elf_calories(input);
    println!("==> Transformed input");
    let elf_with_most_calories = part1(transformed_input.clone());
    println!("Elf with most calories: {:?}", elf_with_most_calories);
    println!("-----------------------------");
    let sum_top3_elfs = part2(transformed_input);
    println!("Sum of top 3 elfs, with most calories: {:?}", sum_top3_elfs);
    println!("---------------------------------------------");
}
