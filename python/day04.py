from typing import Tuple


def part1(input: list[str]) -> int:
    total = 0
    for team in input:
        elf1, elf2 = team.split(",")
        elf1_1, elf1_2 = elf1.split("-")
        elf2_1, elf2_2 = elf2.split("-")
        a, b = int(elf1_1), int(elf1_2) + 1
        c, d = int(elf2_1), int(elf2_2) + 1
        if a >= c and b <= d:
            total += 1
        elif c >= a and d <= b:
            total += 1
    return total


def part2(input: list[str]) -> int:
    total = 0
    for team in input:
        elf1, elf2 = team.split(",")
        elf1_1, elf1_2 = elf1.split("-")
        elf2_1, elf2_2 = elf2.split("-")
        a, b = int(elf1_1), int(elf1_2)
        c, d = int(elf2_1), int(elf2_2)
        range1 = range(a, b + 1)
        range2 = range(c, d + 1)
        if a in range2 or b in range2 or c in range1 or d in range1:
            total += 1
    return total


def solve() -> Tuple[int, int]:
    input = open("../inputs/day04.txt").readlines()
    return (part1(input), part2(input))
