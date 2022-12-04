from typing import Tuple


def part1(input: list[int]) -> int:
    return max(input)


def part2(input: list[int]) -> int:
    sorted_input = sorted(input, reverse=True)
    return sum(sorted_input[0:3])


def transform_input(input: str) -> list[int]:
    output = []
    for elve in input.split('\n\n'):
        output.append(sum([int(x) for x in elve.split('\n') if x != '']))
    return output


def solve() -> Tuple[int, int]:
    input = open("../inputs/day01.txt").read()
    input = transform_input(input)
    return (part1(input), part2(input))
