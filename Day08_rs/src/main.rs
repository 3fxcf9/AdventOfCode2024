use std::collections::{HashMap, HashSet};
use std::str;

fn main() {
    let mut freqs: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let input = str::from_utf8(include_bytes!("input")).unwrap();

    input.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            if c != '.' {
                if let Some(a) = freqs.get_mut(&c) {
                    a.push((x as i32, y as i32));
                } else {
                    freqs.insert(c, vec![(x as i32, y as i32)]);
                }
            }
        })
    });

    let h = input.lines().count() as i32;
    let w = input.lines().collect::<Vec<&str>>()[0].chars().count() as i32;

    let antinodes = part1(&freqs, (w, h));
    println!("Part 1: {}", antinodes.iter().count());
    debug_board(input, antinodes);
    let antinodes = part2(&freqs, (w, h));
    println!("Part 2: {}", antinodes.iter().count());
    debug_board(input, antinodes);
}

fn debug_board(input: &str, antinodes: HashSet<(i32, i32)>) {
    let mut board = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    for antinode in &antinodes {
        if board[antinode.1 as usize][antinode.0 as usize] == '.' {
            board[antinode.1 as usize][antinode.0 as usize] = '#';
        } else {
            board[antinode.1 as usize][antinode.0 as usize] = '@';
        }
    }
    let mut str_board: String = String::new();
    for line in &board {
        for c in line {
            str_board.push(*c);
        }
        str_board.push('\n');
    }
    println!("{str_board}");
}

fn part1(freqs: &HashMap<char, Vec<(i32, i32)>>, (w, h): (i32, i32)) -> HashSet<(i32, i32)> {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_, antennas) in freqs.into_iter() {
        for (x1, y1) in antennas {
            for (x2, y2) in antennas {
                if x1 == x2 && y1 == y2 {
                    continue;
                }

                let vect = (x2 - x1, y2 - y1);
                let first = (x2 + vect.0, y2 + vect.1);
                let second = (x1 - vect.0, y1 - vect.1);

                if 0 <= first.0 && 0 <= first.1 && first.1 < h && first.0 < w {
                    antinodes.insert(first);
                }
                if 0 <= second.0 && 0 <= second.1 && second.1 < h && second.0 < w {
                    antinodes.insert(second);
                }
            }
        }
    }

    antinodes
}

fn part2(freqs: &HashMap<char, Vec<(i32, i32)>>, (w, h): (i32, i32)) -> HashSet<(i32, i32)> {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_, antennas) in freqs.into_iter() {
        for (x1, y1) in antennas {
            for (x2, y2) in antennas {
                if x1 == x2 && y1 == y2 {
                    continue;
                }

                let vect = (x2 - x1, y2 - y1);

                let point_in_map = |p: (i32, i32)| (0 <= p.0 && p.0 < w && 0 <= p.1 && p.1 < h);

                let mut look_pos = (*x1, *y1);
                while point_in_map(look_pos) {
                    antinodes.insert(look_pos);
                    look_pos.0 += vect.0;
                    look_pos.1 += vect.1;
                }

                look_pos = (*x1 - vect.0, *y1 - vect.1);
                while point_in_map(look_pos) {
                    antinodes.insert(look_pos);
                    look_pos.0 -= vect.0;
                    look_pos.1 -= vect.1;
                }
            }
        }
    }

    antinodes
}
