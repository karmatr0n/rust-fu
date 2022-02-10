use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    let input = File::open(filename)?;
    let buffered = BufReader::new(input);

    let mut horizontal:i32 = 0;
    let mut depth:i32 = 0;

    for line in buffered.lines() {
        let line = line?;
        let words: Vec<&str> = line.split_whitespace().collect();
        let cmd = words[0];
        let value:i32 = words[1].parse().unwrap();
        if cmd == "forward" {
            horizontal += value;
        } else if cmd == "down" {
            depth += value;
        } else if cmd == "up" {
            depth -= value;
        }
    }

    println!("Horizontal: {}", horizontal);
    println!("Depth: {}", depth);
    println!("Answer: {}", horizontal * depth);

    Ok(())
}