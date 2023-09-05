use std::collections::HashMap;

const INPUT: &str = include_str!("4.txt");

pub fn part1() -> String {
    part1_inner(INPUT)
}

pub fn part1_inner(input: &str) -> String {
    input
        .lines()
        .filter_map(|line: &str| {
            let (first, last) = line.trim().split_once('[').unwrap();
            let mut parts = first.split('-').collect::<Vec<_>>();
            let id = parts.pop().unwrap().parse::<i64>().unwrap();
            let mut freq = HashMap::new();
            for c in parts.into_iter().flat_map(str::chars) {
                *freq.entry(c).or_insert(0) += 1;
            }
            let mut freq = freq.into_iter().collect::<Vec<_>>();
            freq.sort_unstable_by(|a, b| match b.1.cmp(&a.1) {
                std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                std::cmp::Ordering::Equal => a.0.cmp(&b.0),
                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            });
            let check = freq
                .into_iter()
                .take(5)
                .map(|(c, _f)| c)
                .collect::<String>();

            let last = last.strip_suffix(']').unwrap();
            if last == check {
                Some(id)
            } else {
                None
            }
        })
        .sum::<i64>()
        .to_string()
}

pub fn part2() -> String {
    part2_inner(INPUT)
}

pub fn part2_inner(input: &str) -> String {
    "12345".to_owned()
}

#[test]
fn test_part1() {
    assert_eq!(
        "1514",
        part1_inner(
            "aaaaa-bbb-z-y-x-123[abxyz]
             a-b-c-d-e-f-g-h-987[abcde]
             not-a-real-room-404[oarel]
             totally-real-room-200[decoy]"
        )
    );
}

#[test]
fn test_part2() {
    assert_eq!("2", part2_inner("5"));
}
