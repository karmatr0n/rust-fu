use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    let mut numbers: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(number) = line {
                let a: i32 = number.parse().unwrap();
                numbers.push(a);
                
            }
        }
    }
    let mut sum: i32 = 0;
    for (i, number) in numbers.iter().enumerate() {
        if i+1 < numbers.len() && &numbers[i+1] > number {
            sum += 1;
        }
    }
    println!("{}", sum);
}
