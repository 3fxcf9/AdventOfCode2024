use std::collections::{HashMap, HashSet};

fn main() {
    let max: u64 = 0b111111111111111111111111111111111111111111111;
    let mut input = std::str::from_utf8(include_bytes!("input"))
        .unwrap()
        .split("\n\n");

    let mut wires = HashMap::new();
    input.next().unwrap().lines().for_each(|l| {
        let w = l.split(": ").next().unwrap();
        let v: bool = match l.split(": ").last().unwrap() {
            "0" => false,
            "1" => true,
            _ => todo!(),
        };
        wires.insert(w.to_string(), v);
    });
    let mut gates: HashMap<(String, String), Vec<(String, String)>> = HashMap::new();
    input.next().unwrap().lines().for_each(|l| {
        let parts: Vec<&str> = l.split(" ").collect();
        if gates.contains_key(&(parts[0].to_string(), parts[2].to_string())) {
            let mut new_vec = gates
                .get(&(parts[0].to_string(), parts[2].to_string()))
                .unwrap()
                .clone();
            new_vec.extend([(parts[1].to_string(), parts[4].to_string())]);
            gates.insert((parts[0].to_string(), parts[2].to_string()), new_vec);
        } else {
            gates.insert(
                (parts[0].to_string(), parts[2].to_string()),
                vec![(parts[1].to_string(), parts[4].to_string())],
            );
        }
    });

    // Part 1
    println!("Part 1: {}", circuit(&wires, &gates, None).0);

    // Part 2
    // Not a systematic solution. I went to mermaid and looked a the first incorrect bit shown below.
    
    let exchange = &HashSet::from([
        ("z10".to_string(), "mkk".to_string()),
        ("qbw".to_string(), "z14".to_string()),
        ("wjb".to_string(), "cvp".to_string()),
        ("z34".to_string(), "wcb".to_string()),
    ]);

    // let a = 17767980090787;
    // let b = 19117176970957;
    let a = max;
    let b = 0;
    let result = circuit(&numbers_to_input(a, b, 45), &gates, Some(exchange)).0;
    let result_reversed:String = format!("{result:045b}").chars().rev().collect();
    let expected:String = format!("{:045b}",a+b).chars().rev().collect();

    println!("Part 2:\n{} instead of \n{}", result_reversed, expected);

    let mut to_sort = vec![];
    for (x,y) in exchange {
        to_sort.push(x);
        to_sort.push(y);
    }

    to_sort.sort();
    println!("Part 2 result: {}", to_sort.iter().map(|e| e.as_str()).collect::<Vec<&str>>().join(","));

    println!("{}",export_to_mermaid(&gates));

    // for i in 0..=44 {
    //     let mut a = implied_by_print(format!("z{i:02}").to_string(), &gates, 30)
    //         .into_iter()
    //         .filter(|a| a.starts_with("x") || a.starts_with("y") || a.starts_with("z"))
    //         .collect::<Vec<String>>();
    //     a.sort();
    //     print!("z{i:02} ",);
    //     for b in a {
    //         print!("{b}");
    //     }
    //     println!();
    // }
    //
    // let mut implied_by_hm = HashMap::new();
    // for i in 0..=44 {
    //     implied_by_hm.insert(
    //         format!("z{i:02}").to_string(),
    //         implied_by(format!("z{i:02}").to_string(), &gates),
    //     );
    // }

    // let (out, sorted) = circuit(&numbers_to_input(max, 0, 45), &gates).0;
    // for gate in sorted {
    //     for (gate_name, out) in gate.2 {
    //         let (in1, in2) = if gate.1.starts_with("x") && gate.0.starts_with("y") {
    //             (gate.1.clone(), gate.0.clone())
    //         } else {
    //             (gate.0.clone(), gate.1.clone())
    //         };
    //         println!("{in1} {gate_name} {in2} -> {out}",);
    //     }
    // }

    // let problematic = problematic(&implied_by_hm);
    // let mut a = problematic.clone().into_iter().collect::<Vec<String>>();
    // a.sort();
    // for wire in &a {
    //     println!("{wire} {:?}", implied_by_hm.get(wire).unwrap());
    // }

    // dbg!(&problematic, problematic.iter().count());

    // let to_bruteforce = vec![
    //     implied_by("x14".to_string(), &gates),
    //     implied_by("x15".to_string(), &gates),
    // ]
    // .concat();

    // for swaps in Combinations::new(problematic.iter().collect(), 8) {
    //     swaps
    //     'bruteforce: for i in 0..=44 {
    //         let a = 1 << i;
    //         if circuit(&numbers_to_input(a, 0, 45), &gates, Some(swap)).0 != a {
    //             dbg!(swap);
    //             break 'bruteforce;
    //         }
    //     }
    // }
}

