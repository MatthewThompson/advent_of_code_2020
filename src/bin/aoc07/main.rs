use std::env;
use std::fs;
use std::time::SystemTime;
#[macro_use] extern crate lazy_static;
use regex::Regex;

use core::hash;
use std::collections::HashMap;
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

fn parse_input(input: std::string::String) -> HashMap<String, HashSet<BagInfo>> {

    let mut map = HashMap::new();
    for line in input.lines() {
        parse_rule(line, &mut map);
    }
    map
}

fn parse_rule(rule: &str, rule_tree: &mut HashMap<String, HashSet<BagInfo>>) {

    lazy_static! {
        static ref CONTAINER_NAME_REGEX: Regex = Regex::new(r"^((?P<container>[a-z]+ [a-z]+) bags?)").unwrap();
        static ref COLOUR_NAME_REGEX: Regex = Regex::new(r"((?P<number>[1-9]) (?P<colour>[a-z]+ [a-z]+) bags?)").unwrap();
    }

    let container = CONTAINER_NAME_REGEX.captures_iter(rule).next().unwrap().name("container").unwrap().as_str();
    let contained = COLOUR_NAME_REGEX.captures_iter(rule);

    let mut contains: HashSet<BagInfo> = HashSet::new();

    for capture in contained {
        let number = capture.name("number").unwrap().as_str().parse::<u32>().unwrap();
        let colour = capture.name("colour").unwrap().as_str();
        contains.insert(BagInfo {
            colour: colour.to_string(),
            amount: number,
        });
    }

    rule_tree.insert(container.to_string(), contains);
}

#[derive(Debug)]
struct BagInfo {
    colour: String,
    amount: u32,
}

impl hash::Hash for BagInfo {
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        self.colour.hash(hasher)
    }
}

impl PartialEq for BagInfo {
    fn eq(&self, other: &Self) -> bool {
        self.colour == other.colour
    }
}

impl Eq for BagInfo {}

fn main() {
    let start_time = SystemTime::now();
    let rule_tree = parse_input(get_input());

    let setup_time = SystemTime::now();
    let p1 = part_one(&rule_tree);
    let part_1_time = SystemTime::now();
    let p2 = part_two(&rule_tree);
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

fn contains(container: &str, colour: &str, rule_tree: &HashMap<String, HashSet<BagInfo>>) -> bool {

    let root = rule_tree.get(container);

    match root {
        None => false,
        Some(children) => {
            children.iter().fold(false, |acc, next| {
                acc || next.colour == colour || contains(&next.colour, colour, rule_tree)
            })
        }
    }
}

fn get_number_that_contain_colour(colour: &str, rule_tree: &HashMap<String, HashSet<BagInfo>>) -> usize {

    rule_tree.keys().filter(|&item| contains(item, colour, rule_tree)).count()
}

fn part_one(rule_tree: &HashMap<String, HashSet<BagInfo>>) -> usize {

    get_number_that_contain_colour("shiny gold", rule_tree)
}

fn count_bag_contents(root_bag_colour: &str, rule_tree: &HashMap<String, HashSet<BagInfo>>) -> u32 {

    let root = rule_tree.get(root_bag_colour);

    match root {
        None => 0,
        Some(children) => {
            children.iter().fold(0, |acc, next| {
                acc + next.amount + (next.amount * count_bag_contents(&next.colour, rule_tree))
            })
        }
    }
}

fn part_two(rule_tree: &HashMap<String, HashSet<BagInfo>>) -> u32 {
    count_bag_contents("shiny gold", rule_tree)
}


#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::part_one;
    use super::part_two;

    #[test]
    fn test_part_one() {
let example_input = String::from("light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.");

        assert_eq!(part_one(&parse_input(example_input)), 4);
    }

    #[test]
    fn test_part_two() {
        let example_input = String::from("light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.");

        let example_input_two = String::from("shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.");

        assert_eq!(part_two(&parse_input(example_input)), 32);
        assert_eq!(part_two(&parse_input(example_input_two)), 126);
    }
}
