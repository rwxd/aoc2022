mod day_01_calorie_counting;
mod day_02_rock_paper_scissors;
mod day_03_rucksack_reorganization;
mod day_04_camp_cleanup;
mod day_05_supply_stacks;
mod day_06_tuning_trouble;
mod day_07_no_space_left_on_device;
mod day_08_treetop_tree_house;
mod day_09_rope_bridge;
mod day_10_cathode_ray;
mod etc;
mod input_reader;

use crate::etc::SolutionPair;
use std::time::Instant;

fn main() {
    let day: String = std::env::args()
        .nth(1)
        .expect("No day given. Possible options are: 01-25.");
    let day_int = day.parse::<i32>().unwrap();

    let mut runtime = 0.0;
    let func = get_day_solver(day_int);
    let time = Instant::now();
    let (p1, p2) = func();
    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    runtime += elapsed_ms;
    println!("=== Day {:02} ===", day_int);
    println!("  · Part 1: {}", p1);
    println!("  · Part 2: {}", p2);
    println!("------------------------------");
    println!("Total runtime: {:.4} ms", runtime);
}

fn get_day_solver(day: i32) -> fn() -> SolutionPair {
    match day {
        1 => day_01_calorie_counting::solve,
        2 => day_02_rock_paper_scissors::solve,
        3 => day_03_rucksack_reorganization::solve,
        4 => day_04_camp_cleanup::solve,
        5 => day_05_supply_stacks::solve,
        6 => day_06_tuning_trouble::solve,
        7 => day_07_no_space_left_on_device::solve,
        8 => day_08_treetop_tree_house::solve,
        9 => day_09_rope_bridge::solve,
        10 => day_10_cathode_ray::solve,
        _ => unimplemented!(),
    }
}
