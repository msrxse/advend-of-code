const INPUT: &str = include_str!("2.txt");

enum Command {
    Forward,
    Down,
    Up,
}

impl From<&str> for Command {
    fn from(command: &str) -> Self {
        match command {
            "forward" => Self::Forward,
            "down" => Self::Down,
            "up" => Self::Up,
            _ => panic!("bad input"),
        }
    }
}

pub fn part1() -> String {
    part1_inner(INPUT)
}

pub fn part1_inner(input: &str) -> String {
    let mut horz = 0;
    let mut depth = 0;
    for line in input.lines() {
        let (command, amt) = parse_line(line);
        match command.into() {
            Command::Forward => horz += amt,
            Command::Down => depth += amt,
            Command::Up => depth -= amt,
        }
    }
    (horz * depth).to_string()
}

pub fn part2() -> String {
    part2_inner(INPUT)
}

pub fn part2_inner(input: &str) -> String {
    let mut horz = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in input.lines() {
        let (command, amt) = parse_line(line);
        match command.into() {
            Command::Forward => {
                horz += amt;
                depth += aim * amt;
            }
            Command::Down => aim += amt,
            Command::Up => aim -= amt,
        }
    }
    (horz * depth).to_string()
}

fn parse_line(line: &str) -> (&str, i64) {
    let (command, amt) = line.trim().split_once(' ').unwrap();
    let amt = amt.parse::<i64>().unwrap();
    (command, amt)
}
#[test]
fn test_part1() {
    assert_eq!(
        "150",
        part1_inner(
            "forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2"
        )
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        "900",
        part2_inner(
            "forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2"
        )
    );
}
