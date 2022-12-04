from typing import Tuple


def part1(input: list[int]) -> int:
    return 0


def part2(input: list[int]) -> int:
    return 0


def transform_input(input: str) -> list[int]:
    return []


def solve() -> Tuple[int, int]:
    input = open("../inputs/day01.txt").read()
    input = transform_input(input)
    return (part1(input), part2(input))
