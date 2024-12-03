use colored::Colorize;

fn parse_input(path: &str) -> (Vec<i32>, Vec<i32>) {
    let file = std::fs::read_to_string(path).unwrap();
    let contents = file.lines();

    let mut left_arr: Vec<i32> = vec![];
    let mut right_arr: Vec<i32> = vec![];

    for line in contents {
        let line_processed: Vec<i32> = line
            .split("   ")
            .map(|val| val.parse::<i32>().unwrap())
            .collect();
        left_arr.push(line_processed[0]);
        right_arr.push(line_processed[1]);
    }

    (left_arr, right_arr)
}

fn part_1() {
    let (mut left, mut right) = parse_input("res/2024/d1.txt");

    left.sort();
    right.sort();

    let diff: Vec<i32> = left.iter().zip(right.clone()).map(|(a, b)| a - b).collect();
    let dist: Vec<i32> = diff.iter().map(|d| d.abs()).collect();

    println!(
        "  ‣ {} Total Distance: {}",
        "Part 1".bright_black().bold(),
        dist.iter().sum::<i32>().to_string().bright_green()
    );
}

fn part_2() {
    let (mut left, mut right) = parse_input("res/2024/d1.txt");

    left.sort();
    right.sort();

    let freq: Vec<i32> = left
        .iter()
        .map(|val| {
            let count: usize = right.iter().filter(|&num| num == val).count();
            val * (count as i32)
        })
        .collect();

    println!(
        "  ‣ {} Similarity Score: {}",
        "Part 2".bright_black().bold(),
        freq.iter().sum::<i32>().to_string().bright_green()
    );
}

pub fn run() {
    println!("• {}", "Day 01".bright_blue().bold());
    part_1();
    part_2();
}
