const INPUT: &str = include_str!("2.txt");
const KEYPAD1: [[char; 3]; 3] = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];
const KEYPAD2: [[char; 5]; 5] = [
    ['X', 'X', '1', 'X', 'X'],
    ['X', '2', '3', '4', 'X'],
    ['5', '6', '7', '8', '9'],
    ['X', 'A', 'B', 'C', 'X'],
    ['X', 'X', 'D', 'X', 'X'],
];

type Dimensions = usize;

pub fn part1() -> String {
    part1_inner(INPUT)
}

pub fn part1_inner(input: &str) -> String {
    let mut x: Dimensions = 1;
    let mut y: Dimensions = 1;

    input
        .lines()
        .map(|line| {
            for step in line.chars() {
                match step {
                    'U' => {
                        y = y.saturating_sub(1);
                    }
                    'D' => {
                        y = 2.min(y + 1);
                    }
                    'L' => {
                        x = x.saturating_sub(1);
                    }
                    'R' => {
                        x = 2.min(x + 1);
                    }
                    _ => panic!("bad input"),
                }
            }
            KEYPAD1[y][x]
        })
        .collect()
}

pub fn part2() -> String {
    part2_inner(INPUT)
}

pub fn part2_inner(input: &str) -> String {
    let mut x: Dimensions = 0;
    let mut y: Dimensions = 2;

    input
        .lines()
        .map(|line| {
            for step in line.chars() {
                match step {
                    'U' => {
                        let ny = y.saturating_sub(1);
                        if KEYPAD2[ny][x] != 'X' {
                            y = ny;
                        }
                    }
                    'D' => {
                        let ny = 4.min(y + 1);
                        if KEYPAD2[ny][x] != 'X' {
                            y = ny;
                        }
                    }
                    'L' => {
                        let nx = x.saturating_sub(1);
                        if KEYPAD2[y][nx] != 'X' {
                            x = nx;
                        }
                    }
                    'R' => {
                        let nx = 4.min(x + 1);
                        if KEYPAD2[y][nx] != 'X' {
                            x = nx;
                        }
                    }
                    _ => panic!("bad input"),
                }
            }
            KEYPAD2[y][x]
        })
        .collect()
}

#[test]
fn test_part1() {
    assert_eq!("1985", part1_inner("ULL\nRRDDD\nLURDL\nUUUUD\n"));
}

#[test]
fn test_part2() {
    assert_eq!("5DB3", part2_inner("ULL\nRRDDD\nLURDL\nUUUUD\n"));
}
