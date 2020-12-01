use std::env;
use std::fs;

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
    input.split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

fn main() {
    let values = parse_input(get_input());

    part_one(values);
}

fn part_one(values: Vec<i64>) -> i64 {

    let (a, b) = find_sum_parts(values, 2020);
    let solution = a * b;

    println!("The solution to part 1 is {} ({} * {}).", solution, a, b);

    solution
}

fn find_sum_parts(values: Vec<i64>, total: i64) -> (i64, i64) {

    let mut sorted = values;
    sorted.sort_unstable();

    let mut low_index = 0;
    let mut high_index = sorted.len() - 1;

    while low_index < high_index {

        let low = sorted[low_index];
        let high = sorted[high_index];
        let sum = low + high;

        match sum {
            s if s < total => { low_index += 1; }
            s if s > total => { high_index -= 1; }
            _ => return (low, high),
        }
    }

    panic!("No solution.");
}


#[cfg(test)]
mod tests {
    use super::part_one;

    #[test]
    fn test_part_one() {
        let example_values: Vec<i64> = vec![
            1721,
            979,
            366,
            299,
            675,
            1456,
        ];

        assert_eq!(part_one(example_values), 514579);
    }
}
