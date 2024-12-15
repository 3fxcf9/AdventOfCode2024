use regex::Regex;
use std::{fmt, thread::sleep, time::Duration};

#[derive(Debug, Clone, Copy)]
struct Coordinates {
    x: i16,
    y: i16,
}
impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: Coordinates,
    speed: Coordinates,
}
impl Robot {
    fn step(&mut self, bx: u8, by: u8) {
        self.position.x = (self.position.x + self.speed.x).rem_euclid(bx as i16);
        self.position.y = (self.position.y + self.speed.y).rem_euclid(by as i16);
    }
}
impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Robot\n -> position: {}\n -> speed: {}",
            self.position, self.speed
        )
    }
}

fn print_map(robots: &Vec<Robot>, w: u8, h: u8) {
    let mut grid = vec![vec![0; w as usize]; h as usize];
    for Robot {
        position: Coordinates { x, y },
        ..
    } in robots
    {
        grid[*y as usize][*x as usize] += 1;
    }

    for line in grid {
        for col in line {
            print!(
                "{:>2}",
                match col {
                    0 => " ".to_string(),
                    _ => "#".to_string(),
                }
            );
        }
        println!("");
    }
}

fn main() {
    let mut lines = std::str::from_utf8(include_bytes!("input"))
        .unwrap()
        .lines();
    // let mut lines = std::str::from_utf8(include_bytes!("test")).unwrap().lines();

    let [w, h] = lines
        .next()
        .unwrap()
        .split(" ")
        .map(|e| e.parse::<u8>().unwrap())
        .collect::<Vec<u8>>()[..]
    else {
        todo!("Dimensions parse error")
    };

    assert_eq!(w % 2, 1);
    assert_eq!(h % 2, 1);

    println!("Map dimensions: width={w}; height={h}");

    let mut robots: Vec<Robot> = vec![];
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    lines.for_each(|l| {
        for [px, py, vx, vy] in re
            .captures_iter(l)
            .map(|m| m.extract().1.map(|e| e.parse::<i16>().unwrap()))
        {
            robots.push(Robot {
                position: Coordinates { x: px, y: py },
                speed: Coordinates { x: vx, y: vy },
            });
        }
    });

    let mut robots_2 = robots.clone();

    let iterations_count: u8 = 100;

    for _ in 0..iterations_count {
        robots.iter_mut().for_each(|r| r.step(w, h));
    }

    // Compute safety score
    let mx: u8 = w / 2;
    let my: u8 = h / 2;

    let mut quadrants = vec![0, 0, 0, 0];
    for Robot {
        position: Coordinates { x, y },
        ..
    } in &robots
    {
        if *x as u8 == mx || *y as u8 == my {
            continue;
        }
        if (*x as u8) < mx {
            if (*y as u8) < my {
                quadrants[0] += 1
            } else {
                quadrants[1] += 1
            }
        } else {
            if (*y as u8) < my {
                quadrants[2] += 1
            } else {
                quadrants[3] += 1
            }
        }
    }

    println!("Part 1: {}", quadrants.iter().product::<u32>());

    // Part 2
    let mut iter_count = 0;
    loop {
        robots_2.iter_mut().for_each(|r| r.step(w, h));
        iter_count += 1;
        if is_unlikely(&robots_2, w, h) {
            continue;
        }
        sleep(Duration::from_millis(100));
        println!("Iteration {}", iter_count);
        print_map(&robots_2, w, h);
    }
}

fn is_unlikely(robots: &Vec<Robot>, w: u8, h: u8) -> bool {
    let mut grid = vec![vec![0; w as usize]; h as usize];
    for Robot {
        position: Coordinates { x, y },
        ..
    } in robots
    {
        grid[*y as usize][*x as usize] += 1;
    }

    let mut count_per_line: Vec<u8> = vec![];
    for line in grid {
        count_per_line.push(line.iter().sum());
    }
    let min = count_per_line.iter().min().unwrap();
    let max = count_per_line.iter().max().unwrap();
    (max - min) < 15
}
