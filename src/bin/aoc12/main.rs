use std::env;
use std::fs;
use std::time::SystemTime;

use std::str::FromStr;

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

fn parse_input(input: std::string::String) -> Vec<NavInstruction> {

    input.lines()
        .map(|line| {
            match NavInstruction::from_str(line) {
                Ok(op) => op,
                Err(message) => panic!(format!("Failed to parse {} with error: {}", line, message)),
            }
        }).collect()
}

#[derive(Debug, Copy, Clone)]
enum TurnAngle {
    Degree90,
    Degree180,
    Degree270,
}

impl FromStr for TurnAngle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        if s.is_empty() {
            return Err("cannot parse empty string as TurnAngle".to_string())
        }

        match s {
            "90" => Ok(TurnAngle::Degree90),
            "180" => Ok(TurnAngle::Degree180),
            "270" => Ok(TurnAngle::Degree270),
            unrecognised => Err(format!("failed to parse value {} as TurnAngle", unrecognised)),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum NavInstruction {
    North(u32),
    South(u32),
    East(u32),
    West(u32),
    Forward(u32),
    Left(TurnAngle),
    Right(TurnAngle),
}

impl FromStr for NavInstruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        if s.is_empty() {
            return Err("cannot parse empty string as NavInstruction".to_string())
        }

        let (action, value) = s.split_at(1);

        match action {
            "N" => Ok(NavInstruction::North(value.parse::<u32>().unwrap())),
            "S" => Ok(NavInstruction::South(value.parse::<u32>().unwrap())),
            "E" => Ok(NavInstruction::East(value.parse::<u32>().unwrap())),
            "W" => Ok(NavInstruction::West(value.parse::<u32>().unwrap())),
            "F" => Ok(NavInstruction::Forward(value.parse::<u32>().unwrap())),
            "L" => Ok(NavInstruction::Left(TurnAngle::from_str(value).unwrap())),
            "R" => Ok(NavInstruction::Right(TurnAngle::from_str(value).unwrap())),
            unrecognised => Err(format!("failed to parse value {} as NavInstruction", unrecognised)),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Ship {
    direction: Direction,
    location_x: i64,
    location_y: i64,
}

impl Ship {
    pub fn new() -> Self {
        Ship {
            direction: Direction::East,
            location_x: 0,
            location_y: 0,
        }
    }

    fn turn_left(&mut self) {

        self.direction = match self.direction {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        };
    }

    fn turn_right(&mut self) {

        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        };
    }

    fn turn_back(&mut self) {

        self.direction = match self.direction {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        };
    }

    fn turn_left_by(&mut self, angle: TurnAngle) {

        match angle {
            TurnAngle::Degree90 => self.turn_left(),
            TurnAngle::Degree180 => self.turn_back(),
            TurnAngle::Degree270 => self.turn_right(),
        }
    }

    fn turn_right_by(&mut self, angle: TurnAngle) {

        match angle {
            TurnAngle::Degree90 => self.turn_right(),
            TurnAngle::Degree180 => self.turn_back(),
            TurnAngle::Degree270 => self.turn_left(),
        }
    }

    fn move_direction(&mut self, direction: Direction, value: u32) {

        match direction {
            Direction::North => self.location_y += value as i64,
            Direction::South => self.location_y -= value as i64,
            Direction::East => self.location_x += value as i64,
            Direction::West => self.location_x -= value as i64,
        }
    }

    pub fn follow_instruction(&mut self, instruction: NavInstruction) {

        match instruction {
            NavInstruction::Forward(val) => self.move_direction(self.direction, val),
            NavInstruction::North(val) => self.move_direction(Direction::North, val),
            NavInstruction::South(val) => self.move_direction(Direction::South, val),
            NavInstruction::East(val) => self.move_direction(Direction::East, val),
            NavInstruction::West(val) => self.move_direction(Direction::West, val),
            NavInstruction::Left(angle) => self.turn_left_by(angle),
            NavInstruction::Right(angle) => self.turn_right_by(angle),
        }
    }
}

struct ShipWithWaypoint {
    location_x: i64,
    location_y: i64,

    waypoint_x: i64,
    waypoint_y: i64,
}

impl ShipWithWaypoint {
    pub fn new() -> Self {
        ShipWithWaypoint {
            location_x: 0,
            location_y: 0,

            waypoint_x: 10,
            waypoint_y: 1,
        }
    }

    fn move_towards_waypoint(&mut self, times: u32) {

        self.location_x += self.waypoint_x * (times as i64);
        self.location_y += self.waypoint_y * (times as i64);
    }

    fn move_waypoint(&mut self, direction: Direction, value: u32) {

        match direction {
            Direction::North => self.waypoint_y += value as i64,
            Direction::South => self.waypoint_y -= value as i64,
            Direction::East => self.waypoint_x += value as i64,
            Direction::West => self.waypoint_x -= value as i64,
        }
    }

    fn rotate_waypoint_90_clockwise(&mut self) {

        let prev_waypoint_x = self.waypoint_x;

        self.waypoint_x = self.waypoint_y;
        self.waypoint_y = -prev_waypoint_x;
    }


    fn rotate_waypoint_180(&mut self) {
        self.waypoint_x = -self.waypoint_x;
        self.waypoint_y = -self.waypoint_y;
    }


    fn rotate_waypoint_90_anti_clockwise(&mut self) {

        let prev_waypoint_y = self.waypoint_y;

        self.waypoint_y = self.waypoint_x;
        self.waypoint_x = -prev_waypoint_y;
    }

    fn rotate_waypoint_anti_clockwise(&mut self, angle: TurnAngle) {

        match angle {
            TurnAngle::Degree90 => self.rotate_waypoint_90_anti_clockwise(),
            TurnAngle::Degree180 => self.rotate_waypoint_180(),
            TurnAngle::Degree270 => self.rotate_waypoint_90_clockwise(),
        }
    }

    fn rotate_waypoint_clockwise(&mut self, angle: TurnAngle) {

        match angle {
            TurnAngle::Degree90 => self.rotate_waypoint_90_clockwise(),
            TurnAngle::Degree180 => self.rotate_waypoint_180(),
            TurnAngle::Degree270 => self.rotate_waypoint_90_anti_clockwise(),
        }
    }

    pub fn follow_instruction(&mut self, instruction: NavInstruction) {

        match instruction {
            NavInstruction::Forward(val) => self.move_towards_waypoint(val),
            NavInstruction::North(val) => self.move_waypoint(Direction::North, val),
            NavInstruction::South(val) => self.move_waypoint(Direction::South, val),
            NavInstruction::East(val) => self.move_waypoint(Direction::East, val),
            NavInstruction::West(val) => self.move_waypoint(Direction::West, val),
            NavInstruction::Left(angle) => self.rotate_waypoint_anti_clockwise(angle),
            NavInstruction::Right(angle) => self.rotate_waypoint_clockwise(angle),
        }
    }
}

fn main() {
    let start_time = SystemTime::now();
    let instructions = parse_input(get_input());

    let setup_time = SystemTime::now();
    let p1 = part_one(&instructions);
    let part_1_time = SystemTime::now();
    let p2 = part_two(&instructions);
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

fn part_one(instructions: &[NavInstruction]) -> u32 {

    let mut ship = Ship::new();
    for &inst in instructions {
        ship.follow_instruction(inst);
    }

    (ship.location_x.abs() + ship.location_y.abs()) as u32
}

fn part_two(instructions: &[NavInstruction]) -> u32 {

    let mut ship = ShipWithWaypoint::new();
    for &inst in instructions {
        ship.follow_instruction(inst);
    }

    (ship.location_x.abs() + ship.location_y.abs()) as u32
}


#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::part_one;
    use super::part_two;

    #[test]
    fn test_day_twelve_part_one() {
let example_input = String::from("F10
N3
F7
R90
F11");

        assert_eq!(part_one(&parse_input(example_input)), 25);
    }

    #[test]
    fn test_day_twelve_part_two() {
        let example_input = String::from("F10
N3
F7
R90
F11");

        assert_eq!(part_two(&parse_input(example_input)), 286);
    }
}
