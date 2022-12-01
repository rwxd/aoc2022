use crate::input_reader;

pub fn part1(input: Vec<i32>) -> i32 {
    *input.iter().max().unwrap()
}

pub fn part2(input: Vec<i32>) -> i32 {
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

pub fn run() {
    let input = input_reader::read_file_in_cwd("src/day_01_input.txt");
    println!("==> Loaded input");
    let transformed_input = get_elf_calories(input);
    let input_summed = get_elf_calories_summed(transformed_input);
    println!("==> Transformed input");
    let elf_with_most_calories = part1(input_summed.clone());
    println!("Elf with most calories: {:?}", elf_with_most_calories);
    let sum_top3_elfs = part2(input_summed);
    println!("Sum of top 3 elfs, with most calories: {:?}", sum_top3_elfs);
}
