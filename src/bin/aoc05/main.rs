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

fn parse_input(input: std::string::String) -> Vec<u32> {

    input.lines()
        .map(|pass| get_seat_id(pass))
        .collect()
}

fn get_seat_id(pass: &str) -> u32 {

    let (row_str, col_str) = pass.split_at(7);

    let row = bin_str_to_int(row_str);
    let col = bin_str_to_int(col_str);

    row * 8 + col
}

fn bin_str_to_int(bin_str: &str) -> u32 {

    bin_str.chars().fold(0, |acc, c| {
        let bit = match c {
            'F' => 0,
            'B' => 1,
            'L' => 0,
            'R' => 1,
            _ => panic!("Unexpected char"),
        };
        (acc << 1) + bit
    })
}

fn main() {
    let start_time = SystemTime::now();
    let seat_ids = parse_input(get_input());

    let setup_time = SystemTime::now();
    let p1 = part_one(&seat_ids);
    let part_1_time = SystemTime::now();
    let p2 = part_two(&mut seat_ids.clone());
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

fn part_one(seat_ids: &Vec<u32>) -> u32 {
    let max_val = seat_ids.iter().max();
    match max_val {
        Some(&max) => max,
        None => panic!( "Vector is empty" ),
    }
}

fn part_two(seat_ids: &mut Vec<u32>) -> u32 {
    seat_ids.sort_unstable();
    let mut prev = seat_ids[0];
    for id in seat_ids {
        if *id - prev > 1 {
            return prev + 1
        }
        prev = *id;
    }
    panic!("No empty seat found");
}


#[cfg(test)]
mod tests {
    use super::get_seat_id;

    #[test]
    fn test_get_row() {
        assert_eq!(get_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(get_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(get_seat_id("BBFFBBFRLL"), 820);
    }
}
