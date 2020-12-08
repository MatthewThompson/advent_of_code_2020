use std::env;
use std::fs;
use std::time::SystemTime;
use std::collections::HashSet;

fn default_input_path() -> std::path::PathBuf {
    let mut input_path = env::current_dir().unwrap();
    input_path.push("input.txt");

    input_path
}

fn get_input() -> std::string::String {

    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => fs::read_to_string(default_input_path()).expect("Error reading input file"),
        _ => fs::read_to_string(&args[1]).expect("Error reading input file")
    }
}

fn parse_input(input: std::string::String) -> Computer {

    Computer::new(
        input.lines().map(parse_op).collect()
    )
}

// Could refactor to use "FromStr" instead, maybe more rusty
fn parse_op(line: &str) -> Operation {

    let op_and_val: Vec<&str> = line.split(' ').collect();

    let (sign, val) = op_and_val[1].split_at(1);

    let val_unsigned = val.parse::<i32>().unwrap();
    let value = match sign {
        "+" => val_unsigned,
        "-" => -val_unsigned,
        _ => panic!(),
    };

    match op_and_val[0] {
        "nop" => Operation::NOP(value),
        "acc" => Operation::ACC(value),
        "jmp" => Operation::JMP(value),
        _ => panic!(),
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Operation {
    NOP(i32),
    ACC(i32),
    JMP(i32),
}

impl Operation {
    fn execute(&self, cmp: &mut Computer) {
        match self {
            Operation::NOP(_) => cmp.instruction_ptr += 1,
            Operation::ACC(a) => {
                cmp.accumulator += a;
                cmp.instruction_ptr += 1;
            }
            Operation::JMP(j) => cmp.instruction_ptr += j,
        }
    }
}

#[derive(Debug, Clone)]
struct Computer {
    instruction_ptr: i32,
    visited: HashSet<i32>,
    accumulator: i32,
    instructions: Vec<Operation>,
}

// It would make sense to make the computer have this as a field rather
// than repeating them, but I don't want to do that.
#[derive(Debug, Clone)]
struct ComputerState {
    instruction_ptr: i32,
    visited: HashSet<i32>,
    accumulator: i32,
}

#[derive(Debug)]
enum ExitStatus {
    Success,
    InfiniteLoop,
    OutOfBounds,
}

impl Computer {
    pub fn new(instructions: Vec<Operation>) -> Self {
        Computer {
            instruction_ptr: 0,
            visited: HashSet::new(),
            accumulator: 0,
            instructions,
        }
    }

    pub fn run(&mut self) -> ExitStatus {
        loop {
            match self.check_finished() {
                None => self.step(),
                Some(status) => return status,
            }
        }
    }

    // If an instruction set causes an infinite loop, it should be
    // fixable by flipping one of the nop/jmp instructions (or so
    // I've been told)
    pub fn fix_instructions(&mut self) {
        loop {
            let current_op = self.instructions[self.instruction_ptr as usize];

            match current_op {
                Operation::NOP(_) |
                Operation::JMP(_) => {
                    match self.test_flip() {
                        ExitStatus::Success => return,
                        _ => self.step(),
                    }
                },
                _ => self.step(),
            }
        }

    }

    fn check_finished(&self) -> Option<ExitStatus> {

        if self.visited.get(&self.instruction_ptr).is_some() {
            return Some(ExitStatus::InfiniteLoop);
        }

        let len = self.instructions.len() as i32;
        if self.instruction_ptr > len || self.instruction_ptr < 0 {
            return Some(ExitStatus::OutOfBounds);
        }
        if self.instruction_ptr == len {
            return Some(ExitStatus::Success);
        }

        None
    }

    fn step(&mut self) {

        self.visited.insert(self.instruction_ptr);

        let operation = self.instructions[self.instruction_ptr as usize];
        operation.execute(self);
    }

    fn get_state(&self) -> ComputerState {
        ComputerState {
            instruction_ptr: self.instruction_ptr,
            visited: self.visited.clone(),
            accumulator: self.accumulator,
        }
    }

    fn set_state(&mut self, state: ComputerState) {
        self.instruction_ptr = state.instruction_ptr;
        self.visited = state.visited;
        self.accumulator = state.accumulator;
    }

    fn flip_branch(&mut self) {
        let op = &mut self.instructions[self.instruction_ptr as usize];
        *op = match *op {
            Operation::NOP(v) => Operation::JMP(v),
            Operation::JMP(v) => Operation::NOP(v),
            _ => panic!("Attempted to flip an instruction that was not a noop or a jump"),
        }
    }

    fn test_flip(&mut self) -> ExitStatus {
        let snapshot = self.get_state();
        self.flip_branch();

        match self.run() {
            ExitStatus::Success => ExitStatus::Success,
            status => {
                self.set_state(snapshot);
                self.flip_branch();
                status
            }
        }
    }
}


fn main() {
    let start_time = SystemTime::now();
    let mut cmp = parse_input(get_input());

    let setup_time = SystemTime::now();
    let p1 = part_one(&mut cmp.clone());
    let part_1_time = SystemTime::now();
    let p2 = part_two(&mut cmp);
    let part_2_time = SystemTime::now();

    println!();
    println!("The solution for part one is: {}", p1);
    println!("The solution for part two is: {}", p2);
    println!();

    println!("Time breakdowns:");
    println!("Setup: {:?}", setup_time.duration_since(start_time).unwrap());
    println!("Part 1: {:?}", part_1_time.duration_since(setup_time).unwrap());
    println!("Part 2: {:?}", part_2_time.duration_since(part_1_time).unwrap());
    println!("Total: {:?}", part_2_time.duration_since(start_time).unwrap());
}

fn part_one(cmp: &mut Computer) -> i32 {
    match cmp.run() {
        ExitStatus::InfiniteLoop => cmp.accumulator,
        ExitStatus::Success => panic!("Error: expected infinite loop but computer terminated successfully."),
        ExitStatus::OutOfBounds => panic!("Error: expected infinite loop but instruction went out of bounds before computer could terminate."),
    }
}

fn part_two(cmp: &mut Computer) -> i32 {

    cmp.fix_instructions();
    cmp.accumulator
}


#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::part_one;
    use super::part_two;

    #[test]
    fn test_day_eight_part_one() {
let example_input = String::from("nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6");

        assert_eq!(part_one(&mut parse_input(example_input)), 5);
    }

    #[test]
    fn test_day_eight_part_two() {
        let example_input = String::from("nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6");

        assert_eq!(part_two(&mut parse_input(example_input)), 8);
    }
}
