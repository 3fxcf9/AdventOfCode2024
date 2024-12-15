def input_reader(path: str = "input"):
    with open(path, "r", encoding="utf-8") as reader:
        for line in reader:
            yield line.rstrip()


def parser(reader):
    for line in reader:
        report = line.split()
        yield list(map(int, report))


def report_safe(report):
    l = len(report) - 1

    increasing_count = 0
    while (
        increasing_count < l
        and 0 < report[increasing_count] - report[increasing_count + 1] <= 3
    ):
        increasing_count += 1

    decreasing_count = 0
    while (
        decreasing_count < l
        and -3 <= report[decreasing_count] - report[decreasing_count + 1] < 0
    ):
        decreasing_count += 1

    return (increasing_count == l) or (decreasing_count == l)


def part_1(reports) -> int:
    safe_count = 0
    for report in reports:
        if report_safe(report):
            safe_count += 1

    return safe_count


def part_2(reports) -> int:
    safe_count = 0
    for report in reports:
        for removed in range(len(report) + 1):  # out of range to keep all
            filtered = [report[i] for i in range(len(report)) if i != removed]

            if report_safe(filtered):
                safe_count += 1
                break

    return safe_count


# parsed = parser(input_reader())
parsed = parser(input_reader("test"))
# print(part_1(parsed))
print(part_2(parsed))
