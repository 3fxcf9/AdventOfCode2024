use std::str;

fn main() {
    let mut list: Vec<(f64, Vec<f64>)> = vec![];
    str::from_utf8(include_bytes!("input"))
        .unwrap()
        .lines()
        .map(|l| l.split(": ").collect::<Vec<&str>>())
        .for_each(|l| {
            list.push((
                l[0].parse::<f64>().unwrap(),
                l[1].split(" ")
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|e| e.parse::<f64>().unwrap())
                    .collect::<Vec<f64>>(),
            ))
        });

    // Part 1
    let mut correct_sum: f64 = 0.0;

    let mut incorrect_lines: Vec<(f64, Vec<f64>)> = vec![];

    for line in list {
        if is_true(line.0, &line.1) {
            correct_sum += line.0;
        } else {
            incorrect_lines.push(line.clone())
        }
    }
    println!("Part 1 : {correct_sum}");

    // Part 2
    for line in incorrect_lines {
        if is_true_2(line.0, &line.1) {
            dbg!("CORRECT", &line);
            correct_sum += line.0;
        }
    }
    println!("Part 2 : {correct_sum}");
}

fn is_true(n: f64, l: &[f64]) -> bool {
    if l.len() == 1 {
        return n == l[0];
    }

    let state = is_true(n - l.last().unwrap(), &l[..l.len() - 1])
        || is_true(n / l.last().unwrap(), &l[..l.len() - 1]);

    return state;
}

fn is_true_2(n: f64, l: &[f64]) -> bool {
    if l.len() == 1 {
        return n == l[0];
    }
    
    let state = is_true_2(n - l.last().unwrap(), &l[..l.len() - 1])
        || is_true_2(n / l.last().unwrap(), &l[..l.len() - 1]);
    
    // Concat if possible
    if n.abs() != f64::INFINITY && n.fract() == 0.0
        && (n as u64) % (10u64.pow(l.last().unwrap().max(1.0).log10().floor() as u32 + 1)) as u64
            == *l.last().unwrap() as u64
    {
        return state
            || is_true_2(
                (n / (10u64.pow(l.last().unwrap().max(1.0).log10().floor() as u32 + 1)) as f64)
                    .floor(),
                &l[..l.len() - 1],
            );
    }
    
    return state;
}
