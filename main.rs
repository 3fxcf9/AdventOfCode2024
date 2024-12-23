use std::{cmp::Ordering, collections::HashMap, fmt::Debug, hash::Hash, u128};

#[derive(Debug, Clone, Copy)]
enum RobotActions {
    Up,
    Left,
    Down,
    Right,
    Activate,
}

trait Keypad: Clone + Copy + Debug + Hash + Eq {
    fn get_coordinates(&self) -> (u8, u8);
    // fn empty_coordinates() -> (u8, u8);
}

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
enum NumericalKeys {
    Key(u8),
    #[default]
    Activate,
}
impl NumericalKeys {
    fn empty_coordinates() -> (u8, u8) {
        (2, 3)
    }
}
impl Keypad for NumericalKeys {
    fn get_coordinates(&self) -> (u8, u8) {
        match self {
            Self::Key(7) => (2, 0),
            Self::Key(8) => (1, 0),
            Self::Key(9) => (0, 0),
            Self::Key(4) => (2, 1),
            Self::Key(5) => (1, 1),
            Self::Key(6) => (0, 1),
            Self::Key(1) => (2, 2),
            Self::Key(2) => (1, 2),
            Self::Key(3) => (0, 2),
            Self::Key(0) => (1, 3),
            Self::Activate => (0, 3),
            _ => todo!(),
        }
    }
    // fn empty_coordinates() -> (u8, u8) {
    //     (2, 3)
    // }
}

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
enum DirectionalKeys {
    Up,
    Left,
    Right,
    Down,
    #[default]
    Activate,
}
impl DirectionalKeys {
    fn empty_coordinates() -> (u8, u8) {
        (2, 0)
    }
}
impl Keypad for DirectionalKeys {
    fn get_coordinates(&self) -> (u8, u8) {
        match self {
            Self::Up => (1, 0),
            Self::Activate => (0, 0),
            Self::Left => (2, 1),
            Self::Down => (1, 1),
            Self::Right => (0, 1),
        }
    }
    // fn empty_coordinates() -> (u8, u8) {
    //     (2, 0)
    // }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct NumericalRobot {
    position: NumericalKeys,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct DirectionalRobot {
    position: DirectionalKeys,
}

trait Robot: Clone + Copy + Debug + Hash + Eq {
    type KeypadType: Keypad;
    fn get_position(&self) -> Self::KeypadType;
    fn get_shortest_path_options(&self, to: Self::KeypadType) -> Vec<Vec<RobotActions>> {
        let (from_x, from_y) = self.get_position().get_coordinates();
        let (to_x, to_y) = to.get_coordinates();
        let (dx, dy): (i8, i8) = (to_x as i8 - from_x as i8, to_y as i8 - from_y as i8);

        let lateral_move = match dx.cmp(&0) {
            Ordering::Greater | Ordering::Equal => RobotActions::Left,
            Ordering::Less => RobotActions::Right,
        };
        let vertical_move = match dy.cmp(&0) {
            Ordering::Greater | Ordering::Equal => RobotActions::Down,
            Ordering::Less => RobotActions::Up,
        };

        // Return only one option if one of dy or dx is zero
        if (to_x, from_y) == Self::KeypadType::empty_coordinates() || dy * dx == 0 {
            vec![[
                vec![vertical_move; dy.abs() as usize],
                vec![lateral_move; dx.abs() as usize],
            ]
            .concat()]
        } else if (from_x, to_y) == Self::KeypadType::empty_coordinates() {
            vec![[
                vec![lateral_move; dx.abs() as usize],
                vec![vertical_move; dy.abs() as usize],
            ]
            .concat()]
        } else {
            vec![
                [
                    vec![lateral_move.clone(); dx.abs() as usize],
                    vec![vertical_move.clone(); dy.abs() as usize],
                ]
                .concat(),
                [
                    vec![vertical_move; dy.abs() as usize],
                    vec![lateral_move; dx.abs() as usize],
                ]
                .concat(),
            ]
        }
    }
    fn get_options_for_keypress(&self, key: Self::KeypadType) -> Vec<Vec<RobotActions>> {
        self.get_shortest_path_options(key)
            .iter()
            .map(|path| vec![path.clone(), vec![RobotActions::Activate]].concat())
            .collect()
    }
    /// Static method converting actions planned by a robot into the keys the robot controlling it needs to press.
    fn actions_to_keys(action: RobotActions) -> DirectionalKeys {
        match action {
            RobotActions::Up => DirectionalKeys::Up,
            RobotActions::Left => DirectionalKeys::Left,
            RobotActions::Down => DirectionalKeys::Down,
            RobotActions::Right => DirectionalKeys::Right,
            RobotActions::Activate => DirectionalKeys::Activate,
        }
    }
}

impl Robot for NumericalRobot {
    type KeypadType = NumericalKeys;
    fn get_position(&self) -> Self::KeypadType {
        self.position
    }
}
impl Robot for DirectionalRobot {
    type KeypadType = DirectionalKeys;
    fn get_position(&self) -> Self::KeypadType {
        self.position
    }
}

fn main() {
    let codes = std::str::from_utf8(include_bytes!("input"))
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    c if c.is_ascii_digit() => NumericalKeys::Key(c.to_digit(10).unwrap() as u8),
                    'A' => NumericalKeys::Activate,
                    _ => todo!(),
                })
                .collect::<Vec<NumericalKeys>>()
        })
        .collect::<Vec<Vec<NumericalKeys>>>();

    // Part 1
    println!("Part 1: {}", solve_puzzle(&codes, 3));
    println!("Part 2: {}", solve_puzzle(&codes, 13));
}

