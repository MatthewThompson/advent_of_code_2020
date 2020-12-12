use std::env;
use std::fs;
use std::time::SystemTime;

use std::str::FromStr;
use std::collections::HashSet;
use std::collections::VecDeque;

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

fn parse_input(input: std::string::String) -> Vec<i64> {

    input.lines()
        .map(|line| {
            match i64::from_str(line) {
                Ok(op) => op,
                Err(message) => panic!(format!("Failed to parse {} with error: {}", line, message)),
            }
        }).collect()
}

fn contains_sum(total: i64, values: &HashSet<i64>) -> bool {

    for part in values.iter() {

        if values.get(&(total - part)).is_some() {
            return true
        }
    }

    false
}

struct Xmas {
    preamble_size: usize,
    data: Vec<i64>,
}

impl Xmas {
    pub fn find_number_without_sum(&self) -> i64 {

        let (preamble, rest) = self.data.split_at(self.preamble_size);
        let mut set: HashSet<i64> = preamble.iter().cloned().collect();

        for (i, &next) in rest.iter().enumerate() {

            if !contains_sum(next, &set) {
                return next;
            }

            set.remove(&self.data[i]);
            set.insert(next);
        }

        panic!("no solution");
    }

    pub fn find_contiguous_sum(&self, target: i64) -> i64 {

        let mut sum: i64 = 0;
        let mut queue: VecDeque<i64> = VecDeque::new();

        for &number in self.data.iter() {

            queue.push_back(number);
            sum += number;

            while sum >= target {
                if sum == target  {
                    // I can save a bit of time by keeping track of the min and max
                    // as I go, but I don't think it's worth it
                    return queue.iter().min().unwrap() + queue.iter().max().unwrap()
                }
                else {
                    sum -= queue.pop_front().unwrap();
                }
            }
        }

        panic!("No solution");
    }
}

fn main() {
    let start_time = SystemTime::now();
    let input = parse_input(get_input());

    let setup_time = SystemTime::now();
    let p1 = part_one(&Xmas { preamble_size: 25, data: input.to_vec() });
    let part_1_time = SystemTime::now();
    let p2 = part_two(&Xmas { preamble_size: 25, data: input.to_vec() });
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

// TODO refactor into a struct (XMAS)
fn part_one(xmas: &Xmas) -> i64 {

    xmas.find_number_without_sum()
}

// again, refactor this at some point
fn part_two(xmas: &Xmas) -> i64 {

    let target = xmas.find_number_without_sum();
    xmas.find_contiguous_sum(target)
}


#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::Xmas;
    use super::part_one;
    use super::part_two;

    #[test]
    fn test_day_nine_part_one() {
let example_input = String::from("35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576");

        let xmas = &Xmas { preamble_size: 5, data: parse_input(example_input) };
        assert_eq!(part_one(xmas), 127);
    }

    #[test]
    fn test_day_nine_part_two() {
        let example_input = String::from("35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576");

        let xmas = &Xmas { preamble_size: 5, data: parse_input(example_input) };
        assert_eq!(part_two(xmas), 62);
    }
}
