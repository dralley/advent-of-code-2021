use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
enum Command {
    Up(i32),
    Down(i32),
    Forward(i32),
}

#[derive(Default)]
struct Position {
    depth: i32,
    horizontal: i32,
    aim: i32,
}

fn problem1(commands: &[Command]) -> i32 {
    let mut position = Position::default();

    for command in commands {
        match command {
            Command::Up(val) => position.depth -= val,
            Command::Down(val) => position.depth += val,
            Command::Forward(val) => position.horizontal += val,
        }
    }

    position.depth * position.horizontal
}

fn problem2(commands: &[Command]) -> i32 {
    let mut position = Position::default();

    for command in commands {
        match command {
            Command::Up(val) => position.aim -= val,
            Command::Down(val) => position.aim += val,
            Command::Forward(val) => {
                position.horizontal += val;
                position.depth += position.aim * val;
            },
        }
    }

    position.depth * position.horizontal
}

fn parse_command(line: String) -> Command {
    let mut line_iter = line.split(" ");
    let command_name = line_iter.next().expect("no command?");
    let value = str::parse(line_iter.next().expect("no value?")).expect("unable to parse number");

    match command_name {
        "up" => Command::Up(value),
        "down" => Command::Down(value),
        "forward" => Command::Forward(value),
        _ => panic!("Unexpected command '{}'", command_name),
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    let commands: Vec<Command> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(parse_command)
        .collect();

    println!("Solution 1: {}", problem1(&commands));
    println!("Solution 2: {}", problem2(&commands));
}