fn solve_puzzle(codes: &Vec<Vec<NumericalKeys>>, depth: u8) -> u128 {
    let mut history: HashMap<(NumericalRobot, NumericalKeys), u128> = HashMap::new();
    let mut count: u128 = 0;
    for code in codes {
        let mut code_count = 0;
        let mut numeric_part: u16 = 0;
        for (i, key) in code.iter().enumerate() {
            if let NumericalKeys::Key(n) = key {
                numeric_part *= 10;
                numeric_part += *n as u16;
            }
            let robot = if i == 0 {
                NumericalRobot::default()
            } else {
                NumericalRobot {
                    position: code[i - 1],
                }
            };
            code_count += count_moves(robot, *key, depth, &mut history);
        }
        count += code_count * numeric_part as u128;
    }
    count
}
impl Robot for RobotType {
    type KeypadType = Box<dyn Keypad>; // This is fine, because Box<dyn Keypad> is object-safe

    fn get_position(&self) -> Self::KeypadType {
        match *self {
            RobotType::Numerical(ref robot) => Box::new(robot.position),
            RobotType::Directional(ref robot) => Box::new(robot.position),
        }
    }
    
    // Add the other methods as needed
}
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum RobotType {
    Numerical(NumericalRobot),
    Directional(DirectionalRobot),
}
fn count_moves<R>(
    robot: R,
    key_to_press: R::KeypadType,
    depth: u8,
    history: &mut HashMap<(Box<dyn Robot>, Box<dyn Keypad>), u128>,
) -> u128
where
    R: Robot + 'static,
{
    let robot_box = Box::new(robot) as Box<dyn Robot>;
    let key_box = Box::new(key_to_press) as Box<dyn Keypad>;

    if let Some(seen) = history.get(&(robot_box.clone(), key_box.clone())) {
        return *seen;
    }

    // fn count_moves<R: Robot<KeypadType = impl Keypad>>(
    //     robot: R,
    //     key_to_press: R::KeypadType,
    //     depth: u8,
    //     history: &mut HashMap<(Box<dyn Robot>, Box<dyn Keypad>), u128>,
    // ) -> u128 {
    //     if let Some(seen) = history.get(&(robot, key_to_press)) {
    //         return *seen;
    //     }
    if depth == 0 {
        return 1;
    }
    let options = robot.get_options_for_keypress(key_to_press);

    let mut current_mini: u128 = u128::MAX;
    for option in options {
        let mut move_count = 0;
        for (i, m) in option.iter().enumerate() {
            let new_robot = if i == 0 {
                DirectionalRobot::default()
            } else {
                DirectionalRobot {
                    position: DirectionalRobot::actions_to_keys(option[i - 1]),
                }
            };
            move_count += count_moves(
                new_robot,
                DirectionalRobot::actions_to_keys(*m),
                depth - 1,
                todo!(),
            );
        }
        if move_count < current_mini {
            current_mini = move_count
        }
    }
    current_mini
}
