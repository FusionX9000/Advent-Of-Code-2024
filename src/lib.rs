use std::fs;

pub fn read_input(day: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(format!("inputs/{day}.txt"))
}
