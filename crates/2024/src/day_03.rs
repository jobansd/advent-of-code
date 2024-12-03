use colored::Colorize;
use regex::Regex;

fn parse_input(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

fn parse_mul_instructions(text: &str) -> Vec<&str> {
    let re_mul_instruction = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    re_mul_instruction
        .find_iter(text)
        .map(|m| m.as_str())
        .collect()
}

fn parse_instructions(text: &str) -> Vec<&str> {
    let re_instructions = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\)").unwrap();

    re_instructions
        .find_iter(text)
        .map(|m| m.as_str())
        .collect()
}

fn process_mul_instruction(instruction: &str) -> Option<i32> {
    let re_extract_numbers = Regex::new(r"^mul\((\d{1,3}),(\d{1,3})\)$").unwrap();
    match re_extract_numbers.captures(instruction) {
        Some(mul) => {
            let x = mul.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let y = mul.get(2).unwrap().as_str().parse::<i32>().unwrap();
            Some(x * y)
        }
        None => None,
    }
}

fn process_stack(stack: Vec<&str>) -> i32 {
    let re_mul_instruction = Regex::new(r"^mul\(\d{1,3},\d{1,3}\)$").unwrap();
    let re_do_instruction = Regex::new(r"^\bdo\(\)$").unwrap();
    let re_dont_instruction = Regex::new(r"^\bdon't\(\)$").unwrap();

    let mut stop_flag: bool = false;
    let mut sum: i32 = 0;

    stack.iter().for_each(|&instruction| {
        if !stop_flag {
            if re_dont_instruction.is_match(instruction) {
                stop_flag = true;
            }

            if re_mul_instruction.is_match(instruction) {
                sum += process_mul_instruction(instruction).unwrap();
            }
        } else if re_do_instruction.is_match(instruction) {
            stop_flag = false;
        }
    });

    sum
}

fn part_1() {
    let input = parse_input("res/2024/d3.txt");
    let stack = parse_mul_instructions(input.as_str());

    println!(
        "  ‣ {} Total: {}",
        "Part 1".bright_black().bold(),
        process_stack(stack).to_string().bright_green()
    );
}

fn part_2() {
    let input = parse_input("res/2024/d3.txt");
    let stack = parse_instructions(input.as_str());

    println!(
        "  ‣ {} Total: {}",
        "Part 2".bright_black().bold(),
        process_stack(stack).to_string().bright_green()
    );
}

pub fn run() {
    println!("• {}", "Day 03".bright_blue().bold());
    part_1();
    part_2();
}
