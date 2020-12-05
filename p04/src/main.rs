use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;

type ValidatorFn = fn(&str) -> bool;

fn main() {
    let input = load_input();

    let full_validation_rules: HashMap<&str, ValidatorFn> = [
        ("byr", validate_birth_year as ValidatorFn),
        ("iyr", validate_issue_year),
        ("eyr", validate_expiration_year),
        ("hgt", validate_height),
        ("hcl", validate_hair_colour),
        ("ecl", validate_eye_colour),
        ("pid", validate_passport_id),
    ]
    .iter()
    .cloned()
    .collect();

    let simple_validation_rules: HashMap<&str, ValidatorFn> = full_validation_rules
        .iter()
        .map(|(&key, _)| -> (&str, ValidatorFn) { (key, not_empty) })
        .collect();

    let count_1 = count_valid(&input, &simple_validation_rules);
    println!("counted {} valid passports with simple scheme", count_1);

    let count_2 = count_valid(&input, &full_validation_rules);
    println!("counted {} valid passports with full scheme", count_2);
}

fn not_empty(input: &str) -> bool {
    !input.is_empty()
}

fn load_input() -> Vec<HashMap<String, String>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("please specify input filename");
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("error reading the file");

    contents
        .split("\n\n")
        .filter(|line| !line.is_empty())
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> HashMap<String, String> {
    line.split_ascii_whitespace()
        .map(|word| {
            let pair: Vec<&str> = word.split(':').collect();
            if pair.len() != 2 {
                panic!("unexpected length for pair: {}", pair.len())
            }
            (pair[0].to_owned(), pair[1].to_owned())
        })
        .collect()
}

fn count_valid(
    input: &[HashMap<String, String>],
    validation_rules: &HashMap<&str, ValidatorFn>,
) -> usize {
    input
        .iter()
        .filter(|&passport| validate_passport(passport, validation_rules))
        .count()
}

fn validate_passport(
    passport: &HashMap<String, String>,
    rules: &HashMap<&str, ValidatorFn>,
) -> bool {
    rules
        .iter()
        .all(|(&key, validate)| match passport.get(key) {
            Some(val) => validate(val),
            None => false,
        })
}

fn validate_birth_year(input: &str) -> bool {
    if input.len() != 4 {
        return false;
    }

    let birth_year: u32 = match input.parse() {
        Ok(year) => year,
        Err(_) => return false,
    };

    birth_year >= 1920 && birth_year <= 2002
}

fn validate_issue_year(input: &str) -> bool {
    if input.len() != 4 {
        return false;
    }

    let issue_year: u32 = match input.parse() {
        Ok(year) => year,
        Err(_) => return false,
    };

    issue_year >= 2010 && issue_year <= 2020
}

fn validate_expiration_year(input: &str) -> bool {
    if input.len() != 4 {
        return false;
    }

    let expiration_year: u32 = match input.parse() {
        Ok(year) => year,
        Err(_) => return false,
    };

    expiration_year >= 2020 && expiration_year <= 2030
}

enum Unit {
    CM,
    Inches,
}

fn validate_height(input: &str) -> bool {
    let re = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    let (raw_height, raw_unit) = match re.captures(input) {
        Some(cap) => (cap[1].to_owned(), cap[2].to_owned()),
        None => return false,
    };

    let height: u32 = match raw_height.parse() {
        Ok(num) => num,
        Err(_) => return false,
    };

    let unit = match raw_unit.as_str() {
        "cm" => Unit::CM,
        "in" => Unit::Inches,
        _ => panic!("how did this happen"),
    };

    match unit {
        Unit::CM => height >= 150 && height <= 193,
        Unit::Inches => height >= 59 && height <= 76,
    }
}

fn validate_hair_colour(input: &str) -> bool {
    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    re.is_match(input)
}

fn validate_eye_colour(input: &str) -> bool {
    matches!(input, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
}

fn validate_passport_id(input: &str) -> bool {
    let re = Regex::new(r"^\d{9}$").unwrap();
    re.is_match(input)
}
