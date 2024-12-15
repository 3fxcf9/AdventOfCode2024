def input_reader(path: str = "input"):
    with open(path, "r", encoding="utf-8") as reader:
        return [list(map(int,list(l))) for l in reader.read().rstrip().split("\n")]

topo = input_reader()
# topo = input_reader("test")


offsets = (
    (0,1),(0,-1),(1,0),(-1,0)
)
def count_trailheads(x,y,start, table) -> int:
    if topo[y][x] != start:
        return 0
    if start == 9:
        if (x,y) not in table:
            table.add((x,y))
            return 1
        return 0

    node_count = 0
    for (dx,dy) in offsets:
        if 0<=x+dx<w and 0<=y+dy<h:
            node_count += count_trailheads(x+dx,y+dy,start+1, table)

    return node_count


def count_trailheads_2(x,y,start) -> int:
    if topo[y][x] != start:
        return 0
    if start == 9:
        return 1

    node_count = 0
    for (dx,dy) in offsets:
        if 0<=x+dx<w and 0<=y+dy<h:
            node_count += count_trailheads_2(x+dx,y+dy,start+1)

    return node_count

score = 0
score_2 = 0
h = len(topo)
w = len(topo[0])
for y in range(h):
    for x in range(w):
        score += count_trailheads(x,y,0, set(()))
        score_2 += count_trailheads_2(x,y,0)


print(f'Part 1 : {score}')
print(f'Part 2 : {score_2}')
