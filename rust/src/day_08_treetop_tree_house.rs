use crate::etc::Solution;
use crate::etc::SolutionPair;
use crate::input_reader;

fn part1(map: &Map) -> i32{
    let map_height = map.len();
    let map_width = map[0].len();

    let mut score = 0;

    for y in 0..map_height{
        for x in 0..map_width{
            let tree_height = map[y][x] as usize;
            // println!("Tree height: {}", tree_height);

            let mut visible_left = true;
            let mut visible_right = true;
            let mut visible_top = true;
            let mut visible_bottom = true;

            // Check the same row
            for leftx in 0..x{
                let left_tree_height = map[y][leftx] as usize;
                if leftx != x && left_tree_height >= tree_height{
                    visible_left = false;
                    break;
                }
            }

            for rightx in x+1..map_width{
                let right_tree_height = map[y][rightx] as usize;
                if rightx != x && right_tree_height >= tree_height{
                    visible_right = false;
                    break;
                }
            }

            // Check the same column
            for topy in 0..y{
                let top_tree_height = map[topy][x] as usize;
                if topy != y && top_tree_height >= tree_height{
                    visible_top = false;
                    break;
                }
            }

            for bottomy in y+1..map_height{
                let bottom_tree_height = map[bottomy][x] as usize;
                if bottomy != y && bottom_tree_height >= tree_height{
                    visible_bottom = false;
                    break;
                }
            }


            if visible_left || visible_right || visible_top || visible_bottom {
                // println!("Tree at ({}, {}) is visible", x+1, y+1);
                score += 1;
            }
        }
    }
    score
}

fn part2(map: &Map) -> i32{
    let map_height = map.len();
    let map_width = map[0].len();

    let mut scenic_score = 0;

    for y in 0..map_height{
        for x in 0..map_width{
            let tree_height = map[y][x] as usize;
            // println!("Tree height: {}", tree_height);

            let mut visible_left = 0;
            for leftx in (0..x).rev(){
                visible_left += 1;
                let left_tree_height = map[y][leftx] as usize;
                if left_tree_height >= tree_height{
                    break;
                }
            }

            let mut visible_right = 0;
            for rightx in x+1..map_width{
                visible_right += 1;
                let right_tree_height = map[y][rightx] as usize;
                if right_tree_height >= tree_height{
                    break;
                }
            }

            let mut visible_top = 0;
            for topy in (0..y).rev(){
                visible_top += 1;
                let top_tree_height = map[topy][x] as usize;
                if top_tree_height >= tree_height{
                    break;
                }
            }

            let mut visible_bottom = 0;
            for bottomy in y+1..map_height{
                visible_bottom += 1;
                let bottom_tree_height = map[bottomy][x] as usize;
                if bottom_tree_height >= tree_height{
                    break;
                }
            }

            let current_scenic_score = visible_left * visible_right * visible_top * visible_bottom;
            if current_scenic_score > scenic_score {
                scenic_score = current_scenic_score;
            }
        }
    }
    scenic_score
}

type Map = Vec<Vec<u32>>;

fn build_map(input: &String) -> Map {
    let mut map = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            let height = c.to_digit(10).unwrap();
            row.push(height);
        }
        map.push(row);
    }
    map
}

pub fn solve() -> SolutionPair {
    let input = input_reader::read_file_in_cwd("../inputs/day08.txt");
    let map = build_map(&input);
    let result_1 = part1(&map.clone());
    let result_2 = part2(&map.clone());
    (Solution::I32(result_1), Solution::I32(result_2))
}
