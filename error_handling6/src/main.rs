// Propagating errors with the ? operator.
use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let _f = File::open("hello.txt")?;
    Ok(())
}