fn export_to_mermaid(gates: &HashMap<(String, String), Vec<(String, String)>>) -> String {
    let mut export = vec!["flowchart LR".to_string()];
    for ((in1, in2), outputs) in gates.iter() {
        for (gate_name, out) in outputs {
            match gate_name.as_str() {
                "AND" => {
                    export.push(format!("{in} -- {gate_name} --> {out}[AND {out}]", in=in1));
                    export.push(format!("{in} -- {gate_name} --> {out}[AND {out}]", in=in2));
                }
                "OR" => {
                    export.push(format!("{in} -- {gate_name} --> {out}([OR {out}])", in=in1));
                    export.push(format!("{in} -- {gate_name} --> {out}([OR {out}])", in=in2));
                }
                "XOR" => {
                    export.push(format!("{in} -- {gate_name} --> {out}[/XOR {out}/]", in=in1));
                    export.push(format!("{in} -- {gate_name} --> {out}[/XOR {out}/]", in=in2));
                }
                _ => todo!(),
            }
        }
    }
    export.join("\n")
}

fn exec_login_gate(gate: String, a: bool, b: bool) -> bool {
    match gate.as_str() {
        "AND" => a & b,
        "OR" => a | b,
        "XOR" => a ^ b,
        _ => todo!(),
    }
}

fn numbers_to_input(a: u64, b: u64, size: u8) -> HashMap<String, bool> {
    let mut wires = number_to_wires(a, "x".to_string(), size);
    wires.extend(number_to_wires(b, "y".to_string(), size));
    wires
}

fn number_to_wires(n: u64, label: String, size: u8) -> HashMap<String, bool> {
    let mut wires = HashMap::new();
    let mut n_copy = n;

    let mut count = 0;
    while n_copy != 0 || count < size {
        wires.insert(
            format!("{label}{count:0>2}", label = label),
            match n_copy % 2 {
                0 => false,
                1 => true,
                _ => todo!(),
            },
        );
        count += 1;
        n_copy /= 2;
    }
    wires
}

fn circuit(
    wires: &HashMap<String, bool>,
    gates: &HashMap<(String, String), Vec<(String, String)>>,
    swap: Option<&HashSet<(String, String)>>,
) -> (u64, Vec<(String, String, Vec<(String, String)>)>) {
    let mut gates_copy = gates.clone();
    let mut wires_copy = wires.clone();
    let mut sorted_gates: Vec<(String, String, Vec<(String, String)>)> = vec![];
    while !gates_copy.is_empty() {
        let gates_iter = gates_copy.clone();
        let mut added_this_loop: Vec<(String, String, Vec<(String, String)>)> = vec![];
        for ((in1, in2), g) in gates_iter {
            if !(wires_copy.contains_key(&in1) && wires_copy.contains_key(&in2)) {
                continue;
            }

            for (gate_name, out) in g.clone() {
                let real_out = if let Some(swap) = swap {
                    if let Some((x, y)) = swap.iter().find(|(x, y)| *x == out || *y == out) {
                        if *x == out {
                            y.clone().to_string()
                        } else {
                            x.clone().to_string()
                        }
                    } else {
                        out
                    }
                } else {
                    out
                };
                let result = exec_login_gate(gate_name, wires_copy[&in1], wires_copy[&in2]);
                wires_copy.insert(real_out, result);
            }
            gates_copy.remove(&(in1.clone(), in2.clone()));
            added_this_loop.push((in1, in2, g));
            added_this_loop.sort();
        }
        sorted_gates.extend(added_this_loop);
    }
    let mut z_wires: Vec<String> = wires_copy
        .clone()
        .into_keys()
        .filter(|k| k.starts_with("z"))
        .collect();

    z_wires.sort_by_key(|e| e[1..].parse::<u8>().unwrap());

    let output = z_wires
        .iter()
        .map(|k| match wires_copy[k] {
            true => "1".to_string(),
            false => "0".to_string(),
        })
        .rev()
        .collect::<Vec<String>>()
        .join("");

    (
        u64::from_str_radix(output.as_str(), 2).unwrap(),
        sorted_gates,
    )
}

