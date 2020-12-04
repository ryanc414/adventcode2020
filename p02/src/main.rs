use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let input = load_input();

    let result_1 = count_valid(&input, validate_password_1);
    println!(
        "there are {} valid passwords using the first scheme",
        result_1,
    );

    let result_2 = count_valid(&input, validate_password_2);
    println!(
        "there are {} valid passwords using the second scheme",
        result_2,
    );
}

struct PasswordInput {
    password: String,
    policy_letter: char,
    policy_num_1: usize,
    policy_num_2: usize,
}

fn load_input() -> Vec<PasswordInput> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("please specify input filename");
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Error reading the file");
    let line_re = Regex::new(r"^(\d+)-(\d+) (.): (.+)$").unwrap();

    contents
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| parse_line(line, &line_re))
        .collect()
}

fn parse_line(line: &str, re: &Regex) -> PasswordInput {
    let cap = re.captures(line).unwrap();

    PasswordInput {
        policy_num_1: cap[1].parse().unwrap(),
        policy_num_2: cap[2].parse().unwrap(),
        policy_letter: cap[3].chars().next().unwrap(),
        password: cap[4].to_owned(),
    }
}

fn count_valid(input: &[PasswordInput], validate_password: fn(&PasswordInput) -> bool) -> usize {
    input
        .iter()
        .filter(|&pass_input| validate_password(pass_input))
        .count()
}

fn validate_password_1(pass_input: &PasswordInput) -> bool {
    let letter_count = pass_input
        .password
        .chars()
        .filter(|&letter| letter == pass_input.policy_letter)
        .count();

    letter_count >= pass_input.policy_num_1 && letter_count <= pass_input.policy_num_2
}

fn validate_password_2(pass_input: &PasswordInput) -> bool {
    let letter_1 = match pass_input.password.chars().nth(pass_input.policy_num_1 - 1) {
        Some(val) => val,
        None => return false,
    };
    let letter_2 = match pass_input.password.chars().nth(pass_input.policy_num_2 - 1) {
        Some(val) => val,
        None => return false,
    };

    (letter_1 == pass_input.policy_letter) != (letter_2 == pass_input.policy_letter)
}
