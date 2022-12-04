from typing import Tuple


def part1(input: list[str]) -> int:
    total = 0
    for team in input:
        elf1, elf2 = team.split(",")
        elf1_1, elf1_2 = elf1.split("-")
        elf2_1, elf2_2 = elf2.split("-")
        a, b = int(elf1_1), int(elf1_2) + 1
        c, d = int(elf2_1), int(elf2_2) + 1
        if all(x in range(a, b) for x in range(c, d)):
            total += 1
            continue
        elif all(x in range(c, d) for x in range(a, b)):
            total += 1
            continue
    return total


def part2(input: list[str]) -> int:
    total = 0
    for team in input:
        elf1, elf2 = team.split(",")
        elf1_1, elf1_2 = elf1.split("-")
        elf2_1, elf2_2 = elf2.split("-")
        a, b = int(elf1_1), int(elf1_2) + 1
        c, d = int(elf2_1), int(elf2_2) + 1
        if any(x in range(a, b) for x in range(c, d)):
            total += 1
            continue
        elif any(x in range(c, d) for x in range(a, b)):
            total += 1
            continue
    return total


def solve() -> Tuple[int, int]:
    input = open("../inputs/day04.txt").readlines()
    return (part1(input), part2(input))
