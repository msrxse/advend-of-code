use core::panic;

const INPUT: &str = include_str!("4.txt");
const BINGO_BOARD_SIZE: usize = 5;

type Cell = (i32, bool);
type Row = [Cell; BINGO_BOARD_SIZE];
type Board = [Row; BINGO_BOARD_SIZE];
type Boards = Vec<Board>;

fn check_bingo(board: &[Row]) -> bool {
    for n in 0..BINGO_BOARD_SIZE {
        let mut r = 0;
        let mut c = 0;
        for m in 0..BINGO_BOARD_SIZE {
            if board[n][m].1 {
                r += 1
            }
            if board[m][n].1 {
                c += 1
            }
        }
        if r == BINGO_BOARD_SIZE || c == BINGO_BOARD_SIZE {
            return true;
        }
    }
    false
}

struct LineIterator<'a, Lines: Iterator<Item = &'a str>> {
    lines: Lines,
}

impl<'a, Lines: Iterator<Item = &'a str>> LineIterator<'a, Lines> {
    fn new(lines: Lines) -> Self {
        Self { lines }
    }
}

impl<'a, Lines: Iterator<Item = &'a str>> Iterator for LineIterator<'a, Lines> {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        Some([
            make_row(self.lines.next()?),
            make_row(self.lines.next().unwrap()),
            make_row(self.lines.next().unwrap()),
            make_row(self.lines.next().unwrap()),
            make_row(self.lines.next().unwrap()),
        ])
    }
}

fn parse_bingo_games(input: &str) -> (Vec<i32>, Boards) {
    let mut lines = input.lines();
    let draws = lines
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let mut boards = Vec::new();
    boards.extend(LineIterator::new(
        lines.map(str::trim).filter(|line| !line.is_empty()),
    ));

    (draws, boards)
}

fn make_row(line: &str) -> Row {
    let mut cells = line
        .split_ascii_whitespace()
        .map(|num| (num.trim().parse::<i32>().unwrap(), false));
    [
        cells.next().unwrap(),
        cells.next().unwrap(),
        cells.next().unwrap(),
        cells.next().unwrap(),
        cells.next().unwrap(),
    ]
}

fn play(board: &mut Board, draw: i32) -> Option<i32> {
    let mut found = false;
    for row in board.iter_mut() {
        for cell in row {
            if cell.0 == draw {
                cell.1 = true;
                found = true;
            }
        }
    }
    if !found {
        return None;
    }
    check_bingo(board).then(|| {
        board.iter().fold(0, |score, row| {
            row.iter().fold(
                score,
                |score, cell| {
                    if cell.1 {
                        score
                    } else {
                        score + cell.0
                    }
                },
            )
        })
    })
}

pub fn part1() -> String {
    part1_inner(INPUT)
}

pub fn part1_inner(input: &str) -> String {
    let (draws, mut boards) = parse_bingo_games(input);
    for draw in draws {
        for board in &mut boards {
            if let Some(score) = play(board, draw) {
                return (draw * score).to_string();
            }
        }
    }
    panic!("no winner");
}

pub fn part2() -> String {
    part2_inner(INPUT)
}

pub fn part2_inner(input: &str) -> String {
    let (draws, mut boards) = parse_bingo_games(input);
    let mut final_score = None;
    for draw in draws {
        boards = boards
            .into_iter()
            .fold(Vec::new(), |mut boards, mut board| {
                if let Some(score) = play(&mut board, draw) {
                    final_score = Some((draw * score).to_string());
                } else {
                    boards.push(board);
                }
                boards
            });
        if boards.is_empty() {
            return final_score.unwrap();
        }
    }
    panic!("no winner");
}

#[test]
fn test_part1() {
    assert_eq!(
        "4512",
        part1_inner(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

            22 13 17 11  0
             8  2 23  4 24
            21  9 14 16  7
             6 10  3 18  5
             1 12 20 15 19
            
             3 15  0  2 22
             9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6
            
            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
             2  0 12  3  7"
        )
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        "1924",
        part2_inner(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

            22 13 17 11  0
             8  2 23  4 24
            21  9 14 16  7
             6 10  3 18  5
             1 12 20 15 19
            
             3 15  0  2 22
             9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6
            
            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
             2  0 12  3  7"
        )
    );
}
