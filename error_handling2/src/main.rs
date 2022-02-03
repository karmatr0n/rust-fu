use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    
    let _f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
