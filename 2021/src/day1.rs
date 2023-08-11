const INPUT: &str = include_str!("1.txt");

pub fn part1() -> String {
    part1_inner(INPUT)
}

pub fn part1_inner(input: &str) -> String {
    input
        .lines()
        .fold((0, None), |(mut n, prev), line| {
            let depth = line.trim().parse::<i64>().unwrap();
            if let Some(prev) = prev {
                if depth > prev {
                    n += 1;
                }
            }
            (n, Some(depth))
        })
        .0
        .to_string()
}

pub fn part2() -> String {
    part2_inner(INPUT)
}

pub fn part2_inner(input: &str) -> String {
    input
        .lines()
        .collect::<Vec<&str>>()
        .windows(3)
        .fold((0, None), |(mut n, prev), lines| {
            let depth = lines
                .iter()
                .fold(0, |sum, line| sum + line.trim().parse::<i64>().unwrap());
            if let Some(prev) = prev {
                if depth > prev {
                    n += 1;
                }
            }
            (n, Some(depth))
        })
        .0
        .to_string()
}

#[test]
fn test_part1() {
    assert_eq!(
        "7",
        part1_inner("199\n200\n208\n210\n200\n207\n240\n269\n260\n263")
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        "5",
        part2_inner("199\n200\n208\n210\n200\n207\n240\n269\n260\n263")
    );
}
