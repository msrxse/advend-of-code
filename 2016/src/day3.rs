const INPUT: &str = include_str!("3.txt");

pub fn part1() -> String {
    part1_inner(INPUT)
}

pub fn part1_inner(input: &str) -> String {
    input
        .lines()
        .filter(|line| {
            let lengths = line
                .split_ascii_whitespace()
                .map(str::trim)
                .map(|length| length.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            (lengths[0] + lengths[1] > lengths[2])
                && (lengths[1] + lengths[2] > lengths[0])
                && (lengths[0] + lengths[2] > lengths[1])
        })
        .count()
        .to_string()
}

pub fn part2() -> String {
    part2_inner(INPUT)
}

fn extract_columns(line: &str) -> impl Iterator<Item = i64> + '_ {
    line.split_ascii_whitespace()
        .map(str::trim)
        .map(|length| length.parse::<i64>().unwrap())
}

pub fn part2_inner(input: &str) -> String {
    let mut lines_iter = input.lines();
    let mut n = 0;
    while let (Some(l1), Some(l2), Some(l3)) =
        (lines_iter.next(), lines_iter.next(), lines_iter.next())
    {
        let mut l1 = extract_columns(l1);
        let mut l2 = extract_columns(l2);
        let mut l3 = extract_columns(l3);
        while let (Some(c1), Some(c2), Some(c3)) = (l1.next(), l2.next(), l3.next()) {
            if (c1 + c2 > c3) && (c2 + c3 > c1) && (c1 + c3 > c2) {
                n += 1;
            }
        }
    }
    n.to_string()
}

#[test]
fn test_part1() {
    assert_eq!("2", part1_inner("5 10 25\n5 10 14\n3 4 5"));
}

#[test]
fn test_part2() {
    assert_eq!("2", part2_inner("5 5 3\n10 10 4\n25 14 5"));
}
