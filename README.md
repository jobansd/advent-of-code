# 🤷‍♂️ About

Advent of Code is an annual event in which programmers around the world complete small puzzles every day in December. Each puzzle is unique, and the difficulty increases progressively as the month goes on. It's a fun way to improve coding skills and engage with the programming community!

You can learn more about Advent of Code [here](https://adventofcode.com/).

# 🏗️ Structure

```txt
advent-of-code-rs
├── crates/
│   ├── 2024/           Advent of Code (2024) solutions
│       └── ...
│   └── common/         Shared library code (ie. utilities)
│       └── ...
├── res/                Problem input files (organized by year)
│   └── 2024/
│       └── *.txt
├── src/
│   └── main.rs         Entry point for the project
├── Cargo.toml
└── README.md
```

# 📦 Technologies

- [colored](https://crates.io/crates/colored) - terminal colors

# 📖 Usage

To run the solutions locally, you can follow these steps:

1. Clone the repository:

```sh
$ git clone https://github.com/jobansd/advent-of-code-rs.git
```

2. Change directory into the project directory

```sh
$ cd advent-of-code
```

3. Run the solutions by invoking the progam with `cargo`.

```sh
$ cargo run
```

# 🤝 Contributions

Feel free to fork this repository and improve upon the solutions.

# 📜 License

This repository is licensed under the [MIT License](https://mit-license.org/). See [LICENSE](LICENSE) for more details.
