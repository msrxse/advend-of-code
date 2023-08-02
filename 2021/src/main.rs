use std::env;

mod day0;
mod day1;

pub use day0::{part1 as p0_1, part2 as p0_2};
pub use day1::{part1 as p1_1, part2 as p1_2};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "0-1" => println!("{}", p0_1()),
        "0-2" => println!("{}", p0_2()),
        "1-1" => println!("{}", p1_1()),
        "1-2" => println!("{}", p1_2()),
        _ => println!("Exercise not found"),
    }
}
