use std::env;
use std::fs;
use std::time::SystemTime;
use std::hash::Hash;

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

fn parse_input(input: std::string::String) -> Vec<GroupAnswers> {

    input.split("\n\n")
        .map(parse_group_answers)
        .collect()
}

fn parse_group_answers(group_answers: &str) -> GroupAnswers {

    group_answers.lines()
        .map(parse_answers)
        .collect()
}

fn parse_answers(answers: &str) -> Answers {

    answers.chars().collect::<Answers>()
}

type Answers = HashSet<char>;
type GroupAnswers = Vec<Answers>;

fn main() {
    let start_time = SystemTime::now();
    let group_answers = parse_input(get_input());

    let setup_time = SystemTime::now();
    let p1 = part_one(&group_answers);
    let part_1_time = SystemTime::now();
    let p2 = part_two(&group_answers);
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

fn union_sets<T: Eq + Hash + Copy>(sets: &Vec<HashSet<T>>) -> HashSet<T>
{

    sets.iter()
        .fold(HashSet::new(), |set, next| {
            set.union(next).copied().collect()
        })
}

fn intersect_sets<T: Eq + Hash + Copy>(sets: &Vec<HashSet<T>>) -> HashSet<T> {

    sets.iter()
        .skip(1)
        .fold(sets[0].clone(), |set, next| {
            set.intersection(next).copied().collect()
        })
}

fn sum_by<T>(collection: &Vec<T>, operator: &dyn Fn(&T) -> usize) -> usize {

    collection.iter().fold(0, |acc, next| acc + operator(next))
}

fn part_one(group_answers: &Vec<GroupAnswers>) -> usize {

    sum_by(group_answers, &|a| union_sets(a).len())
}

fn part_two(group_answers: &Vec<GroupAnswers>) -> usize {

    sum_by(group_answers, &|a| intersect_sets(a).len())
}


#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::part_one;
    use super::part_two;

    #[test]
    fn test_day_six_part_one() {
        let example_input = String::from("abc

a
b
c

ab
ac

a
a
a
a

b");

        assert_eq!(part_one(&parse_input(example_input)), 11);
    }

    #[test]
    fn test_day_six_part_two() {
        let example_input = String::from("abc

a
b
c

ab
ac

a
a
a
a

b");

        assert_eq!(part_two(&parse_input(example_input)), 6);
    }
}
