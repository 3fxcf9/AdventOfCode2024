import re


def input_reader(path: str = "input"):
    with open(path, "r", encoding="utf-8") as reader:
        return reader.read().rstrip().split("\n\n")


def parser(reader):  # [[ax, ay, bx, by, px, py]]
    machines = []
    for machine in reader:
        expr = r"[XY][+=](\d+)"
        machines.append(list(map(int, re.findall(expr, machine))))
    return machines


def part_1(machines, part_2: bool = False):
    sum = 0

    for ax, ay, bx, by, px, py in machines:
        if part_2:
            px += 10000000000000
            py += 10000000000000

        det = ax * by - bx * ay
        assert det != 0, "Null determinant"
        detA = px * by - bx * py
        detB = ax * py - px * ay
        A, B = detA / det, detB / det

        if A % 1 != 0 or B % 1 != 0:
            continue

        sum += 3 * A + B

    return int(sum)


parsed = parser(input_reader())
# parsed = parser(input_reader("test"))

print(f"Part 1: {part_1(parsed)}")
print(f"Part 2: {part_1(parsed, True)}")
