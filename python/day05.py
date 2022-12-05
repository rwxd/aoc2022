from typing import Tuple
from dataclasses import dataclass
from collections import defaultdict
from typing import DefaultDict
from copy import deepcopy


@dataclass
class Move:
    count: int
    start: int
    to: int


Moves = list[Move]
SupplyStacks = DefaultDict[int, list[str]]


def part1(stacks: SupplyStacks, moves: Moves) -> str:
    for mv in moves:
        for _ in range(0, mv.count):
            stacks[mv.to].append(stacks[mv.start].pop())
    return "".join([x[-1] for x in stacks.values()])


def part2(stacks: SupplyStacks, moves: Moves) -> str:
    for mv in moves:
        stacks[mv.to].extend(
            reversed(list(stacks[mv.start].pop() for _ in range(0, mv.count)))
        )
    return "".join([x[-1] for x in stacks.values()])


def get_supply_stacks(input: str) -> SupplyStacks:
    harbor, _ = input.split('\n\n')
    harbor_rows = harbor.splitlines()
    stack_identifier_row = harbor_rows[-1]

    stacks = defaultdict(list)
    for i in reversed(range(0, len(harbor_rows) - 1)):
        for idx, c in enumerate(harbor_rows[i]):
            if c.lower() in 'abcdefghijklmnopqrstuvwxyz'.lower():
                stack_number = int(stack_identifier_row[idx])
                if stacks.get(stack_number):
                    stacks[stack_number].append(c)
                else:
                    stacks[stack_number] = [c]
    return stacks


def get_moves(input: str) -> Moves:
    _, moves = input.split('\n\n')
    moves = moves.splitlines()
    extracted_moves = []
    for mv in moves:
        parts = mv.split(' ')
        count = int(parts[1])
        start = int(parts[3])
        to = int(parts[5])
        extracted_moves.append(Move(count, start, to))
    return extracted_moves


def solve() -> Tuple[str, str]:
    input = open("../inputs/day05.txt").read()
    stacks = get_supply_stacks(input)
    moves = get_moves(input)
    return (part1(deepcopy(stacks), moves), part2(deepcopy(stacks), moves))
