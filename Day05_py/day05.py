import functools


def input_reader(path: str = "input"):
    with open(path, "r", encoding="utf-8") as reader:
        for line in reader:
            yield line.rstrip()


def parser(reader):
    rules = []
    updates = []

    part2 = False
    for line in reader:
        if line == "":
            part2 = True
            continue
        if part2:
            updates.append(list(map(int, line.split(","))))
        else:
            rule = list(map(int, line.split("|")))
            rules.append(rule)

    return rules, updates


def is_update_correct(update, rules):
    i = len(rules)

    while i > 0:
        i -= 1
        rule = rules[i]
        if (rule[0] not in update) or (rule[1] not in update):
            continue
        if update.index(rule[0]) >= update.index(rule[1]):
            break

    return i == 0


def sum_middle_items(updates):
    sum = 0
    for update in updates:
        assert len(update) % 2 == 1
        sum += update[len(update) // 2]
    return sum


def part_1(rules, updates) -> int:
    correct_updates = list(
        filter(lambda update: is_update_correct(update, rules), updates)
    )

    return sum_middle_items(correct_updates)


def part_2(rules, updates) -> int:
    incorrect = list(
        filter(lambda update: not is_update_correct(update, rules), updates)
    )

    # Locally declared to have access to rules with only 2 params
    def compare(a, b):
        for rule in rules:
            if a in rule and b in rule:
                return -1 if rule[0] == a else 1
        return 0

    return sum_middle_items(
        [sorted(to_sort, key=functools.cmp_to_key(compare)) for to_sort in incorrect]
    )


reader = input_reader()
# reader = input_reader("test")
rules, updates = parser(reader)
print("Part 1:", part_1(rules, updates))
print("Part 2:", part_2(rules, updates))
