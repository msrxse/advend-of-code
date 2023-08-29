const INPUT: &str = include_str!("3.txt");

pub fn part1() -> String {
    part1_inner(INPUT)
}

pub fn part1_inner(input: &str) -> String {
    let num_lines = input.lines().count();
    let (gamma, num_cols) = {
        let counters = input
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|char| char != '0')
                    .collect::<Vec<_>>()
            })
            .fold(Vec::new(), |mut counters, line| {
                counters.resize(counters.len().max(line.len()), 0);
                for (counter, digit) in counters.iter_mut().zip(line) {
                    if digit {
                        *counter += 1
                    }
                }
                counters
            })
            .into_iter();
        let num_cols = counters.len();
        (
            counters.fold(0, |value, counter| {
                (value << 1)
                    + match num_lines.cmp(&(counter * 2)) {
                        std::cmp::Ordering::Less => 1,
                        std::cmp::Ordering::Equal => panic!("bad input"),
                        std::cmp::Ordering::Greater => 0,
                    }
            }),
            num_cols,
        )
    };
    (gamma * (!gamma & ((1 << num_cols) - 1))).to_string()
}

pub fn part2() -> String {
    part2_inner(INPUT)
}

pub fn part2_inner(input: &str) -> String {
    let numbers = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|char| char != '0')
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let o2 = extract_number(numbers.clone(), |one_bits: usize, total_bits| {
        one_bits * 2 >= total_bits
    });
    let co2 = extract_number(numbers, |one_bits, total_bits| one_bits * 2 < total_bits);
    (o2 * co2).to_string()
}

fn extract_number(mut numbers: Vec<Vec<bool>>, criteria: fn(usize, usize) -> bool) -> i32 {
    for pos in 0..numbers[0].len() {
        if numbers.len() == 1 {
            break;
        }
        number_filter(&mut numbers, pos, criteria);
    }
    numbers[0]
        .iter()
        .fold(0, |value, &bit| (value << 1) + if bit { 1 } else { 0 })
}

fn number_filter(numbers: &mut Vec<Vec<bool>>, pos: usize, criteria: fn(usize, usize) -> bool) {
    let num_ones = numbers
        .iter()
        .fold(0, |n, number| if number[pos] { n + 1 } else { n });
    let bit = criteria(num_ones, numbers.len());
    numbers.retain(|number| number[pos] == bit);
}

#[test]
fn test_part1() {
    assert_eq!(
        "198",
        part1_inner(
            "00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010"
        )
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        "230",
        part2_inner(
            "00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010"
        )
    );
}
