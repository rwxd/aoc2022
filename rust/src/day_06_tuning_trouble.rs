use crate::etc::Solution;
use crate::etc::SolutionPair;
use crate::input_reader;

fn part1(input: &String) -> i32{
    check_sequence(input, 4)
}

fn part2(input: &String) -> i32{
    check_sequence(input, 14)
}

fn check_sequence(sequence: &String, length: i32) -> i32 {
    let bytes = sequence.as_bytes();
    let bytes_count = bytes.len();

    println!("Checking sequences of length {}", length);
    let mut i = 0;
    'outer: while i < (bytes_count - length as usize){
        for c1 in 0..length - 1 {
            for c2 in c1 + 1..length {
                if bytes[i + c1 as usize] == bytes[i + c2 as usize] {
                    println!("Found match at index {} and index {}", i + c1 as usize, i + c2 as usize);
                    i += 1;
                    continue 'outer;
                }
            }
        }
        break
    }
    i as i32 + length
}


pub fn solve() -> SolutionPair {
    let input = input_reader::read_file_in_cwd("../inputs/day06.txt");
    let result_1 = part1(&input);
    let result_2 = part2(&input);
    (Solution::I32(result_1), Solution::I32(result_2))
}
