use memoize::memoize;
use std::{
    sync::{Arc, Mutex},
    thread, u64,
};

fn main() {
    let input: Vec<u64> = std::str::from_utf8(include_bytes!("input"))
        .unwrap()
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect();

    // Part 1
    let mut input_1 = input.clone();
    for _ in 0..2000 {
        input_1.iter_mut().for_each(|n| *n = next(*n));
    }
    let sum: u64 = input_1.iter().sum();
    println!("Part 1 : {sum}");

    // dbg!(next_delta_i(123, 10));

    // Part 2
    let maximums: Arc<Mutex<(u64, (i8, i8, i8, i8))>> = Arc::new(Mutex::new((0, (0, 0, 0, 0))));
    let mut handles = vec![];
    for d1 in 0..10 {
        let maximums_clone = maximums.clone();
        let input_clone = input.clone();
        handles.push(thread::spawn(move || {
            for d2 in 0..10 {
                for d3 in 0..10 {
                    for d4 in 0..10 {
                        for signs in 0..0b1111i8 {
                            let mut total_price: u64 = 0;
                            let sign1 = ((signs ^ 0b1000) >> 3) % 2 * 2 - 1;
                            let sign2 = ((signs ^ 0b0100) >> 2) % 2 * 2 - 1;
                            let sign3 = ((signs ^ 0b0010) >> 1) % 2 * 2 - 1;
                            let sign4 = (signs ^ 0b0001) % 2 * 2 - 1;
                            let sequence = (sign1 * d1, sign2 * d2, sign3 * d3, sign4 * d4);

                            for secret in &input_clone {
                                let secrets: Vec<(u8, i8)> = next_delta_i(*secret, 2000);
                                if let Some((i, _)) = secrets[..secrets.len() - 3]
                                    .iter()
                                    .enumerate()
                                    .find(|(i, _)| {
                                        secrets[*i].1 == sequence.0
                                            && secrets[*i + 1].1 == sequence.1
                                            && secrets[*i + 2].1 == sequence.2
                                            && secrets[*i + 3].1 == sequence.3
                                    })
                                {
                                    total_price += secrets[i + 3].0 as u64;
                                }
                            }
                            let mut maximums = maximums_clone.lock().unwrap();
                            if total_price > maximums.0 {
                                *maximums = (total_price, sequence);
                            }
                        }
                    }
                }
            }
        }));
    }
    for handle in handles {
        thread::JoinHandle::join(handle).unwrap();
    }
    println!("Part 2 : {}", maximums.lock().unwrap().0);
}

#[memoize]
fn next(n: u64) -> u64 {
    let a = ((n << 6) ^ n) & 16777215;
    let b = ((a >> 5) ^ a) & 16777215;
    return ((b << 11) ^ b) & 16777215;
}

#[memoize]
fn next_delta(n: u64) -> (u64, i8) {
    let a = ((n << 6) ^ n) & 16777215;
    let b = ((a >> 5) ^ a) & 16777215;
    let c = ((b << 11) ^ b) & 16777215;
    (c, (c % 10) as i8 - (n % 10) as i8)
}

#[memoize]
fn next_delta_i(n: u64, i: u16) -> Vec<(u8, i8)> {
    let mut deltas = vec![(0, 0); i as usize];
    let mut current_secret = n;
    for _i in 0..i {
        let (a, b) = next_delta(current_secret);
        deltas[_i as usize] = ((a % 10) as u8, b);
        current_secret = a;
    }
    deltas
}
