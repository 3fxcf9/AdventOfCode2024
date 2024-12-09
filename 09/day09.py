def input_reader(path: str = "input"):
    with open(path, "r", encoding="utf-8") as reader:
        return reader.read().rstrip().split("\n")

def parse(reader):
    assert len(reader) == 1

    diskmap = []
    diskmap2 = []
    file = True
    id = 0
    for char in reader[0]:
        if file:
            symbol = str(id)
            id += 1
        else :
            symbol = "."

        for _ in range(int(char)):
            diskmap.append(symbol)
        if int(char) > 0:diskmap2.append([symbol, int(char)])

        file = not file

    assert file==False # Last is file

    return diskmap,diskmap2


def compute_checksum(diskmap):
    return sum(i*int(diskmap[i]) for i in range(len(diskmap)) if diskmap[i] != ".")


def part_1(diskmap):
    current_index = 0
    max_index = len(diskmap)-1
    for i in range(len(diskmap)-1,-1,-1):
        if diskmap[i] == ".":
            continue

        while current_index < max_index and diskmap[current_index] != ".":
            current_index += 1

        if current_index > i:
            break

        diskmap[current_index] = diskmap[i]
        diskmap[i] = "."

    return compute_checksum(diskmap)


def part_2(diskmap):
    i = -1
    while i >= -len(diskmap):
        if diskmap[i][0] == ".":
            i-=1
            continue

        current_index = -len(diskmap)
        while current_index < i and (diskmap[current_index][0] != "." or diskmap[current_index][1] < diskmap[i][1]):
            current_index += 1

        if current_index >= i:
            i-=1
            continue

        if diskmap[current_index][1] == diskmap[i][1]:
            diskmap[current_index] = diskmap[i].copy()
            diskmap[i][0] = "."
        else:
            diskmap[current_index][1] -= diskmap[i][1]
            diskmap.insert(current_index,diskmap[i].copy())
            diskmap[i][0] = "."
        
        i-=1

    # Unwrap blocks
    blocks = []
    for b in diskmap:
        blocks.extend([b[0]] * b[1])

    return compute_checksum(blocks)



reader = input_reader()
# reader = input_reader("test")

diskmap,diskmap2 = parse(reader);
print("Part 1:", part_1(diskmap))
print("Part 2:", part_2(diskmap2))
