def input_reader(path: str = "input"):
    with open(path, "r", encoding="utf-8") as reader:
        return [list(l) for l in reader.read().rstrip().split("\n")]


def part_1(garden_map, substract_map=None):
    h = len(garden_map)
    w = len(garden_map[0])

    perimeter_map = []  # [[(label, perimeter)]]
    for y in range(w):
        perimeter_map.append([])
        for x in range(w):
            label = garden_map[y][x]
            tile_perimeter = 0

            # Substract (for part 2)
            if (
                substract_map is not None
                and 0 <= x < len(substract_map)
                and 0 <= y < len(substract_map[0])
            ):
                tile_perimeter = substract_map[y][x]

            # Measure perimeter
            for dx, dy in ((-1, 0), (1, 0), (0, -1), (0, 1)):
                nx, ny = x + dx, y + dy
                if not (0 <= nx < w and 0 <= ny < h):
                    tile_perimeter += 1
                    continue

                if garden_map[ny][nx] != label:
                    tile_perimeter += 1

            perimeter_map[y].append((label, tile_perimeter))

    tested_tiles = set({})

    def count_area(label, x, y) -> tuple[int, int]:
        if (x, y) in tested_tiles:
            return 0, 0

        area = 1
        perimeter = perimeter_map[y][x][1]
        tested_tiles.add((x, y))

        for dx, dy in ((-1, 0), (1, 0), (0, -1), (0, 1)):
            if (
                0 <= x + dx < w
                and 0 <= y + dy < h
                and garden_map[y + dy][x + dx] == label
            ):
                d_area, d_perimeter = count_area(label, x + dx, y + dy)
                area += d_area
                perimeter += d_perimeter

        return area, perimeter

    processed_map = []
    for y in range(w):
        processed_map.append([])
        for x in range(w):
            a, p = count_area(garden_map[y][x], x, y)
            processed_map[y].append((garden_map[y][x], a, p))

    # Sum everything
    sum = 0
    for y in range(w):
        for x in range(h):
            sum += processed_map[y][x][1] * processed_map[y][x][2]

    return sum


def part_2(garden_map):
    h = len(garden_map)
    w = len(garden_map[0])

    def outside_or_different(label, x, y):
        if not (0 <= x < w and 0 <= y < h):
            return True
        return garden_map[y][x] != label

    history = set({})
    substract_map = []

    for y in range(h):
        substract_map.append([])
        for x in range(w):
            substract_map[y].append(0)
            label = garden_map[y][x]

            # Look for edges to remove
            for dx, dy in ((-1, 0), (1, 0), (0, -1), (0, 1)):
                if not (0 <= x + dx < w and 0 <= y + dy < h):
                    continue
                if (x + dx, y + dy) in history:
                    continue
                if garden_map[y + dy][x + dx] != label:
                    continue

                side_offsets = ((dy, dx), (-dy, -dx))

                for kdx, kdy in side_offsets:
                    if outside_or_different(
                        label, x + kdx, y + kdy
                    ) and outside_or_different(label, x + dx + kdx, y + dy + kdy):
                        # We can delete one edge here
                        substract_map[y][-1] -= 1

            history.add((x, y))

    return part_1(garden_map, substract_map)


garden_map = input_reader()
# garden_map = input_reader("test")

print(f"Part 1: {part_1(garden_map)}")
print(f"Part 2: {part_2(garden_map)}")
