from typing import Callable
from datetime import datetime
import argparse
import day01
import day04


def get_day_solver(day: int) -> Callable[[], tuple[int, int]]:
    match day:
        case 1:
            return day01.solve
        case 4:
            return day04.solve
        case _:
            raise ValueError(f"Day {day} not implemented")


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('day', help='Day of the Advent of Code')
    args = parser.parse_args()
    day = int(args.day)

    solver = get_day_solver(day)

    start = datetime.now()
    p1, p2 = solver()
    end = datetime.now()
    milliseconds = round((end - start).total_seconds() * 1000, 4)

    print(f"=== Day {day} ===")
    print(f"  . Part 1: {p1}")
    print(f"  . Part 2: {p2}")
    print(f"Total runtime: {milliseconds} ms")
