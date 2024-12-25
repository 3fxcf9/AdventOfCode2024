use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    LeftBox,
    RightBox,
    Wall,
}

#[derive(Debug, Clone, Copy)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Robot {
    x: i16,
    y: i16,
}
impl Robot {
    fn get_offsets(mov: Movement) -> (i16, i16) {
        match mov {
            Movement::Up => (0, -1),
            Movement::Down => (0, 1),
            Movement::Left => (-1, 0),
            Movement::Right => (1, 0),
        }
    }
}

type Map = Vec<Vec<Tile>>;

pub fn main() {
    // Parse map
    let mut initial_x: Option<u8> = None;
    let mut initial_y: Option<u8> = None;
    let input_parts = std::str::from_utf8(include_bytes!("input"))
        .unwrap()
        .split("\n\n")
        .collect::<Vec<&str>>();

    let mut map: Map = vec![];
    for (y, line) in input_parts.first().unwrap().lines().enumerate() {
        map.push(
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => vec![Tile::Wall, Tile::Wall],
                    'O' => vec![Tile::LeftBox, Tile::RightBox],
                    '.' => vec![Tile::Empty, Tile::Empty],
                    '@' => {
                        initial_x = Some(2 * x as u8);
                        initial_y = Some(y as u8);
                        return vec![Tile::Empty, Tile::Empty];
                    }
                    _ => todo!("Unknown map tile"),
                })
                .flatten()
                .collect(),
        );
    }

    let mut robot = Robot {
        x: initial_x.unwrap() as i16,
        y: initial_y.unwrap() as i16,
    };

    // Parse movements
    let movements = input_parts
        .last()
        .unwrap()
        .chars()
        .map(|c| match c {
            '<' => Some(Movement::Left),
            'v' => Some(Movement::Down),
            '>' => Some(Movement::Right),
            '^' => Some(Movement::Up),
            _ => None,
        })
        .filter(|m| m.is_some())
        .map(|m| m.unwrap())
        .collect::<Vec<Movement>>();

    for mov in movements {
        // print_map(&map, Some(&robot));
        // println!("Moving: {:?}", &mov);
        let (dx, dy) = Robot::get_offsets(mov);
        let (nx, ny) = ((robot.x + dx) as usize, (robot.y + dy) as usize);
        match map[ny][nx] {
            Tile::Empty => {
                robot.x += dx;
                robot.y += dy;
            }
            Tile::RightBox => match mov {
                Movement::Left => {
                    if let Some((i, _)) = map[robot.y as usize]
                        .iter()
                        .enumerate()
                        .rev()
                        .find(|(i, &t)| *i < nx && t == Tile::Empty)
                    {
                        if !map[robot.y as usize][i..nx].contains(&Tile::Wall) {
                            for x in i..nx {
                                map[robot.y as usize][x] = match map[robot.y as usize][x] {
                                    Tile::LeftBox => Tile::RightBox,
                                    Tile::RightBox => Tile::LeftBox,
                                    Tile::Empty => Tile::LeftBox,
                                    _ => todo!(),
                                };
                            }
                            map[ny][nx] = Tile::Empty;
                            robot.x += dx;
                            robot.y += dy;
                        }
                    }
                }
                Movement::Down | Movement::Up => {
                    if let Some(to_move) = is_movable_wrapper((nx as i16 - 1, ny as i16), mov, &map)
                    {
                        let reversed = match mov {
                            Movement::Up => false,
                            Movement::Down => true,
                            _ => todo!("Unreachable"),
                        };
                        let mut sorted = to_move.iter().map(|a| *a).collect::<Vec<(i16, i16)>>();
                        sorted.sort_by_key(|x| (x.1, x.0));
                        if reversed {
                            sorted.reverse();
                        }
                        for (_x, _y) in sorted {
                            map[_y as usize][_x as usize] = Tile::Empty;
                            map[_y as usize][_x as usize + 1] = Tile::Empty;
                            map[(_y + dy) as usize][_x as usize] = Tile::LeftBox;
                            map[(_y + dy) as usize][_x as usize + 1] = Tile::RightBox;
                        }
                        map[ny as usize][nx as usize] = Tile::Empty;
                        map[ny as usize][nx as usize - 1] = Tile::Empty;
                        robot.x += dx;
                        robot.y += dy;
                    }
                }
                _ => todo!(),
            },
            Tile::LeftBox => match mov {
                Movement::Right => {
                    if let Some((i, _)) = map[robot.y as usize]
                        .iter()
                        .enumerate()
                        .find(|(i, &t)| *i > nx as usize && t == Tile::Empty)
                    {
                        if !map[robot.y as usize][nx..i].contains(&Tile::Wall) {
                            for x in nx..=i {
                                map[robot.y as usize][x] = match map[robot.y as usize][x] {
                                    Tile::LeftBox => Tile::RightBox,
                                    Tile::RightBox => Tile::LeftBox,
                                    Tile::Empty => Tile::RightBox,
                                    _ => todo!(),
                                };
                            }
                            map[ny][nx] = Tile::Empty;
                            robot.x += dx;
                            robot.y += dy;
                        }
                    }
                }
                Movement::Down | Movement::Up => {
                    if let Some(to_move) = is_movable_wrapper((nx as i16, ny as i16), mov, &map) {
                        let reversed = match mov {
                            Movement::Up => false,
                            Movement::Down => true,
                            _ => todo!("Unreachable"),
                        };
                        let mut sorted = to_move.iter().map(|a| *a).collect::<Vec<(i16, i16)>>();
                        sorted.sort_by_key(|x| (x.1, x.0));
                        if reversed {
                            sorted.reverse();
                        }
                        for (_x, _y) in sorted {
                            map[_y as usize][_x as usize] = Tile::Empty;
                            map[_y as usize][_x as usize + 1] = Tile::Empty;
                            map[(_y + dy) as usize][_x as usize] = Tile::LeftBox;
                            map[(_y + dy) as usize][_x as usize + 1] = Tile::RightBox;
                        }
                        map[ny as usize][nx as usize] = Tile::Empty;
                        map[ny as usize][nx as usize + 1] = Tile::Empty;
                        robot.x += dx;
                        robot.y += dy;
                    }
                }
                _ => todo!(),
            },
            Tile::Wall => (),
        }
    }
    print_map(&map, Some(&robot));
    println!("Part 2 : {}", sum_gps(&map));
}

