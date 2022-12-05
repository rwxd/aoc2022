use crate::etc::Solution;
use crate::etc::SolutionPair;
use crate::input_reader;

#[derive(Debug, Clone)]
struct Move {
    count: u32,
    from: u32,
    to: u32,
}

type Moves = Vec<Move>;
type SupplyStacks = Vec<Vec<char>>;

fn part1(stacks: &mut SupplyStacks, moves: Moves) -> String {
    for mv in moves {
        for _ in 0..mv.count {
            let item = stacks[mv.from as usize -1].pop().unwrap();
            stacks[mv.to as usize -1].push(item);
        }
    }

    let result = stacks.iter().map(|x| x.last().unwrap()).collect::<String>();
    result
}

fn part2(stacks: &mut SupplyStacks, moves: Moves) -> String {
    for mv in moves {
        let pick_end = stacks[mv.from as usize -1].len();
        let pick_start = pick_end - mv.count as usize;
        let pick_range = pick_start..pick_end;
        let picked: Vec<char> = stacks[mv.from as usize -1].drain(pick_range).collect();
        stacks[mv.to as usize -1].append(&mut picked.clone());
    }

    let result = stacks.iter().map(|x| x.last().unwrap()).collect::<String>();
    result
}

fn get_stacks(input: &String) -> SupplyStacks {
    let (harbor, _) = input.split_once("\n\n").unwrap();
    let harbor_rows: Vec<Vec<char>> = harbor.lines().map(|l| l.chars().collect()).collect();
    let stack_identifier_row = harbor_rows.last().unwrap();

    let mut stacks: SupplyStacks = Vec::new();
    // Reverse to get the stacks in the right order
    for i in (0..harbor_rows.len() -1 ).rev()  {
        for (column, ch) in harbor_rows[i].iter().enumerate() {
            if ch.is_alphabetic() {
                let stack_number = stack_identifier_row[column].to_digit(10).unwrap();
                if stack_number > stacks.len() as u32 {
                    stacks.insert(stack_number as usize - 1, vec![*ch]);
                } else {
                    stacks[stack_number as usize -1].push(*ch);
                }
            }
        }
    }
    stacks
}

fn get_moves(input: &String) -> Moves {
    let (_, moves) = input.split_once("\n\n").unwrap();
    moves.lines().map(|l| {
        let mut parts = l.split_whitespace();
        let count = parts.nth(1).unwrap().parse::<u32>().unwrap();
        let from = parts.nth(1).unwrap().parse::<u32>().unwrap();
        let to = parts.nth(1).unwrap().parse::<u32>().unwrap();
        Move { count, from, to }
    }).collect()
}

pub fn solve() -> SolutionPair {
    let input = input_reader::read_file_in_cwd("../inputs/day05.txt");
    let stacks = get_stacks(&input);
    let moves = get_moves(&input);
    let result_1 = part1(&mut stacks.clone(), moves.clone());
    let result_2 = part2(&mut stacks.clone(), moves.clone());
    (Solution::Str(result_1), Solution::Str(result_2))
}
