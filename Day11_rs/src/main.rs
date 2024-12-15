use memoize::memoize;

fn main() {
    let input: Vec<u64> = std::str::from_utf8(include_bytes!("input"))
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(" ")
        .map(|e| e.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    println!("Part 1 : {}", solve(input.clone(), 25));
    println!("Part 2 : {}", solve(input.clone(), 75));
}

fn solve(input: Vec<u64>, depth: u8) -> u64 {
    let mut count: u64 = 0;
    for stone in input {
        count += count_stones(stone, depth)
    }
    count
}

#[memoize]
fn count_stones(stone: u64, depth: u8) -> u64 {
    return if depth == 0 {
        1
    } else if stone == 0 {
        count_stones(1, depth - 1)
    } else if stone.ilog10() % 2 == 1 {
        let half_digit_count: u32 = (stone.ilog10() + 1) / 2;

        let second = stone % (10u64.pow(half_digit_count));
        let first = (stone - second) / (10u64.pow(half_digit_count as u32));

        count_stones(first, depth - 1) + count_stones(second, depth - 1)
    } else {
        count_stones(stone * 2024, depth - 1)
    };
}
