#![allow(unused)]
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Hash, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
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
    let end_x = end_x.unwrap();
    let end_y = end_y.unwrap();
    // let reference = solve_maze(&map, (initial_x, initial_y), (end_x, end_y));

    let offsets_2 = get_radius_offsets(2);
    let offsets_20 = get_radius_offsets(20);

    let distances_from_start = dijkstra(&map, (initial_x, initial_y));
    let distances_from_end = dijkstra(&map, (end_x, end_y));

    let reference = distances_from_start.get(&(end_x, end_y)).unwrap();

    // print_map_distances(&map, Some(&distances_from_start));

    println!(
        "Part 1: {}",
        solve(
            &map,
            &offsets_2,
            &distances_from_start,
            &distances_from_end,
            reference,
            100
        )
    );
    println!(
        "Part 2: {}",
        solve(
            &map,
            &offsets_20,
            &distances_from_start,
            &distances_from_end,
            reference,
            100
        )
    );
}

fn solve(
    map: &Vec<Vec<Tile>>,
    offsets: &Vec<(i16, i16)>,
    start_distances: &HashMap<(i16, i16), u16>,
    end_distances: &HashMap<(i16, i16), u16>,
    reference: &u16,
    threshold: u16,
) -> u32 {
    let mut count: u32 = 0;
    let mut tested_cheats: HashSet<((u8, u8), (u8, u8))> = HashSet::new();
    for (y, line) in map.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            if tile == &Tile::Wall {
                continue;
            }
            let reachable_tiles = get_radius_coordinates(map, &offsets, x as i16, y as i16);
            // print!("\x1b[H");
            // print_map(map, Some(&reachable_tiles));
            for (end_x, end_y) in reachable_tiles {
                // Pass if cheat already tested this way...
                if tested_cheats.contains(&((x as u8, y as u8), (end_x as u8, end_y as u8))) {
                    continue;
                }
                // ... and in reversed order
                if tested_cheats.contains(&((end_x as u8, end_y as u8), (x as u8, y as u8))) {
                    continue;
                }

                let cheat_distance = end_x.abs_diff(x as i16) + end_y.abs_diff(y as i16);
                let total_distance_a = start_distances.get(&(x as i16, y as i16)).unwrap()
                    + (cheat_distance as u16)
                    + end_distances.get(&(end_x as i16, end_y as i16)).unwrap();
                let total_distance_b = start_distances.get(&(end_x as i16, end_y as i16)).unwrap()
                    + (cheat_distance as u16)
                    + end_distances.get(&(x as i16, y as i16)).unwrap();
                let distance = total_distance_a.min(total_distance_b);
                if distance + threshold <= *reference {
                    count += 1;
                }
                tested_cheats.insert(((x as u8, y as u8), (end_x as u8, end_y as u8)));
            }
        }
    }
    count
}

fn get_radius_offsets(radius: u8) -> Vec<(i16, i16)> {
    let mut offsets = vec![];
    for x in -(radius as i16)..=(radius as i16) {
        for y in -(radius as i16)..=(radius as i16) {
            if x.abs() + y.abs() > radius as i16 {
                continue;
            }
            offsets.push((x, y));
        }
    }
    offsets
}

fn get_radius_coordinates(
    map: &Vec<Vec<Tile>>,
    offsets: &Vec<(i16, i16)>,
    x: i16,
    y: i16,
) -> Vec<(i16, i16)> {
    let mut filtered = vec![];
    for (dx, dy) in offsets {
        let (nx, ny) = (x + *dx, y + *dy);
        if !(0 <= nx && (nx as usize) < map[0].len() && 0 <= ny && (ny as usize) < map.len()) {
            continue;
        }
        if map[ny as usize][nx as usize] == Tile::Wall {
            continue;
        }
        filtered.push((nx, ny));
    }
    filtered
}

fn pop_queue(queue: &mut Vec<(i16, i16)>, distances: &HashMap<(i16, i16), u16>) -> (i16, i16) {
    let min_index = queue
        .iter()
        .enumerate()
        .min_by_key(|(_, i)| distances.get(i))
        .map(|(index, _)| index)
        .unwrap();

    queue.remove(min_index)
}

fn dijkstra(map: &Vec<Vec<Tile>>, start: (i16, i16)) -> HashMap<(i16, i16), u16> {
    let mut distances: HashMap<(i16, i16), u16> = HashMap::new(); // (x,y) : distance
    distances.insert(start, 0);
    let mut queue: Vec<(i16, i16)> = vec![start]; // (x,y)

    while !queue.is_empty() {
        // Get the nearest unvisited tile and remove it from the queue
        let current = pop_queue(&mut queue, &distances);

        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (tx, ty): (i16, i16) = (current.0 as i16 + dx, current.1 as i16 + dy);
            if !(0 <= tx && tx < map[0].len() as i16 && 0 <= ty && ty < map.len() as i16) {
                continue;
            }
            if map[ty as usize][tx as usize] == Tile::Wall {
                continue;
            }
            if distances.contains_key(&(tx, ty)) {
                continue;
            }
            distances.insert((tx, ty), *distances.get(&current).unwrap() + 1);
            queue.push((tx, ty));
        }
    }
    distances
}

fn print_map(map: &Vec<Vec<Tile>>, highlight: Option<&Vec<(i16, i16)>>) {
    for (y, line) in map.iter().enumerate() {
        for (x, col) in line.iter().enumerate() {
            if let Some(ref dist) = highlight {
                if dist.contains(&(x as i16, y as i16)) {
                    print!("\x1b[101m  \x1b[0m",);
                    continue;
                }
            }
            if *col == Tile::Wall {
                print!("\x1b[100m  \x1b[0m");
            } else {
                print!("  ");
            }
        }
        println!("");
    }
}
fn print_map_distances(map: &Vec<Vec<Tile>>, distances: Option<&HashMap<(i16, i16), u16>>) {
    for (y, line) in map.iter().enumerate() {
        for (x, col) in line.iter().enumerate() {
            if let Some(ref dist) = distances {
                if dist.contains_key(&(x as i16, y as i16)) {
                    print!(
                        "\x1b[101m{:>2}\x1b[0m",
                        dist.get(&(x as i16, y as i16)).unwrap() % 100
                    );
                    continue;
                }
            }
            if *col == Tile::Wall {
                print!("\x1b[100m  \x1b[0m");
            } else {
                print!("  ");
            }
        }
        println!("");
    }
}
