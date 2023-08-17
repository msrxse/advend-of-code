const INPUT: &str = include_str!("1.txt");

pub fn part1() -> String {
    part1_inner(INPUT)
}

pub fn part1_inner(_input: &str) -> String {
    "12345".to_owned()
}

pub fn part2() -> String {
    part2_inner(INPUT)
}

pub fn part2_inner(_input: &str) -> String {
    "12345".to_owned()
}

#[test]
fn test_part1() {
    assert_eq!(
        "1985",
        part1_inner(
            "ULL
                RRDDD
                LURDL
                UUUUD
            "
        )
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        "1985",
        part2_inner(
            "ULL
                RRDDD
                LURDL
                UUUUD
            "
        )
    );
}