fn print_map(map: &Map, robot: Option<&Robot>) {
    for (y, line) in map.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            if let Some(r) = robot {
                if x as i16 == r.x && y as i16 == r.y {
                    print!("@");
                    continue;
                }
            }
            match tile {
                Tile::LeftBox => print!("\x1b[92m[\x1b[0m"),
                Tile::RightBox => print!("\x1b[92m]\x1b[0m"),
                Tile::Wall => print!("\x1b[91m╳\x1b[0m"),
                Tile::Empty => print!("\x1b[90m·\x1b[0m"),
            }
        }
        println!();
    }
}

fn gps(x: usize, y: usize) -> usize {
    return x + y * 100;
}

fn sum_gps(map: &Map) -> usize {
    let mut sum: usize = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            match tile {
                Tile::LeftBox => {
                    sum += gps(x, y);
                }
                _ => (),
            }
        }
    }
    sum
}

fn is_movable(
    left_part_coords: (i16, i16),
    direction: Movement,
    map: &Map,
    to_move: &mut HashSet<(i16, i16)>,
) -> bool {
    let (dx, dy) = Robot::get_offsets(direction);
    let (nx, ny) = ((left_part_coords.0 + dx), (left_part_coords.1 + dy));
    to_move.insert(left_part_coords);
    (match map[ny as usize][nx as usize] {
        // Check the left side of the box
        Tile::Empty => true,
        Tile::LeftBox => is_movable((nx, ny), direction, map, to_move),
        Tile::RightBox => is_movable((nx - 1, ny), direction, map, to_move),
        Tile::Wall => false,
    } && match map[ny as usize][nx as usize + 1] {
        // Check the right side of the box
        Tile::Empty => true,
        Tile::LeftBox => is_movable((nx + 1, ny), direction, map, to_move),
        Tile::RightBox => is_movable((nx, ny), direction, map, to_move),
        Tile::Wall => false,
    })
}

fn is_movable_wrapper(
    left_part_coords: (i16, i16),
    direction: Movement,
    map: &Map,
) -> Option<HashSet<(i16, i16)>> {
    let mut to_move = HashSet::new();

    if is_movable(left_part_coords, direction, map, &mut to_move) {
        Some(to_move)
    } else {
        None
    }
}
