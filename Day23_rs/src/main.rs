use combinations::Combinations;
use std::collections::{HashMap, HashSet};

type Computer = String;
fn main() {
    let links: Vec<(Computer, Computer)> = std::str::from_utf8(include_bytes!("input"))
        .unwrap()
        .lines()
        .map(|l| {
            let (a, b) = l.split_at(l.find("-").unwrap());
            (a.into(), b[1..].into())
        })
        .collect();

    // Part 1: detect 3-cycles
    // Graph representation
    let mut computers: HashMap<Computer, HashSet<Computer>> = HashMap::new();
    for (a, b) in &links {
        if let Some(linked) = computers.get_mut(a) {
            linked.insert(b.clone());
        } else {
            computers.insert(a.clone(), HashSet::from([b.clone()]));
        }
        if let Some(linked) = computers.get_mut(b) {
            linked.insert(a.clone());
        } else {
            computers.insert(b.clone(), HashSet::from([a.clone()]));
        }
    }

    // Strategy: Start at a node and explore the network from it.
    // Detect when we go back to a previously explored one and if yes, compute the cycle length.
    let mut involved_links: HashSet<(Computer, Computer)> = HashSet::new();
    for (start, _) in &computers {
        let mut distances: HashMap<Computer, u8> = HashMap::new();
        distances.insert(start.clone(), 0);
        let mut queue: HashSet<Computer> = HashSet::from([start.clone()]);
        while !queue.is_empty() {
            let current = pop_queue(&mut queue, &distances);
            for neighbor in &computers[&current] {
                let node_distance = *distances.get(&current).unwrap() + 1;
                if distances.contains_key(neighbor) {
                    let cycle_length = &node_distance + distances.get(neighbor).unwrap();
                    if cycle_length == 3 {
                        let mut link = vec![current.clone(), neighbor.clone()];
                        link.sort();
                        involved_links.insert((link[0].clone(), link[1].clone()));
                    }
                    continue;
                }
                distances.insert(neighbor.clone(), node_distance);
                queue.insert(neighbor.clone());
            }
        }
    }

    let mut cycles: HashSet<(Computer, Computer, Computer)> = HashSet::new();
    for (a, b) in involved_links.iter() {
        for linked_to_a in &computers[a] {
            if !(a.starts_with("t") || b.starts_with("t") || linked_to_a.starts_with("t")) {
                continue;
            }
            if !computers[b].contains(linked_to_a) {
                continue;
            }
            let mut cycle = vec![a, b, linked_to_a];
            cycle.sort();
            cycles.insert((cycle[0].clone(), cycle[1].clone(), cycle[2].clone()));
        }
    }

    println!("Part 1: {}", cycles.len());

    // Part 2
    // Strategy: for each node, test all possible combination of connected computers for a dense group,
    // append it in a set and take the bigger

    let mut groups: HashSet<Vec<&Computer>> = HashSet::new();
    for (computer, linked) in &computers {
        // Test for every length...
        for set_len in 2..linked.len() {
            // ... of combinations
            'comb: for linked_comb in
                Combinations::new(linked.iter().collect(), set_len).collect::<Vec<Vec<&Computer>>>()
            {
                let mut computer_set = [vec![computer], linked_comb].concat();

                // Check if computer_set is NOT densely linked
                for c in &computer_set {
                    let others: Vec<&Computer> = computer_set
                        .iter()
                        .filter(|x| *x != c)
                        .map(|x| *x)
                        .collect();

                    if others
                        .iter()
                        .any(|o| !computers.get(*c).unwrap().contains(*o))
                    {
                        continue 'comb;
                    }
                }

                computer_set.sort();
                groups.insert(computer_set);
            }
        }
    }

    let mut bigger_group = groups.iter().max_by_key(|g| g.len()).unwrap().to_owned();
    bigger_group.sort();

    println!(
        "Part 2: {}",
        bigger_group
            .iter()
            .map(|x| x.as_str())
            .collect::<Vec<&str>>()
            .join(",")
    );
}

fn pop_queue(queue: &mut HashSet<Computer>, distances: &HashMap<Computer, u8>) -> Computer {
    let closest = queue
        .iter()
        .enumerate()
        .min_by_key(|(_, i)| distances.get(*i).unwrap())
        .map(|(_, element)| element.clone())
        .unwrap();

    queue.remove(&closest);
    closest
}
