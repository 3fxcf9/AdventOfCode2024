use std::collections::HashSet;

fn main() {
    let blocks: Vec<Vec<&str>> = std::str::from_utf8(include_bytes!("input"))
        .unwrap()
        .split("\n\n")
        .map(|b| b.lines().collect::<Vec<&str>>())
        .collect();

    let mut locks: HashSet<Vec<u8>> = HashSet::new();
    let mut keys: HashSet<Vec<u8>> = HashSet::new();
    let max = blocks[0].len()-2;
    
    for block in &blocks {
        // Lock
        if block[0].starts_with("#") {
            let mut lock: Vec<u8> = vec![];
            for x in 0..block[0].len() {
                lock.push(
                    (1..block.len())
                        .map(|i| block[i].chars().nth(x).unwrap())
                        .position(|e| e == '.')
                        .unwrap() as u8,
                )
            }
            locks.insert(lock);
        }
        // Key
        else if block[0].starts_with(".") {
            let mut key: Vec<u8> = vec![];
            for x in 0..block[0].len() {
                key.push(
                    (0..block.len() - 1)
                        .rev()
                        .map(|i| block[i].chars().nth(x).unwrap())
                        .position(|e| e == '.')
                        .unwrap() as u8,
                )
            }
            keys.insert(key);
        }
    }

    let mut count: u16 = 0;
    for key in &keys {
        'test: for lock in &locks {
            for i in 0..lock.len() {
                if lock[i] + key[i] > max as u8 {
                    continue 'test;
                }
            }
            count += 1;
        }
    }

    println!("Part 1: {count}");
}
