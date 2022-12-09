// Heavily stolen from https://fasterthanli.me/series/advent-of-code-2022/part-7

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::etc::Solution;
use crate::etc::SolutionPair;
use crate::input_reader;
use camino::Utf8PathBuf;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(i)
}

#[derive(Debug)]
struct Ls;

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug)]
struct Cd(Utf8PathBuf);

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(Utf8PathBuf),
}


#[derive(Debug)]
enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );
    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

    alt((parse_file, parse_dir))(i)
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}


impl From<Ls> for Command {
    fn from(_ls: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}


fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}


fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
}

type NodeHandle = Rc<RefCell<Node>>;

#[derive(Default)]
struct Node {
    size: usize,
    children: HashMap<Utf8PathBuf, NodeHandle>,
    parent: Option<NodeHandle>,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("size", &self.size)
            .field("children", &self.children)
            // .field("parent", &self.parent)
            .finish()
    }
}

impl Node {
    fn is_dir(&self) -> bool {
        self.size == 0 && !self.children.is_empty()
    }

    fn total_size(&self) -> u64 {
        self.size as u64 + self.children.values().map(|child| child.borrow().total_size()).sum::<u64>()
    }
}


fn all_dirs(n: NodeHandle) -> Box<dyn Iterator<Item = NodeHandle>> {
    // clippy is wrong and should feel bad
    #[allow(clippy::needless_collect)]
    let children = n.borrow().children.values().cloned().collect::<Vec<_>>();

    Box::new(
        std::iter::once(n).chain(
            children
                .into_iter()
                .filter_map(|c| {
                    if c.borrow().is_dir() {
                        Some(all_dirs(c))
                    } else {
                        None
                    }
                })
                .flatten(),
        ),
    )
}


fn build_tree(input: &String) {
    let lines = input.lines().map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    let root = Rc::new(RefCell::new(Node::default()));
    let mut node = root.clone();

    for line in lines {
        // println!("{:?}", line);
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => {
                    // ignoring ls
                }
                Command::Cd(path) => match path.as_str() {
                    "/" => {
                        // ignoring
                    }
                    ".." => {
                        let parent = node.borrow().parent.clone().unwrap();
                        node = parent;
                    }
                    _ => {
                        let child = node.borrow_mut().children.entry(path).or_default().clone();
                        node = child;
                    }
                    // TODO: implement
                    // todo!();
                }
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(dir) => {
                    let entry = node.borrow_mut().children.entry(dir).or_default().clone();
                    entry.borrow_mut().parent = Some(node.clone());
                }
                Entry::File(size, file) => {
                    let entry = node.borrow_mut().children.entry(file).or_default().clone();
                    entry.borrow_mut().size = size as usize;
                    entry.borrow_mut().parent = Some(node.clone());
                }
            },
        }
    }
    println!("{:#?}", root);

    // let sum = all_dirs(root)
    //     .map(|d| d.borrow().total_size())
    //     .filter(|&s| s <= 100_000)
    //     .inspect(|s| {
    //         dbg!(s);
    //     }).sum::<u64>();
    // dbg!(sum);

    let total_space = 70000000_u64;
    let used_space = root.borrow().total_size();
    let free_space = total_space.checked_sub(dbg!(used_space)).unwrap();
    let needed_free_space = 30000000_u64;
    let minimum_space_to_free = needed_free_space.checked_sub(free_space).unwrap();

    let removed_dir_size = all_dirs(root)
        .map(|d| d.borrow().total_size())
        .filter(|&s| s >= minimum_space_to_free)
        .inspect(|s| {
            dbg!(s);
        })
        .min();
    dbg!(removed_dir_size);

}


fn part1(input: &String) -> i32{
    0
}


fn part2(input: &String) -> i32{
    0
}


pub fn solve() -> SolutionPair {
    let input = input_reader::read_file_in_cwd("../inputs/day07.txt");
    let tree = build_tree(&input);
    let result_1 = part1(&input);
    let result_2 = part2(&input);
    (Solution::I32(result_1), Solution::I32(result_2))
}
