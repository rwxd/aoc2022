from typing import Tuple


def part1(input: list[str]) -> int:
    total = 0
    for team in input:
        elf1, elf2 = team.split(",")
        elf1_1, elf1_2 = elf1.split("-")
        elf2_1, elf2_2 = elf2.split("-")
        a, b = int(elf1_1), int(elf1_2) + 1
        c, d = int(elf2_1), int(elf2_2) + 1
        range1 = range(a, b)
        range2 = range(c, d)
        if (a in range2 and b in range2) or (c in range1 and d in range1):
            total += 1
    return total


def part2(input: list[str]) -> int:
    total = 0
    for team in input:
        elf1, elf2 = team.split(",")
        elf1_1, elf1_2 = elf1.split("-")
        elf2_1, elf2_2 = elf2.split("-")
        a, b = int(elf1_1), int(elf1_2) + 1
        c, d = int(elf2_1), int(elf2_2) + 1
        range1 = range(a, b)
        range2 = range(c, d)
        if (a in range2 or b in range2) or (c in range1 or d in range1):
            total += 1
    return total


def solve() -> Tuple[int, int]:
    input = open("../inputs/day04.txt").readlines()
    return (part1(input), part2(input))
