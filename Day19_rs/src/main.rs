use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
enum Colors {
    Red,
    Green,
    Blue,
    White,
    Black,
}

fn parse_patterns<'a>(patterns: impl Iterator<Item = &'a str>) -> Vec<Vec<Colors>> {
    patterns
        .map(|p| {
            p.chars()
                .map(|c| match c {
                    'r' => Colors::Red,
                    'g' => Colors::Green,
                    'u' => Colors::Blue,
                    'w' => Colors::White,
                    'b' => Colors::Black,
                    _ => todo!("Unknown color"),
                })
                .collect()
        })
        .collect()
}

fn main() {
    let mut lines = std::str::from_utf8(include_bytes!("input"))
        .unwrap()
        .lines();

    let towels = parse_patterns(lines.next().unwrap().split(", "));

    lines.next(); // Skip empty line
    let designs = parse_patterns(lines);

    let mut count1 = 0;
    let mut count2 = 0;
    for design in designs {
        let mut history = HashMap::new();
        let pos = count_possibilities(&design, &towels, &mut history);
        if pos > 0 {
            count1 += 1;
        }
        count2 += pos;
    }
    println!("Part 1: {count1}");
    println!("Part 2: {count2}");
}

fn count_possibilities(
    design: &Vec<Colors>,
    towels: &Vec<Vec<Colors>>,
    history: &mut HashMap<Vec<Colors>, u64>,
) -> u64 {
    if design.len() == 0 {
        return 1;
    }
    if let Some(d) = history.get(design) {
        return *d;
    }

    let mut count: u64 = 0;
    for towel in towels.iter() {
        if !design.starts_with(&towel) {
            continue;
        }

        count += count_possibilities(&design[towel.len()..].to_vec(), towels, history);
    }
    history.insert(design.to_owned(), count);
    count
}
