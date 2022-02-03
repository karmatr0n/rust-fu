fn main() {
    let x = 6;
    if x > 5 {
        println!("condition was true!");
    } else {
        println!("condition was false!");
    }

    // Rust will not automatically try to convert non-Boolean types to a Boolean

    let number = 1;
    if number != 0 {
        println!("The value of number greater than zero: {}", number);
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}
