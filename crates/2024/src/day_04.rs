use colored::Colorize;

fn parse_input(path: &str) -> Vec<Vec<char>> {
    let file = std::fs::read_to_string(path).unwrap();
    let contents = file.lines();

    let mut char_matrix = vec![];
    for line in contents {
        let char_arr: Vec<char> = line.chars().collect();
        char_matrix.push(char_arr);
    }

    char_matrix
}

fn check_right(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

    if x + 3 > grid[y].len() - 1 {
        return false;
    }

    grid[y][x..=x + 3].to_vec() == XMAS
}

fn check_bottom_right(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    if x + 3 > grid[y].len() - 1 || y + 3 > grid.len() - 1 {
        return false;
    }

    grid[y + 1][x + 1] == 'M' && grid[y + 2][x + 2] == 'A' && grid[y + 3][x + 3] == 'S'
}

fn check_down(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    if y + 3 > grid.len() - 1 {
        return false;
    }

    grid[y + 1][x] == 'M' && grid[y + 2][x] == 'A' && grid[y + 3][x] == 'S'
}

fn check_bottom_left(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    if y + 3 > grid.len() - 1 {
        return false;
    }

    match x.checked_sub(3) {
        Some(_) => {
            grid[y + 1][x - 1] == 'M' && grid[y + 2][x - 2] == 'A' && grid[y + 3][x - 3] == 'S'
        }
        None => false,
    }
}

fn check_left(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    const SAMX: [char; 4] = ['S', 'A', 'M', 'X'];

    match x.checked_sub(3) {
        Some(xi) => grid[y][xi..x + 1].to_vec() == SAMX,
        None => false,
    }
}

fn check_top_left(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    match x.checked_sub(3) {
        Some(_) => match y.checked_sub(3) {
            Some(_) => {
                grid[y - 1][x - 1] == 'M' && grid[y - 2][x - 2] == 'A' && grid[y - 3][x - 3] == 'S'
            }
            None => false,
        },
        None => false,
    }
}

fn check_up(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    match y.checked_sub(3) {
        Some(_) => grid[y - 1][x] == 'M' && grid[y - 2][x] == 'A' && grid[y - 3][x] == 'S',
        None => false,
    }
}

fn check_top_right(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    if x + 3 > grid[y].len() - 1 {
        return false;
    }

    match y.checked_sub(3) {
        Some(_) => {
            grid[y - 1][x + 1] == 'M' && grid[y - 2][x + 2] == 'A' && grid[y - 3][x + 3] == 'S'
        }
        None => false,
    }
}

fn search_grid(grid: &[Vec<char>]) -> i32 {
    let mut count = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'X' {
                if check_right(grid, x, y) {
                    count += 1;
                }
                if check_bottom_right(grid, x, y) {
                    count += 1;
                }
                if check_down(grid, x, y) {
                    count += 1;
                }
                if check_bottom_left(grid, x, y) {
                    count += 1;
                }
                if check_left(grid, x, y) {
                    count += 1;
                }
                if check_top_left(grid, x, y) {
                    count += 1;
                }
                if check_up(grid, x, y) {
                    count += 1;
                }
                if check_top_right(grid, x, y) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn check_mmass(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    if x + 1 > grid.len() - 1 || y + 1 > grid.len() - 1 {
        return false;
    }

    match x.checked_sub(1) {
        Some(_) => match y.checked_sub(1) {
            Some(_) => {
                grid[y - 1][x - 1] == 'M'
                    && grid[y - 1][x + 1] == 'M'
                    && grid[y + 1][x - 1] == 'S'
                    && grid[y + 1][x + 1] == 'S'
            }
            None => false,
        },
        None => false,
    }
}

fn check_ssamm(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    if x + 1 > grid.len() - 1 || y + 1 > grid.len() - 1 {
        return false;
    }

    match x.checked_sub(1) {
        Some(_) => match y.checked_sub(1) {
            Some(_) => {
                grid[y - 1][x - 1] == 'S'
                    && grid[y - 1][x + 1] == 'S'
                    && grid[y + 1][x - 1] == 'M'
                    && grid[y + 1][x + 1] == 'M'
            }
            None => false,
        },
        None => false,
    }
}

fn check_smasm(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    if x + 1 > grid.len() - 1 || y + 1 > grid.len() - 1 {
        return false;
    }

    match x.checked_sub(1) {
        Some(_) => match y.checked_sub(1) {
            Some(_) => {
                grid[y - 1][x - 1] == 'S'
                    && grid[y - 1][x + 1] == 'M'
                    && grid[y + 1][x - 1] == 'S'
                    && grid[y + 1][x + 1] == 'M'
            }
            None => false,
        },
        None => false,
    }
}

fn check_msams(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    if x + 1 > grid.len() - 1 || y + 1 > grid.len() - 1 {
        return false;
    }

    match x.checked_sub(1) {
        Some(_) => match y.checked_sub(1) {
            Some(_) => {
                grid[y - 1][x - 1] == 'M'
                    && grid[y - 1][x + 1] == 'S'
                    && grid[y + 1][x - 1] == 'M'
                    && grid[y + 1][x + 1] == 'S'
            }
            None => false,
        },
        None => false,
    }
}

fn search_grid_cross(grid: &[Vec<char>]) -> i32 {
    let mut count = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'A' {
                if check_mmass(grid, x, y) {
                    count += 1;
                }
                if check_ssamm(grid, x, y) {
                    count += 1;
                }
                if check_smasm(grid, x, y) {
                    count += 1;
                }
                if check_msams(grid, x, y) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn part_1() {
    let input = parse_input("res/2024/d4.txt");

    println!(
        "  ‣ {} Total: {}",
        "Part 1".bright_black().bold(),
        search_grid(&input).to_string().bright_green()
    );
}

fn part_2() {
    let input = parse_input("res/2024/d4.txt");

    println!(
        "  ‣ {} Total: {}",
        "Part 2".bright_black().bold(),
        search_grid_cross(&input).to_string().bright_green()
    );
}

pub fn run() {
    println!("• {}", "Day 04".bright_blue().bold());
    part_1();
    part_2();
}
