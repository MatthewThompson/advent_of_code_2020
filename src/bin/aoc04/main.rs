use std::env;
use std::fs;
use std::str::FromStr;
use std::time::SystemTime;
#[macro_use] extern crate lazy_static;
use regex::Regex;

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

fn parse_input(input: std::string::String) -> Vec<Passport> {
    input.split("\n\n")
        .map(parse_passport)
        .collect()
}

fn parse_passport(passport_str: &str) -> Passport {

    let mut passport = Passport::new();

    for field in passport_str.split_ascii_whitespace() {
        let key_and_value: Vec<_> = field.split(':').collect();
        if key_and_value.len() != 2 {
            panic!("Failed to parse passport field.");
        }

        match key_and_value[0] {
            "byr" => passport.byr = Some(key_and_value[1].parse::<u32>().unwrap()),
            "iyr" => passport.iyr = Some(key_and_value[1].parse::<u32>().unwrap()),
            "eyr" => passport.eyr = Some(key_and_value[1].parse::<u32>().unwrap()),
            "hgt" => passport.hgt = Some(Height::from_str(key_and_value[1]).unwrap()),
            "hcl" => passport.hcl = Some(key_and_value[1].to_string()),
            "ecl" => passport.ecl = Some(key_and_value[1].to_string()),
            "pid" => passport.pid = Some(key_and_value[1].to_string()),
            "cid" => passport.cid = Some(key_and_value[1].to_string()),
            _ => panic!("Unknown passport field key."),
        }
    }

    passport
}

enum LengthUnit {
    Centimeter,
    Inch,
}

struct Height {
    value: u32,
    unit: Option<LengthUnit>,
}

impl FromStr for Height {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        lazy_static! {
            static ref HEIGHT_REGEX: Regex = Regex::new(r"(?P<value>[0-9]+)(?P<unit>cm|in|)").unwrap();
        }

        let matches = HEIGHT_REGEX.captures(s).unwrap();

        let value_match = matches.name("value").unwrap().as_str();
        let unit_match = matches.name("unit").unwrap().as_str();

        let unit = match unit_match {
            "cm" => Some(LengthUnit::Centimeter),
            "in" => Some(LengthUnit::Inch),
            _ => None,
        };

        Ok(Height {
            value: value_match.parse::<u32>().unwrap(),
            unit,
        })
    }
}

struct Passport {
    byr: Option<u32>,
    iyr: Option<u32>,
    eyr: Option<u32>,
    hgt: Option<Height>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Default for Passport {
    fn default() -> Self { Passport::new() }
}

impl Passport {
    pub fn new() -> Self {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    pub fn has_required_fields(&self) -> bool {
        self.byr.is_some() &&
        self.iyr.is_some() &&
        self.eyr.is_some() &&
        self.hgt.is_some() &&
        self.hcl.is_some() &&
        self.ecl.is_some() &&
        self.pid.is_some()
    }

    fn byr_valid(&self) -> bool {
        self.byr.map_or(false, |byr| {
            byr >= 1920 && byr <= 2002
        })
    }

    fn iyr_valid(&self) -> bool {
        self.iyr.map_or(false, |iyr| {
            iyr >= 2010 && iyr <= 2020
        })
    }

    fn eyr_valid(&self) -> bool {
        self.eyr.map_or(false, |eyr| {
            eyr >= 2020 && eyr <= 2030
        })
    }

    fn hgt_valid(&self) -> bool {
        self.hgt.as_ref().map_or(false, |hgt| {
            match hgt.unit {
               Some(LengthUnit::Centimeter) => hgt.value >= 150 && hgt.value <= 193,
               Some(LengthUnit::Inch) => hgt.value >= 59 && hgt.value <= 76,
                _ => false,
            }
        })
    }

    fn hcl_valid(&self) -> bool {
        lazy_static! {
            static ref HAIR_COLOUR_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        }

        self.hcl.as_ref().map_or(false, |hcl| {
            HAIR_COLOUR_REGEX.is_match(hcl)
        })
    }

    fn ecl_valid(&self) -> bool {
        lazy_static! {
            static ref EYE_COLOUR_REGEX: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        }

        self.ecl.as_ref().map_or(false, |ecl| {
            EYE_COLOUR_REGEX.is_match(ecl)
        })
    }

    fn pid_valid(&self) -> bool {
        lazy_static! {
            static ref PASSPORT_ID_REGEX: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
        }

        self.pid.as_ref().map_or(false, |pid| {
            PASSPORT_ID_REGEX.is_match(pid)
        })
    }

    pub fn is_valid(&self) -> bool {
        self.byr_valid() &&
        self.iyr_valid() &&
        self.eyr_valid() &&
        self.hgt_valid() &&
        self.hcl_valid() &&
        self.ecl_valid() &&
        self.pid_valid()
    }
}


fn main() {
    let start_time = SystemTime::now();
    let passports = parse_input(get_input());

    let setup_time = SystemTime::now();
    let p1 = part_one(&passports);
    let part_1_time = SystemTime::now();
    let p2 = part_two(&passports);
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

fn part_one(passports: &Vec<Passport>) -> usize {

    passports.iter().filter(|p| p.has_required_fields()).count()
}

fn part_two(passports: &Vec<Passport>) -> usize {

    passports.iter().filter(|p| p.is_valid()).count()
}


#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::part_one;
    use super::part_two;

    #[test]
    fn test_day_four_part_one() {
        let example_input =
String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in");

        assert_eq!(part_one(&parse_input(example_input)), 2);
    }

    #[test]
    fn test_day_four_part_two() {
        let example_invalid =
String::from("eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007");

        let example_valid =
String::from("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719");

        assert_eq!(part_two(&parse_input(example_invalid)), 0);
        assert_eq!(part_two(&parse_input(example_valid)), 4);
    }
}
