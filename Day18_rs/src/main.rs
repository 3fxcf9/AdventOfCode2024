use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Tile {
    Empty,
    Corrupted,
}

fn main() {
    // Hardcoded config
    let n = 1024;
    let (h, w) = (71, 71);
    let exit = (h - 1, w - 1);

    let falling_bytes: Vec<Vec<u8>> = std::str::from_utf8(include_bytes!("input"))
        .unwrap()
        .lines()
        .map(|l| {
            l.split(",")
                .map(|c| c.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect();

    let map = (0..h)
        .map(|y| {
            (0..w)
                .map(|x| match &falling_bytes[..n].contains(&vec![x, y]) {
                    true => Tile::Corrupted,
                    false => Tile::Empty,
                })
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<Vec<Tile>>>();

    // Part 1
    let (exit_distance, path) = solve_maze(&map, exit);
    print_map(&map, Some(path));
    println!("Part 1: {}", exit_distance.unwrap());

    // Part 2
    let mut i = n;
    let first_blocked = loop {
        let map = (0..h)
            .map(|y| {
                (0..w)
                    .map(|x| match &falling_bytes[..=(n + i)].contains(&vec![x, y]) {
                        true => Tile::Corrupted,
                        false => Tile::Empty,
                    })
                    .collect::<Vec<Tile>>()
            })
            .collect::<Vec<Vec<Tile>>>();
        let (exit_distance, path) = solve_maze(&map, exit);
        print_map(&map, Some(path.clone()));
        print!("\x1bc");
        if exit_distance == None {
            print_map(&map, Some(path));
            break falling_bytes[n + i].clone();
        }
        i += 1;
    };
    println!(
        "Part 2: {}",
        first_blocked
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );
}

fn pop_queue(queue: &mut Vec<(u8, u8)>, distances: &HashMap<(u8, u8), u16>) -> (u8, u8) {
    let min_index = queue
        .iter()
        .enumerate()
        .min_by_key(|(_, i)| distances.get(i))
        .map(|(index, _)| index)
        .unwrap();

    queue.remove(min_index)
}

fn solve_maze(map: &Vec<Vec<Tile>>, exit: (u8, u8)) -> (Option<u16>, Vec<(u8, u8)>) {
    let mut distances: HashMap<(u8, u8), u16> = HashMap::new(); // (x,y) : distance
    distances.insert((0, 0), 0);
    let mut queue: Vec<(u8, u8)> = vec![(0, 0)]; // (x,y)

    let mut exit_distance: Option<u16> = None;
    while !queue.is_empty() {
        // Get the nearest unvisited tile and remove it from the queue
        let current = pop_queue(&mut queue, &distances);

        if current == exit {
            exit_distance = Some(distances.get(&current).unwrap().to_owned());
            break;
        }

        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (tx, ty): (i8, i8) = (current.0 as i8 + dx, current.1 as i8 + dy);
            if !(0 <= tx && tx < map[0].len() as i8 && 0 <= ty && ty < map.len() as i8) {
                continue;
            }
            if map[ty as usize][tx as usize] == Tile::Corrupted {
                continue;
            }
            if distances.contains_key(&(tx as u8, ty as u8)) {
                continue;
            }
            distances.insert((tx as u8, ty as u8), *distances.get(&current).unwrap() + 1);
            queue.push((tx as u8, ty as u8));
        }
    }
    (
        exit_distance,
        distances.into_keys().collect::<Vec<(u8, u8)>>(),
    )
}

fn print_map(map: &Vec<Vec<Tile>>, path: Option<Vec<(u8, u8)>>) {
    println!("\x1b[93m╭{}╮\x1b[0m", "──".repeat(map[0].len()));
    for (y, line) in map.iter().enumerate() {
        for (x, col) in line.iter().enumerate() {
            if x == 0 {
                print!("\x1b[93m│\x1b[0m")
            }
            if let Some(ref visited) = path {
                if visited.contains(&(x as u8, y as u8)) {
                    print!("\x1b[101m  \x1b[0m");
                    if x == map[0].len() - 1 {
                        print!("\x1b[93m│\x1b[0m")
                    }
                    continue;
                }
            }
            if *col == Tile::Corrupted {
                print!("\x1b[100m  \x1b[0m");
            } else {
                print!("  ");
            }
            if x == map[0].len() - 1 {
                print!("\x1b[93m│\x1b[0m")
            }
        }
        println!("");
    }
    println!("\x1b[93m╰{}╯\x1b[0m", "──".repeat(map[0].len()));
}
