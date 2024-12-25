#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Box,
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

pub fn main() {
    // Parse map
    let mut initial_x: Option<u8> = None;
    let mut initial_y: Option<u8> = None;
    let input_parts = std::str::from_utf8(include_bytes!("input"))
        .unwrap()
        .split("\n\n")
        .collect::<Vec<&str>>();

    let mut map: Vec<Vec<Tile>> = vec![];
    for (y, line) in input_parts.first().unwrap().lines().enumerate() {
        map.push(
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Tile::Wall,
                    'O' => Tile::Box,
                    '.' => Tile::Empty,
                    '@' => {
                        initial_x = Some(x as u8);
                        initial_y = Some(y as u8);
                        return Tile::Empty;
                    }
                    _ => todo!("Unknown map tile"),
                })
                .collect(),
        );
    }

    let h = map.len() as i16;

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
        //print_map(&map, Some(&robot));
        // println!("Moving: {:?}", &mov);
        let (dx, dy) = Robot::get_offsets(mov);
        let (nx, ny) = ((robot.x + dx) as usize, (robot.y + dy) as usize);
        match map[ny][nx] {
            Tile::Empty => {
                robot.x += dx;
                robot.y += dy;
            }

            Tile::Box => match mov {
                Movement::Up | Movement::Down => {
                    let range = match mov {
                        Movement::Up => (0..=ny).rev().collect::<Vec<usize>>(),
                        Movement::Down => (ny..h as usize).collect::<Vec<usize>>(),
                        _ => todo!("Error"),
                    };

                    let mut first_empty_row: Option<usize> = None;
                    for y in range {
                        if map[y][nx] == Tile::Wall {
                            break;
                        }
                        if map[y][nx] == Tile::Empty {
                            first_empty_row = Some(y);
                            break;
                        }
                    }
                    if let Some(free_y) = first_empty_row {
                        map[free_y][nx] = Tile::Box;
                        map[ny][nx] = Tile::Empty;
                        robot.x += dx;
                        robot.y += dy;
                    }
                }
                Movement::Left => {
                    if let Some((i, _)) = map[robot.y as usize]
                        .iter()
                        .enumerate()
                        .rev()
                        .find(|(i, &t)| *i < nx && t == Tile::Empty)
                    {
                        if !map[robot.y as usize][i..nx].contains(&Tile::Wall) {
                            map[ny][nx] = Tile::Empty;
                            map[ny][i] = Tile::Box;
                            robot.x += dx;
                            robot.y += dy;
                        }
                    }
                }
                Movement::Right => {
                    if let Some((i, _)) = map[robot.y as usize]
                        .iter()
                        .enumerate()
                        .find(|(i, &t)| *i > nx as usize && t == Tile::Empty)
                    {
                        if !map[robot.y as usize][nx..i].contains(&Tile::Wall) {
                            map[ny][nx] = Tile::Empty;
                            map[ny][i] = Tile::Box;
                            robot.x += dx;
                            robot.y += dy;
                        }
                    }
                }
            },
            Tile::Wall => (),
        }
    }
    print_map(&map, Some(&robot));
    println!("Part 1 : {}", sum_gps(&map));
}

fn print_map(map: &Vec<Vec<Tile>>, robot: Option<&Robot>) {
    for (y, line) in map.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            if let Some(r) = robot {
                if x as i16 == r.x && y as i16 == r.y {
                    print!("🤖");
                    continue;
                }
            }
            match tile {
                Tile::Box => print!("📦"),
                Tile::Wall => print!("\x1b[91m╳╳\x1b[0m"),
                Tile::Empty => print!("\x1b[90m<>\x1b[0m"),
            }
        }
        println!();
    }
}

fn gps(x: usize, y: usize) -> usize {
    return x + y * 100;
}

fn sum_gps(map: &Vec<Vec<Tile>>) -> usize {
    let mut sum: usize = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            match tile {
                Tile::Box => {
                    sum += gps(x, y);
                }
                _ => (),
            }
        }
    }
    sum
}
