import re


def input_reader(path: str = "input"):
    with open(path, "r", encoding="utf-8") as reader:
        return reader.read().replace("\n", " ")


def part_1(code) -> int:
    pattern = r"mul\((\d+),(\d+)\)"
    sum = 0
    matches = re.finditer(pattern, code)
    for match in matches:
        n1, n2 = int(match.group(1)), int(match.group(2))
        sum += n1 * n2
    return sum


def part_2(code) -> int:
    pattern = r"(?:mul\((\d+),(\d+)\)|do\(\)|don\'t\(\))"
    enabled = True
    sum = 0
    matches = re.finditer(pattern, code)
    for match in matches:
        if match.group().startswith("don't"):
            enabled = False
        elif match.group().startswith("do"):
            enabled = True
        else:
            if not enabled:
                continue
            n1, n2 = int(match.group(1)), int(match.group(2))
            sum += n1 * n2

    return sum
    return 0


code = input_reader()
# code = input_reader("test")
print(part_1(code))
print(part_2(code))