// fn problematic(implied_by_hm: &HashMap<String, Vec<String>>) -> HashSet<String> {
//     let mut problematic = HashSet::new();

//     for (wire, wires) in implied_by_hm {
//         let n = wire[1..].parse::<u8>().unwrap();
//         for i in 1..n {
//             if wires.iter().filter(|x| **x == format!("x{i:02}")).count() != 2
//                 || wires.iter().filter(|x| **x == format!("y{i:02}")).count() != 2
//             {
//                 problematic.insert(wire.clone());
//                 break;
//             }
//         }
//         if wires.iter().filter(|x| **x == format!("x{n:02}")).count() != 1
//             || wires.iter().filter(|x| **x == format!("y{n:02}")).count() != 1
//             || wires.iter().filter(|x| **x == format!("x00")).count() != 1
//             || wires.iter().filter(|x| **x == format!("y00")).count() != 1
//         {
//             problematic.insert(wire.clone());
//         }
//     }
//     return problematic;
// }

// fn implied_by_print(
//     gate: String,
//     gates: &HashMap<(String, String), Vec<(String, String)>>,
//     depth: u8,
// ) -> Vec<String> {
//     if gate.starts_with("x") || gate.starts_with("y") {
//         return vec![];
//     }
//     let parents = gates
//         .iter()
//         .filter_map(|g| {
//             for (_, out) in g.1 {
//                 if *out == gate {
//                     return Some((g.0 .0.to_string(), g.0 .1.to_string()));
//                 }
//             }
//             None
//         })
//         .next()
//         .unwrap();
//     let mut ret_vec = vec![];
//     ret_vec.push(if gate.starts_with('z') {
//         format!(
//             "{:>2$}{} dirctely",
//             "",
//             parents.0.clone(),
//             0 * 2 * depth as usize
//         )
//     } else {
//         format!(
//             "{:>2$}{}",
//             "",
//             parents.0.clone(),
//             0 * 2 * depth as usize //gate,
//         )
//     });
//     ret_vec.append(&mut implied_by_print(
//         parents.0.clone(),
//         gates,
//         depth.max(1) - 1,
//     ));
//     ret_vec.push(if gate.starts_with('z') {
//         format!(
//             "{:>2$}{} dirctely",
//             "",
//             parents.1.clone(),
//             0 * 2 * depth as usize
//         )
//     } else {
//         format!(
//             "{:>2$}{}",
//             "",
//             parents.1.clone(),
//             0 * 2 * depth as usize //gate,
//         )
//     });
//     ret_vec.append(&mut implied_by_print(
//         parents.1.clone(),
//         gates,
//         depth.max(1) - 1,
//     ));
//     return ret_vec;
// }

// fn implied_by(
//     gate: String,
//     gates: &HashMap<(String, String), Vec<(String, String)>>,
// ) -> Vec<String> {
//     if gate.starts_with("x") || gate.starts_with("y") {
//         return vec![];
//     }
//     let parents = gates
//         .iter()
//         .filter_map(|g| {
//             for (_, out) in g.1 {
//                 if *out == gate {
//                     return Some((g.0 .0.to_string(), g.0 .1.to_string()));
//                 }
//             }
//             None
//         })
//         .next()
//         .unwrap();
//     let mut ret_vec = vec![];
//     ret_vec.push(parents.0.clone());
//     ret_vec.append(&mut implied_by(parents.0.clone(), gates));
//     ret_vec.push(parents.1.clone());
//     ret_vec.append(&mut implied_by(parents.1.clone(), gates));
//     return ret_vec;
// }
