import functools

def input_reader(path: str = "input"):
    with open(path, "r", encoding="utf-8") as reader:
        return list(map(int,reader.read().rstrip().split("\n")[0].split(" ")))

@functools.cache
def count_stones(stone, depth=25) -> int:
    if depth == 0:
        return 1

    if stone == 0:
        return count_stones(1, depth-1)
    elif len(str(stone)) % 2 == 0:
        split_index = int(len(str(stone))/2)
        return count_stones(int(str(stone)[:split_index]), depth-1) + count_stones(int(str(stone)[split_index:]), depth-1)
    else:
        return count_stones(stone * 2024, depth-1)

def solve(first_line, depth):
    return sum([count_stones(s, depth) for s in first_line])

first_line = input_reader()
# first_line = input_reader("test")

print(f"Part 1: {solve(first_line, 25)}")
print(f"Part 2: {solve(first_line, 75)}")
