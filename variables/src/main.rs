fn main() {

    // Mutablility
    let mut x = 5;

    println!("The value for x is: {}", x);

    x = 6;

    println!("The value for x is: {}", x);

    // shadowing

    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value for y in the inner scope is: {}", y);
    }
    println!("The value for y is: {}", y);
}
