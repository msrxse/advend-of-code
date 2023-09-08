use std::env;

mod day0;
mod day2;
mod day3;
mod day4;
mod day5;

pub use day0::{part1 as p0_1, part2 as p0_2};
pub use day2::{part1 as p2_1, part2 as p2_2};
pub use day3::{part1 as p3_1, part2 as p3_2};
pub use day4::{part1 as p4_1, part2 as p4_2};
pub use day5::{part1 as p5_1, part2 as p5_2};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "0-1" => println!("{}", p0_1()),
        "0-2" => println!("{}", p0_2()),
        "2-1" => println!("{}", p2_1()),
        "2-2" => println!("{}", p2_2()),
        "3-1" => println!("{}", p3_1()),
        "3-2" => println!("{}", p3_2()),
        "4-1" => println!("{}", p4_1()),
        "4-2" => println!("{}", p4_2()),
        "5-1" => println!("{}", p5_1()),
        "5-2" => println!("{}", p5_2()),
        _ => println!("Exercise not found"),
    }
}
