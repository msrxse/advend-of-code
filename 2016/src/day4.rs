const INPUT: &str = include_str!("4.txt");

pub fn part1() -> String {
    part1_inner(INPUT)
}

pub fn part1_inner(input: &str) -> String {
    "12345".to_owned()
}

pub fn part2() -> String {
    part2_inner(INPUT)
}

pub fn part2_inner(input: &str) -> String {
    "12345".to_owned()
}

#[test]
fn test_part1() {
    assert_eq!("2", part1_inner("5"));
}

#[test]
fn test_part2() {
    assert_eq!("2", part2_inner("5"));
}
