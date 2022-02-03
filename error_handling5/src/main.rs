use std::fs::File;

fn main() {
    // let _f1 = File::open("hello.txt").unwrap();
    let _f2 = File::open("hello.txt").expect("Failed to open hello.txt");
}
