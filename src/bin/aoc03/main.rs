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

fn parse_input(input: std::string::String) -> Vec<Vec<Space>> {

    input.lines()
        .map(|s| parse_row(s.to_string()))
        .collect()
}

fn parse_row(row: std::string::String) -> Vec<Space> {

    row.chars().map(|c| {
        match c {
            '.' => Space::Empty,
            '#' => Space::Tree,
            _ => panic!("unexpected character in input")
        }
    })
    .collect()
}

#[derive(Copy, Clone)]
enum Space {
    Empty,
    Tree,
}

type TobogganMap = [Vec<Space>];

#[derive(Copy, Clone)]
struct Slope {
    right: usize,
    down: usize,
}

// Travel all the way from (0,0) to the bottom for a given slope.
// return the number of trees we hit.
fn count_trees_on_slope(map: &TobogganMap, direction: Slope) -> usize {

    let width = map[0].len();

    map.iter()
        .step_by(direction.down)
        .enumerate()
        .filter(|&(step, row)| matches!(row[step * direction.right % width], Space::Tree))
        .count()
}

fn main() {
    let start_time = SystemTime::now();
    let map = parse_input(get_input());

    let setup_time = SystemTime::now();
    let p1 = part_one(&map);
    let part_1_time = SystemTime::now();
    let p2 = part_two(&map);
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

fn part_one(map: &TobogganMap) -> usize {
    count_trees_on_slope(map, Slope{right: 3, down: 1})
}

fn part_two(map: &TobogganMap) -> usize {

    let slopes_to_test = vec![
        Slope{right: 1, down: 1},
        Slope{right: 3, down: 1},
        Slope{right: 5, down: 1},
        Slope{right: 7, down: 1},
        Slope{right: 1, down: 2},
    ];

    slopes_to_test.iter().fold(1, |product, &slope| product * count_trees_on_slope(map, slope))
}


#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::part_one;
    use super::part_two;

    #[test]
    fn test_day_three_part_one() {
        let example_input =
"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

        let map = parse_input(example_input.to_string());
        assert_eq!(part_one(&map), 7);
    }

    #[test]
    fn test_day_three_part_two() {
        let example_input =
"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

        let map = parse_input(example_input.to_string());
        assert_eq!(part_two(&map), 336);
    }
}
