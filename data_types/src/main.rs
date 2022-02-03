use std::io;

fn main() {
    // There are integer, float, bool, char (string), compound type (array and tuple)
    // let c = 'A';
    // let difference = 95.5 - 4.3;
    // let sum = 1 + 5;
    // let t = true;
    // let f:bool = false;
    let x = [1, 2, 3, 4, 5];
    println!("Please enter an array index");
    
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = x[index];

    println!("The value of the element at index {} is {}", index, element);
}
