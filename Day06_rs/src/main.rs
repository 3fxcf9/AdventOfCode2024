use std::collections::HashSet;

#[derive(Debug, PartialEq)]
enum Tile {
    Obstacle,
    Player,
    Empty,
}

#[derive(Debug, Clone)]
struct Guard {
    facing: Facing,
    x: i16,
    y: i16,
}
impl Guard {
    fn turn(&mut self) {
        self.facing = match self.facing {
            Facing::Up => Facing::Right,
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
        }
    }
    fn get_offsets(&self) -> (i16, i16) {
        match self.facing {
            Facing::Up => (0, -1),
            Facing::Right => (1, 0),
            Facing::Down => (0, 1),
            Facing::Left => (-1, 0),
        }
    }
    fn advance(&mut self) {
        let (dx, dy) = self.get_offsets();
        self.x += dx;
        self.y += dy;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}
impl Eq for Facing {}

fn main() {
    let mut initial_x: Option<u8> = None;
    let mut initial_y: Option<u8> = None;
    let map: Vec<Vec<Tile>> = std::str::from_utf8(include_bytes!("input"))
        .unwrap()
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, e)| match e {
                    '#' => Tile::Obstacle,
                    '^' => {
                        initial_x = Some(x as u8);
                        initial_y = Some(y as u8);
                        return Tile::Player;
                    }
                    _ => Tile::Empty,
                })
                .collect()
        })
        .collect();

    let (h, w) = (map.len() as i16, map[0].len() as i16);

    // Part 1

    let mut guard = Guard {
        facing: Facing::Up,
        x: initial_x.unwrap() as i16,
        y: initial_y.unwrap() as i16,
    };

    let mut base_path: HashSet<(i16, i16)> = HashSet::new();

    loop {
        base_path.insert((guard.x, guard.y));
        let (dx, dy) = guard.get_offsets();

        if !(0 <= guard.x + dx && guard.x + dx < w && 0 <= guard.y + dy && guard.y + dy < h) {
            break;
        };
        if map[(guard.y + dy) as usize][(guard.x + dx) as usize] == Tile::Obstacle {
            guard.turn();
        };
        guard.advance();
    }

    println!("Part 1: {}", base_path.len());

    // Part 2

    let mut count = 0;
    for (x, y) in base_path.iter() {
        let mut visited: HashSet<(i16, i16, Facing)> = HashSet::new();

        let mut guard = Guard {
            facing: Facing::Up,
            x: initial_x.unwrap() as i16,
            y: initial_y.unwrap() as i16,
        };

        'walk: loop {
            if visited.contains(&(guard.x, guard.y, guard.facing)) {
                count += 1;
                break;
            }

            visited.insert((guard.x, guard.y, guard.facing));

            loop {
                let (dx, dy) = guard.get_offsets();

                if !(0 <= guard.x + dx && guard.x + dx < w && 0 <= guard.y + dy && guard.y + dy < h)
                {
                    break 'walk;
                };

                if (map[(guard.y + dy) as usize][(guard.x + dx) as usize] == Tile::Obstacle)
                    || (guard.x + dx == *x && guard.y + dy == *y)
                {
                    guard.turn(); // Turn until free orientation found
                } else {
                    break; // Orientation found
                };
            }
            guard.advance();
        }
    }

    println!("Part 2: {}", count);
}
