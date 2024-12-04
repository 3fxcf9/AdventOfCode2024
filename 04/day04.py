def input_reader(path: str = "input"):
    with open(path, "r", encoding="utf-8") as reader:
        return reader.read().rstrip().split("\n")


look_table_1 = [
    # Horizontal, LTR
    ((0, 0, "X"), (1, 0, "M"), (2, 0, "A"), (3, 0, "S")),
    # Horizontal, RTL
    ((3, 0, "X"), (2, 0, "M"), (1, 0, "A"), (0, 0, "S")),
    # Vertical TTB
    ((0, 0, "X"), (0, 1, "M"), (0, 2, "A"), (0, 3, "S")),
    # Vertical BTT
    ((0, 3, "X"), (0, 2, "M"), (0, 1, "A"), (0, 0, "S")),
    # Diagnoals
    ((0, 0, "X"), (1, 1, "M"), (2, 2, "A"), (3, 3, "S")),
    ((0, 0, "S"), (1, 1, "A"), (2, 2, "M"), (3, 3, "X")),
    ((0, 3, "X"), (1, 2, "M"), (2, 1, "A"), (3, 0, "S")),
    ((0, 3, "S"), (1, 2, "A"), (2, 1, "M"), (3, 0, "X")),
]

look_table_2 = [
    ((0, 0, "M"), (1, 1, "A"), (2, 2, "S"), (2, 0, "M"), (0, 2, "S")),
    ((0, 0, "M"), (1, 1, "A"), (2, 2, "S"), (2, 0, "S"), (0, 2, "M")),
    ((0, 0, "S"), (1, 1, "A"), (2, 2, "M"), (2, 0, "M"), (0, 2, "S")),
    ((0, 0, "S"), (1, 1, "A"), (2, 2, "M"), (2, 0, "S"), (0, 2, "M")),
]


def solve(lines, look_table) -> int:
    found = 0

    col_len = len(lines)
    line_len = len(lines[0])
    assert [len(line) for line in lines] == [line_len] * len(lines)

    for y in range(col_len):
        for x in range(line_len):
            for word_pos in look_table:
                word_found = True
                for lpos in word_pos:
                    sx = x + lpos[0]
                    sy = y + lpos[1]
                    if not (0 <= sx < line_len and 0 <= sy < col_len):
                        word_found = False
                        break
                    if lines[sy][sx] != lpos[2]:
                        word_found = False
                if word_found:
                    found += 1
    return found


lines = input_reader()
# lines = input_reader("test")
print("Part 1:", solve(lines, look_table_1))
print("Part 2:", solve(lines, look_table_2))
