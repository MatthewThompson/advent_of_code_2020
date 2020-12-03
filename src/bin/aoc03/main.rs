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

fn parse_input(input: std::string::String) -> TobogganMap {

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

type TobogganMap = Vec<Vec<Space>>;

#[derive(Copy, Clone)]
struct Slope {
    right: usize,
    down: usize,
}

// Travel all the way from (0,0) to the bottom for a given slope.
// return thenumber of trees we hit.
fn count_trees_on_slope(map: &TobogganMap, direction: Slope) -> usize {

    let width = map[0].len();

    map.iter()
        .step_by(direction.down)
        .enumerate()
        .filter(|&(step, row)| matches!(row[step * direction.right % width], Space::Tree))
        .count()
}

fn main() {
    let map = parse_input(get_input());

    println!("The solution for part one is: {}", part_one(&map));
    println!("The solution for part two is: {}", part_two(&map));
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
    fn test_part_one() {
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
    fn test_part_two() {
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
