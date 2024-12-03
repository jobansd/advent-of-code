use aoc_common::utilities;
use colored::Colorize;

fn is_strictly_increasing(arr: &[i32]) -> bool {
    arr.windows(2).all(|win| win[0] < win[1])
}

fn is_strictly_decreasing(arr: &[i32]) -> bool {
    arr.windows(2).all(|win| win[0] > win[1])
}

fn is_monotonic(arr: &[i32]) -> bool {
    is_strictly_increasing(arr) || is_strictly_decreasing(arr)
}

fn is_within_range(arr: &[i32], min: i32, max: i32) -> bool {
    arr.iter().all(|&val| val >= min && val <= max)
}

fn is_valid(arr: &[i32]) -> bool {
    let diff = utilities::diff(arr);
    let dist: Vec<i32> = utilities::abs(&diff);

    is_within_range(&dist, 1, 3) && is_monotonic(arr)
}

fn generate_variants(arr: &[i32]) -> Vec<Vec<i32>> {
    let mut variants_arr = vec![];

    for i in 0..arr.len() {
        let mut variant = vec![];
        for (j, &val) in arr.iter().enumerate() {
            if j != i {
                variant.push(val);
            }
        }
        variants_arr.push(variant);
    }

    variants_arr
}

fn parse_input(path: &str) -> Vec<Vec<i32>> {
    let file = std::fs::read_to_string(path).unwrap();
    let contents = file.lines();

    let mut data: Vec<Vec<i32>> = vec![];

    for line in contents {
        let line_processed: Vec<i32> = line
            .split(' ')
            .map(|val| val.parse::<i32>().unwrap())
            .collect();
        data.push(line_processed);
    }

    data
}

fn part_1() {
    let input = parse_input("res/2024/d2.txt");

    let mut safe_reports: i32 = 0;
    input.iter().for_each(|arr| {
        if is_valid(arr) {
            safe_reports += 1;
        }
    });

    println!(
        "  ‣ {} Safe Reports: {}",
        "Part 1".bright_black().bold(),
        safe_reports.to_string().bright_green()
    );
}

fn part_2() {
    let input = parse_input("res/2024/d2.txt");

    let mut safe_reports: i32 = 0;
    let mut dampened_reports: i32 = 0;
    input.iter().for_each(|arr| {
        if !is_valid(arr) {
            let mut is_safe = false;
            for possibly_safe_report in generate_variants(arr) {
                if !is_safe && is_valid(&possibly_safe_report) {
                    is_safe = true;
                    dampened_reports += 1;
                }
            }
        } else {
            safe_reports += 1;
        }
    });

    println!(
        "  ‣ {} Safe Reports (Dampened): {} {}",
        "Part 2".bright_black().bold(),
        (safe_reports + dampened_reports).to_string().bright_green(),
        format!("({} + {})", safe_reports, dampened_reports).bright_black()
    );
}

pub fn run() {
    println!("• {}", "Day 02".bright_blue().bold());
    part_1();
    part_2();
}
