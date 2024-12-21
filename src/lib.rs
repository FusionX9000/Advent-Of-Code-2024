use std::{fmt::Debug, fs};

pub fn read_input(day: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(format!("inputs/{day}.txt"))
}

pub fn print_grid<T: Debug>(v: &Vec<Vec<T>>) {
    for r in v {
        println!("{:?}", r);
    }
}
