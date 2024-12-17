#[derive(Debug)]
pub struct Computer {
    pub reg_a: u128,
    pub reg_b: u128,
    pub reg_c: u128,
    pub ip: i128,
    pub output: Vec<u8>,
}

impl Computer {
    fn new(reg_a: u128, reg_b: u128, reg_c: u128) -> Computer {
        Computer {
            reg_a,
            reg_b,
            reg_c,
            ip: 0,
            output: vec![],
        }
    }

    fn forward(&mut self, instructions: &Vec<u8>) {
        while (self.ip as usize) <= instructions.len() {
            self.exec(
                instructions[self.ip as usize],
                instructions[self.ip as usize + 1] as u128,
            );
            self.ip += 2;
            if (self.ip as usize) >= instructions.len() {
                break;
            }
        }
    }

    fn exec(&mut self, i: u8, val: u128) {
        let real_val: u128;
        if [0, 2, 5, 6, 7].contains(&i) {
            match val {
                0 | 1 | 2 | 3 => real_val = val,
                4 => real_val = self.reg_a,
                5 => real_val = self.reg_b,
                6 => real_val = self.reg_c,
                _ => todo!(),
            }
        } else {
            real_val = val;
        }
        match i {
            0 => self.adv(real_val),
            1 => self.bxl(real_val),
            2 => self.bst(real_val),
            3 => self.jnz(real_val),
            4 => self.bxc(real_val),
            5 => self.out(real_val),
            6 => self.bdv(real_val),
            7 => self.cdv(real_val),
            _ => todo!(),
        }
    }

    fn formatted_output(&self) -> String {
        return format!(
            "{}",
            self.output
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
    }

    fn adv(&mut self, val: u128) {
        self.reg_a /= 2u128.pow(val as u32);
    }

    fn bxl(&mut self, val: u128) {
        self.reg_b ^= val;
    }

    fn bst(&mut self, val: u128) {
        self.reg_b = val % 8;
    }

    fn jnz(&mut self, val: u128) {
        if self.reg_a != 0 {
            self.ip = val as i128 - 2;
        }
    }

    fn bxc(&mut self, _val: u128) {
        self.reg_b ^= self.reg_c;
    }

    fn out(&mut self, val: u128) {
        self.output.push((val % 8) as u8);
    }

    fn bdv(&mut self, val: u128) {
        self.reg_b = self.reg_a / 2u128.pow(val as u32);
    }

    fn cdv(&mut self, val: u128) {
        self.reg_c = self.reg_a / 2u128.pow(val as u32);
    }
}

fn main() {
    let input: Vec<&str> = std::str::from_utf8(include_bytes!("input"))
        .unwrap()
        .split("\n\n")
        .collect();
    let registers: Vec<u128> = input[0]
        .lines()
        .map(|l| l.split(" ").last().unwrap().parse::<u128>().unwrap())
        .collect();
    let instructions: Vec<u8> = input[1]
        .trim()
        .split(" ")
        .last()
        .unwrap()
        .split(",")
        .map(|i| i.parse::<u8>().unwrap())
        .collect();

    // Part 1
    let mut computer = Computer::new(registers[0], registers[1], registers[2]);
    computer.forward(&instructions);

    println!("Part 1: {}", computer.formatted_output());

    // Part 2
    // The program to run is a loop, dividing reg_a by 3 at each iteration.
    // Thus, only the 3 smaller bits of reg_a are changed during an iteration.
    // We can find the value of reg_a 3 bits at a time, starting from the last instruction.
    // A recursive function is needed to test all possible branches (the optimal solution
    // could fail later).
    let working_a_values: Vec<u128> = (1..=7)
        .map(|i| {
            part_2(
                i,
                registers[0],
                registers[1],
                &instructions,
                instructions.len() as u8 - 1,
            )
        })
        .filter(|x| *x != None)
        .map(|x| x.unwrap())
        .collect::<Vec<u128>>();

    println!("Part 2: {}", working_a_values.iter().min().unwrap());
}

fn part_2(
    reg_a: u128,
    reg_b: u128,
    reg_c: u128,
    instructions: &Vec<u8>,
    depth: u8,
) -> Option<u128> {
    if depth == 0 {
        return Some(reg_a);
    }

    let mut computer = Computer::new(reg_a, reg_b, reg_c);
    computer.forward(&instructions);

    if computer.output != instructions[depth as usize..] {
        return None;
    }

    let valid_reg_a: Vec<Option<u128>> = (0..=7)
        .map(|i| part_2(reg_a * 8 + i, reg_b, reg_c, instructions, depth - 1))
        .filter(|i| *i != None)
        .collect();

    if valid_reg_a.len() == 0 {
        return None;
    }

    return Some(valid_reg_a.iter().map(|a| a.unwrap()).min()?);
}
