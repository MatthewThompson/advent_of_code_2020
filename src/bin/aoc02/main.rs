use std::env;
use std::fs;
use std::time::SystemTime;

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

fn parse_input(input: std::string::String) -> Vec<DatabasePassword> {
    input.lines()
        .map(|s| parse_line_as_password(s))
        .collect()
}

fn parse_line_as_password(line: &str) -> DatabasePassword {
    let policy_and_pass: Vec<_> = line.split(':').collect();

    if policy_and_pass.clone().len() != 2 {
        panic!("Invalid password entry");
    }

    DatabasePassword {
        policy: parse_str_as_policy(policy_and_pass[0]),
        password: policy_and_pass[1].trim().to_string(),
    }
}

fn parse_str_as_policy(s: &str) -> Policy {
    let minmax_and_char: Vec<_> = s.split(' ').collect();

    if minmax_and_char.len() != 2 {
        panic!("Invalid policy entry");
    }

    let minmax = minmax_and_char[0];
    let policy_char: Vec<_> = minmax_and_char[1].chars().collect();

    if policy_char.len() != 1 || !policy_char[0].is_alphabetic() {
        panic!("Invalid policy letter");
    }

    let min_max: Vec<_> = minmax.split('-').collect();

    if min_max.len() != 2 {
        panic!("Invalid policy entry, could not pass min and max vals");
    }
    let min = usize::from_str_radix(min_max[0], 10).expect("Failed to parse policy minimum.");
    let max = usize::from_str_radix(min_max[1], 10).expect("Failed to parse policy maximum.");

    Policy {
        min,
        max,
        letter: policy_char[0],
    }
}

fn main() {
    let start_time = SystemTime::now();
    let values = parse_input(get_input());

    let setup_time = SystemTime::now();
    let p1 = part_one(&values);
    let part_1_time = SystemTime::now();
    let p2 = part_two(&values);
    let part_2_time = SystemTime::now();

    println!("The solution for part one is: {}", p1);
    println!("The solution for part two is: {}", p2);
    println!();

    println!("Time breakdowns:");
    println!("Setup: {:?}", setup_time.duration_since(start_time).unwrap());
    println!("Part 1: {:?}", part_1_time.duration_since(setup_time).unwrap());
    println!("Part 2: {:?}", part_2_time.duration_since(part_1_time).unwrap());
    println!("Total: {:?}", part_2_time.duration_since(start_time).unwrap());
}

struct Policy {
    min: usize,
    max: usize,
    letter: char,
}

struct DatabasePassword {
    password: std::string::String,
    policy: Policy,
}

fn part_one(passwords: &Vec<DatabasePassword>) -> usize {

    passwords.iter().filter(|p| check_password_validity_one(p)).count()
}

fn part_two(passwords: &Vec<DatabasePassword>) -> usize {

    passwords.iter().filter(|p| check_password_validity_two(p)).count()
}

fn check_password_validity_one(pass: &DatabasePassword) -> bool {

    let matching_chars = pass.password.chars().filter(|c| *c == pass.policy.letter).count();
    matching_chars >= pass.policy.min && matching_chars <= pass.policy.max
}

fn check_password_validity_two(pass: &DatabasePassword) -> bool {

    let first_index = pass.policy.min - 1;
    let second_index = pass.policy.max - 1;

    let policy_letter = pass.policy.letter;

    let password_chars: Vec<_> = pass.password.chars().collect();

    (password_chars[first_index] == policy_letter) ^
        (password_chars[second_index] == policy_letter)
}

#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::part_one;
    use super::part_two;

    #[test]
    fn test_day_two_part_one() {
        let example_input =
"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

        let db = parse_input(example_input.to_string());
        assert_eq!(part_one(&db), 2);
    }

    #[test]
    fn test_day_two_part_two() {
        let example_input =
"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

        let db = parse_input(example_input.to_string());
        assert_eq!(part_two(&db), 1);
    }
}
