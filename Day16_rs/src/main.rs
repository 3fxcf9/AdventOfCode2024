use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Hash)]
enum Tile {
    Empty,
    Wall,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl Orientation {
    fn get_offsets(&self) -> (i16, i16) {
        match self {
            Orientation::Up => (0, -1),
            Orientation::Down => (0, 1),
            Orientation::Left => (-1, 0),
            Orientation::Right => (1, 0),
        }
    }
}

fn main() {
    let mut initial_x: Option<i16> = None;
    let mut initial_y: Option<i16> = None;
    let mut end_x: Option<i16> = None;
    let mut end_y: Option<i16> = None;
    let map: Vec<Vec<Tile>> = std::str::from_utf8(include_bytes!("input"))
        .unwrap()
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, e)| match e {
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    'S' => {
                        initial_x = Some(x as i16);
                        initial_y = Some(y as i16);
                        return Tile::Empty;
                    }
                    'E' => {
                        end_x = Some(x as i16);
                        end_y = Some(y as i16);
                        Tile::Empty
                    }
                    _ => todo!(),
                })
                .collect()
        })
        .collect();

    let initial_x = initial_x.unwrap();
    let initial_y = initial_y.unwrap();

    let mut hm: HashMap<((i16, i16), Orientation), u32> = HashMap::new();

    rec((initial_x, initial_y), Orientation::Right, &map, &mut hm, 0);

    // Part 1
    let mut min = hm
        .get(&((end_x.unwrap(), end_y.unwrap()), Orientation::Left))
        .unwrap();

    let mut end_orientation: Orientation = Orientation::Left;
    for orientation in [Orientation::Up, Orientation::Down, Orientation::Right] {
        let current = hm
            .get(&((end_x.unwrap(), end_y.unwrap()), orientation))
            .unwrap();
        if min > current {
            end_orientation = orientation;
            min = current;
        }
    }
    println!("Part 1 : {min}");

    // Part 2
    let mut hs: HashSet<(i16, i16)> = HashSet::new();
    count_best_ways_tiles_rec(
        (initial_x, initial_y),
        Orientation::Right,
        end_orientation,
        &map,
        &mut hm,
        &mut hs,
        0,
        (end_x.unwrap(), end_y.unwrap()),
    );
    print_hs(&map, &hs);
    println!("Part 2 : {}", hs.iter().count());
}

fn rec(
    pos: (i16, i16),
    orientation: Orientation,
    map: &Vec<Vec<Tile>>,
    hm: &mut HashMap<((i16, i16), Orientation), u32>,
    current_score: u32,
) {
    if let Some(score) = hm.get(&(pos, orientation)) {
        if *score < current_score {
            return;
        }
    }
    hm.insert((pos, orientation), current_score);

    match orientation {
        Orientation::Up | Orientation::Down => {
            // à la rache
            let (dx, dy) = Orientation::Left.get_offsets();
            if map[(pos.1 + dy) as usize][(pos.0 + dx) as usize] == Tile::Empty {
                rec(pos, Orientation::Left, map, hm, current_score + 1000);
            }
            let (dx, dy) = Orientation::Right.get_offsets();
            if map[(pos.1 + dy) as usize][(pos.0 + dx) as usize] == Tile::Empty {
                rec(pos, Orientation::Right, map, hm, current_score + 1000);
            }
        }
        Orientation::Left | Orientation::Right => {
            let (dx, dy) = Orientation::Up.get_offsets();
            if map[(pos.1 + dy) as usize][(pos.0 + dx) as usize] == Tile::Empty {
                rec(pos, Orientation::Up, map, hm, current_score + 1000);
            }
            let (dx, dy) = Orientation::Down.get_offsets();
            if map[(pos.1 + dy) as usize][(pos.0 + dx) as usize] == Tile::Empty {
                rec(pos, Orientation::Down, map, hm, current_score + 1000);
            }
        }
    }
    let (dx, dy) = orientation.get_offsets();
    if map[(pos.1 + dy) as usize][(pos.0 + dx) as usize] == Tile::Empty {
        rec(
            (pos.0 + dx, pos.1 + dy),
            orientation,
            map,
            hm,
            current_score + 1,
        );
    }
}

fn count_best_ways_tiles_rec(
    pos: (i16, i16),
    orientation: Orientation,
    end_orientation: Orientation,
    map: &Vec<Vec<Tile>>,
    hm: &mut HashMap<((i16, i16), Orientation), u32>,
    hs: &mut HashSet<(i16, i16)>,
    current_score: u32,
    end: (i16, i16),
) -> bool {
    if pos == end && orientation == end_orientation {
        hs.insert(pos);
        return true;
    }
    if let Some(score) = hm.get(&(pos, orientation)) {
        if *score < current_score {
            return false;
        }
    }
    hm.insert((pos, orientation), current_score);

    let mut truth = false;
    match orientation {
        Orientation::Up | Orientation::Down => {
            let (dx, dy) = Orientation::Left.get_offsets();
            if map[(pos.1 + dy) as usize][(pos.0 + dx) as usize] == Tile::Empty {
                let res = count_best_ways_tiles_rec(
                    pos,
                    Orientation::Left,
                    end_orientation,
                    map,
                    hm,
                    hs,
                    current_score + 1000,
                    end,
                );
                truth = truth || res;
            }
            let (dx, dy) = Orientation::Right.get_offsets();
            if map[(pos.1 + dy) as usize][(pos.0 + dx) as usize] == Tile::Empty {
                let res = count_best_ways_tiles_rec(
                    pos,
                    Orientation::Right,
                    end_orientation,
                    map,
                    hm,
                    hs,
                    current_score + 1000,
                    end,
                );
                truth = truth || res;
            }
        }
        Orientation::Left | Orientation::Right => {
            let (dx, dy) = Orientation::Up.get_offsets();
            if map[(pos.1 + dy) as usize][(pos.0 + dx) as usize] == Tile::Empty {
                let res = count_best_ways_tiles_rec(
                    pos,
                    Orientation::Up,
                    end_orientation,
                    map,
                    hm,
                    hs,
                    current_score + 1000,
                    end,
                );
                truth = truth || res;
            }
            let (dx, dy) = Orientation::Down.get_offsets();
            if map[(pos.1 + dy) as usize][(pos.0 + dx) as usize] == Tile::Empty {
                let res = count_best_ways_tiles_rec(
                    pos,
                    Orientation::Down,
                    end_orientation,
                    map,
                    hm,
                    hs,
                    current_score + 1000,
                    end,
                );
                truth = truth || res;
            }
        }
    }
    let (dx, dy) = orientation.get_offsets();
    if map[(pos.1 + dy) as usize][(pos.0 + dx) as usize] == Tile::Empty {
        let res = count_best_ways_tiles_rec(
            (pos.0 + dx, pos.1 + dy),
            orientation,
            end_orientation,
            map,
            hm,
            hs,
            current_score + 1,
            end,
        );
        truth = truth || res;
    }
    if truth {
        hs.insert(pos);
    } // un truc comme ça
    truth
}

fn print_hs(map: &Vec<Vec<Tile>>, hs: &HashSet<(i16, i16)>) {
    for (y, line) in map.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            if hs.contains(&(x as i16, y as i16)) {
                print!("\x1b[101m  \x1b[0m");
            } else {
                print!(
                    "{symb}",
                    symb = match tile {
                        Tile::Empty => "  ",
                        Tile::Wall => "\x1b[100m  \x1b[0m",
                    }
                );
            }
        }
        println!("  ");
    }
}
