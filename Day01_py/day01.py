def input_reader(path:str="input"):
    with open(path, 'r', encoding="utf-8") as reader:
        for line in reader:
            yield line.rstrip()

def parser(reader) -> tuple[list[int],list[int]]:
    left_list, right_list = [],[]
    for line in reader:
        numbers = line.split()
        left_list.append(int(numbers[0]))
        right_list.append(int(numbers[1]))
    return (left_list, right_list)

def part_1(left:list[int], right:list[int]) -> int:
    left.sort()
    right.sort()

    assert len(left)==len(right)

    total = 0
    for (a,b) in zip(left, right):
        total += abs(a-b)

    return total


def part_2(left:list[int], right:list[int]) -> int:
    count = {
        number : right.count(number)
        for number in left
    }
    return sum([number * count[number] for number in left])

parsed=parser(input_reader())
# parsed=parser(input_reader("test"))
print(part_1(*parsed))
print(part_2(*parsed))
