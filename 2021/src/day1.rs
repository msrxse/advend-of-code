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
    // start with an input which is a bunch of characters
    input
        // split it into an iterator of lines
        .lines()
        // remove any spaces at the front/back of each line
        .map(str::trim)
        // convert each line from a string to a number
        .map(|line| line.parse::<i64>().unwrap())
        // store all the numbers in a vector
        .collect::<Vec<i64>>()
        // apply a windowing function of size 3
        .windows(3)
        // for each triple, add them up
        .map(|depths| depths.iter().sum())
        // store the sums in a vector
        .collect::<Vec<i64>>()
        // apply a windowing function of 2
        .windows(2)
        // keep only groups where the second number is greater than the first
        .filter(|depths| depths[1] > depths[0])
        // count them
        .count()
        // convert to answer to a string
        .to_string()

    // another way below !!
    // input
    //     .lines()
    //     .collect::<Vec<&str>>()
    //     .windows(3)
    //     .fold((0, None), |(mut n, prev), lines| {
    //         let depth = lines
    //             .iter()
    //             .fold(0, |sum, line| sum + line.trim().parse::<i64>().unwrap());
    //         if let Some(prev) = prev {
    //             if depth > prev {
    //                 n += 1;
    //             }
    //         }
    //         (n, Some(depth))
    //     })
    //     .0
    //     .to_string()
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
